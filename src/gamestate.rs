mod moves;
use std::io::Empty;

use crate::{
    board::{BitBoard, Board, Column, Row, Square, EMPTY_BOARD},
    pieces::{
        constants::{BLACK_KING, WHITE_KING},
        Color, Figure, Piece,
    },
};
use moves::Move;

const A1: Square = Square::from_coords(Row::new(0), Column::new(0));
const C1: Square = Square::from_coords(Row::new(0), Column::new(2));
const D1: Square = Square::from_coords(Row::new(0), Column::new(3));
const E1: Square = Square::from_coords(Row::new(0), Column::new(4));
const F1: Square = Square::from_coords(Row::new(0), Column::new(5));
const G1: Square = Square::from_coords(Row::new(0), Column::new(6));
const H1: Square = Square::from_coords(Row::new(0), Column::new(7));
const A8: Square = Square::from_coords(Row::new(7), Column::new(0));
const C8: Square = Square::from_coords(Row::new(7), Column::new(2));
const D8: Square = Square::from_coords(Row::new(7), Column::new(3));
const E8: Square = Square::from_coords(Row::new(7), Column::new(4));
const F8: Square = Square::from_coords(Row::new(7), Column::new(5));
const G8: Square = Square::from_coords(Row::new(7), Column::new(6));
const H8: Square = Square::from_coords(Row::new(7), Column::new(7));

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CastleRights(u8);

impl CastleRights {
    pub fn new(wk: bool, wq: bool, bk: bool, bq: bool) -> Self {
        Self(u8::from(wk) + 2 * u8::from(wq) + 4 * u8::from(bk) + 8 * u8::from(bq))
    }

    pub fn remove_castle_rights(&mut self, color: Color) {
        match color {
            Color::White => self.0 &= 0b1100,
            Color::Black => self.0 &= 0b0011,
        }
    }

    pub fn remove_kingside_castle_rights(&mut self, color: Color) {
        match color {
            Color::White => self.0 &= 0b1110,
            Color::Black => self.0 &= 0b1011,
        }
    }

    pub fn remove_queenside_castle_rights(&mut self, color: Color) {
        match color {
            Color::White => self.0 &= 0b1101,
            Color::Black => self.0 &= 0b0111,
        }
    }

    fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
        let mut wk = false;
        let mut wq = false;
        let mut bk = false;
        let mut bq = false;
        for c in fen.chars() {
            match c {
                '-' => return Ok(Self(0)),
                'K' => wk = true,
                'Q' => wq = true,
                'k' => bk = true,
                'q' => bq = true,
                _ => return Err("invalid char"),
            }
        }
        Ok(Self::new(wk, wq, bk, bq))
    }

    fn to_fen(self) -> String {
        let fen = match self.0 {
            0b0000 => "-",
            0b0001 => "K",
            0b0010 => "Q",
            0b0011 => "KQ",
            0b0100 => "k",
            0b0101 => "Kk",
            0b0110 => "Qk",
            0b0111 => "KQk",
            0b1000 => "q",
            0b1001 => "Kq",
            0b1010 => "Qq",
            0b1011 => "KQq",
            0b1100 => "kq",
            0b1101 => "Kkq",
            0b1110 => "Qkq",
            0b1111 => "KQkq",
            _ => panic!(),
        };
        fen.into()
    }
}

#[derive(Debug)]
pub struct MoveRecord {
    move_: Move,
    captured: Option<Piece>,
    castle_rights: CastleRights,
    half_move: u16,
}

impl MoveRecord {
    fn new(
        move_: Move,
        captured: Option<Piece>,
        castle_rights: CastleRights,
        half_move: u16,
    ) -> Self {
        Self {
            move_,
            captured,
            castle_rights,
            half_move,
        }
    }
}

#[derive(Debug)]
pub struct MoveIter<'a> {
    from_square: Square,
    to_squares: BitBoard,
    game: &'a GameState,
}

impl<'a> MoveIter<'a> {
    pub fn new(game: &'a GameState, square: Square) -> Self {
        Self {
            from_square: square,
            to_squares: game.board.get_moves_from_sq(square),
            game,
        }
    }
}

impl<'a> Iterator for MoveIter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        self.to_squares.next().map(|to| Move::MovePiece {
            from: self.from_square,
            to,
        })
    }
}

#[derive(Debug)]
pub struct GameState {
    board: Board,
    turn: Color,
    castle: CastleRights,
    ep: Option<Square>,
    half_moves: u16,
    full_moves: u16,
    white_king: Square,
    black_king: Square,
    move_list: Vec<MoveRecord>,
}

impl GameState {
    pub fn get_king_sq(&self, color: Color) -> Square {
        match color {
            Color::White => self.white_king,
            Color::Black => self.black_king,
        }
    }

    fn make_legal_move(&mut self, move_: Move) {
        let castle_rights = self.castle;
        let half_moves = self.half_moves;
        let captured = match move_ {
            Move::MovePiece { from, to } => self.move_piece(from, to),
            Move::PromotePawn { from, to, piece } => self.promote_pawn(from, to, piece),
            Move::EnPassant { from, to, ep } => self.move_enpassant(from, to, ep),
            Move::KingsideCastle => {
                self.move_kingside_castle();
                None
            }
            Move::QueensideCastle => {
                self.move_queenside_castle();
                None
            }
        };
        let record = MoveRecord::new(move_, captured, castle_rights, half_moves);
        self.move_list.push(record);
        if self.turn == Color::Black {
            self.full_moves += 1;
        }
        self.turn = !self.turn;
    }

    fn unmake_move(&mut self) {
        let Some(prev_move) = self.move_list.pop() else {
            return;
        };
        match prev_move.move_ {
            Move::MovePiece { from, to } => self.unmove_piece(from, to, prev_move.captured),
            Move::PromotePawn { from, to, piece } => {
                self.unpromote_pawn(from, to, piece, prev_move.captured)
            }
            Move::EnPassant { from, to, ep } => self.unmake_enpassant(from, to, ep),
            Move::KingsideCastle => self.unmake_kingside_castle(),
            Move::QueensideCastle => self.unmake_queenside_castle(),
        }
        self.castle = prev_move.castle_rights;
        self.half_moves = prev_move.half_move;
        if self.turn == Color::White {
            self.full_moves -= 1;
        }
        self.turn = !self.turn;
    }

    fn unmove_piece(&mut self, from: Square, to: Square, captured: Option<Piece>) {
        self.board.move_piece(to, from);
        if let Some(piece) = captured {
            self.board.set_sq(to, piece);
        }
    }

    fn unpromote_pawn(
        &mut self,
        from: Square,
        to: Square,
        promotion: Piece,
        captured: Option<Piece>,
    ) {
        let pawn = Piece {
            color: promotion.color,
            figure: Figure::Pawn,
        };
        self.board.set_sq(from, pawn);
        match captured {
            Some(captured) => self.board.set_sq(to, captured),
            None => self.board.clear_sq(to),
        };
    }

    fn unmake_enpassant(&mut self, from: Square, to: Square, ep: Square) {
        self.board.move_piece(to, from);
        let pawn = Piece {
            color: self.turn,
            figure: Figure::Pawn,
        };
        self.board.set_sq(ep, pawn);
        self.ep = Some(ep);
    }

    fn unmake_kingside_castle(&mut self) {
        let (king_sq, new_rook_sq, new_king_sq, rook_sq) = match self.turn {
            Color::White => (E1, F1, G1, H1),
            Color::Black => (E8, F8, G8, H8),
        };
        self.board.move_piece(new_king_sq, king_sq);
        self.board.move_piece(new_rook_sq, rook_sq);
    }

    fn unmake_queenside_castle(&mut self) {
        let (rook_sq, new_king_sq, new_rook_sq, king_sq) = match self.turn {
            Color::White => (A1, C1, D1, E1),
            Color::Black => (A8, C8, D8, E8),
        };
        self.board.move_piece(new_king_sq, king_sq);
        self.board.move_piece(new_rook_sq, rook_sq);
    }

    fn move_piece(&mut self, from: Square, to: Square) -> Option<Piece> {
        let (queen_rook, king, king_rook) = match self.turn {
            Color::White => (A1, E1, H1),
            Color::Black => (A8, E8, H8),
        };
        match from {
            f if f == queen_rook => self.castle.remove_queenside_castle_rights(self.turn),
            f if f == king => self.castle.remove_castle_rights(self.turn),
            f if f == king_rook => self.castle.remove_kingside_castle_rights(self.turn),
            _ => (),
        }
        let (queen_rook, king_rook) = match self.turn {
            Color::White => (A8, H8),
            Color::Black => (A1, H1),
        };
        match to {
            f if f == queen_rook => self.castle.remove_queenside_castle_rights(!self.turn),
            f if f == king_rook => self.castle.remove_kingside_castle_rights(!self.turn),
            _ => (),
        }
        let captured = self.board.move_piece(from, to);
        if captured.is_some()
            || self
                .board
                .get_sq(to)
                .map_or(false, |p| p.figure == Figure::Pawn)
        {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }
        captured
    }

    fn promote_pawn(&mut self, from: Square, to: Square, promotion: Piece) -> Option<Piece> {
        let (queen_rook, king_rook) = match self.turn {
            Color::White => (A8, H8),
            Color::Black => (A1, H1),
        };
        match to {
            f if f == queen_rook => self.castle.remove_queenside_castle_rights(!self.turn),
            f if f == king_rook => self.castle.remove_kingside_castle_rights(!self.turn),
            _ => (),
        }
        self.half_moves = 0;
        self.board.clear_sq(from);
        self.board.set_sq(to, promotion)
    }

    fn move_enpassant(&mut self, from: Square, to: Square, ep: Square) -> Option<Piece> {
        self.half_moves = 0;
        self.move_piece(from, to);
        self.board.clear_sq(ep)
    }

    fn move_kingside_castle(&mut self) {
        self.half_moves += 1;
        self.castle.remove_castle_rights(self.turn);
        let (king_sq, new_rook_sq, new_king_sq, rook_sq) = match self.turn {
            Color::White => (E1, F1, G1, H1),
            Color::Black => (E8, F8, G8, H8),
        };
        self.board.move_piece(king_sq, new_king_sq);
        self.board.move_piece(rook_sq, new_rook_sq);
    }

    fn move_queenside_castle(&mut self) {
        self.half_moves += 1;
        self.castle.remove_castle_rights(self.turn);
        let (rook_sq, new_king_sq, new_rook_sq, king_sq) = match self.turn {
            Color::White => (A1, C1, D1, E1),
            Color::Black => (A8, C8, D8, E8),
        };
        self.board.move_piece(king_sq, new_king_sq);
        self.board.move_piece(rook_sq, new_rook_sq);
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
        let mut fen_iter = fen.split(' ');

        let position_fen = fen_iter.next().ok_or("Empty Fen")?;
        let board = Board::try_from_fen(position_fen)?;

        let turn = match fen_iter.next() {
            Some("w") => Color::White,
            Some("b") => Color::Black,
            _ => return Err("Invalid Fen"),
        };

        let castle_fen = fen_iter.next().ok_or("Empty Fen")?;
        let castle = CastleRights::try_from_fen(castle_fen)?;

        let ep: Option<Square> = match fen_iter.next() {
            Some("-") => None,
            Some(coords) => Some(Square::try_from_alg(coords)?),
            None => return Err("Invalid Fen"),
        };
        let half_moves = fen_iter.next().map(|x| x.parse::<u16>()).unwrap().unwrap();
        let full_moves = fen_iter.next().map(|x| x.parse::<u16>()).unwrap().unwrap();
        let white_king = board
            .get_piece_mask(WHITE_KING)
            .iter_forward()
            .next()
            .unwrap();
        let black_king = board
            .get_piece_mask(BLACK_KING)
            .iter_forward()
            .next()
            .unwrap();

        Ok(Self {
            board,
            turn,
            castle,
            ep,
            half_moves,
            full_moves,
            white_king,
            black_king,
            move_list: Vec::with_capacity(20),
        })
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(25);

        // board
        fen.push_str(&self.board.to_fen());
        fen.push(' ');

        // turn
        let turn_char = match self.turn {
            Color::White => 'w',
            Color::Black => 'b',
        };
        fen.push(turn_char);
        fen.push(' ');

        // castle rights
        fen.push_str(&self.castle.to_fen());
        fen.push(' ');

        // en passant
        match self.ep {
            Some(s) => fen.push_str(&s.to_alg()),
            None => fen.push('-'),
        }
        fen.push(' ');

        // halfmove
        fen.push_str(&self.half_moves.to_string());
        fen.push(' ');

        // fullmove
        fen.push_str(&self.full_moves.to_string());

        fen
    }

    fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut perft = 0;
        let move_list: Vec<Move> = self
            .board
            .get_color_mask(self.turn)
            .flat_map(|square| MoveIter::new(self, square))
            .collect();
        for move_ in move_list {
            self.make_legal_move(move_);
            perft += self.perft(depth - 1);
            self.unmake_move();
        }
        perft
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::try_from_fen(DEFAULT_FEN).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen() {
        let gs = GameState::default();
        assert_eq!(gs.board, Board::try_from_fen(DEFAULT_FEN).unwrap());
        assert_eq!(gs.turn, Color::White);
        assert_eq!(gs.castle, CastleRights::new(true, true, true, true));
        assert_eq!(gs.ep, None);
        assert_eq!(gs.half_moves, 0);
        assert_eq!(gs.full_moves, 1);
    }

    #[test]
    fn test_perft_3() {
        let mut gs = GameState::default();
        let perft = gs.perft(3);
        assert_eq!(perft, 8902);
    }
}
