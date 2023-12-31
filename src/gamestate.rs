mod castlerights;
mod moves;

use std::num::NonZeroU32;

use crate::{
    board::{BitBoard, Board, Column, Row, Square, EMPTY_BOARD, FULL_BOARD},
    pieces::{
        constants::{BLACK_KING, WHITE_KING},
        Color, Figure, Piece,
    },
};
use castlerights::CastleRights;
use moves::{Move, MoveRecord};

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

#[derive(Debug)]
pub struct MoveIter<'a> {
    from_square: Square,
    to_squares: BitBoard,
    piece: Piece,
    game: &'a GameState,
}

impl<'a> MoveIter<'a> {
    pub fn new(game: &'a GameState, square: Square) -> Self {
        Self {
            from_square: square,
            piece: game.board.get_square(square).unwrap(),
            to_squares: game.board.get_moves(square),
            game,
        }
    }
}

impl<'a> Iterator for MoveIter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        if self.game.turn != self.piece.color {
            return None;
        }
        match self.piece.figure {
            Figure::King => self.to_squares.next().map(|to| Move::MoveKing {
                from: self.from_square,
                to,
            }),
            _ => self.to_squares.next().map(|to| Move::MovePiece {
                from: self.from_square,
                to,
            }),
        }
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

    fn check_move_legality(&self, move_: Move) -> bool {
        match move_ {
            Move::MoveKing { from, to } => self.check_king_move_legality(from, to),
            Move::MovePiece { from, to } => self.check_move_piece_legality(from, to),
        }
    }

    fn check_king_move_legality(&self, from: Square, to: Square) -> bool {
        let to = to.as_bitboard();
        let safe_mask = self.board.get_safe_squares(from, self.turn);
        to & safe_mask == to
    }

    fn check_move_piece_legality(&self, from: Square, to: Square) -> bool {
        let to = to.as_bitboard();
        let king_square = self.get_king_sq(self.turn);
        let pin_mask = self.board.get_pin_mask(from, king_square, self.turn);
        let stop_check_mask = self.board.get_check_stops(king_square, self.turn);
        to & pin_mask & stop_check_mask == to
    }

    fn make_legal_move(&mut self, move_: Move) {
        let castle_rights = self.castle;
        let half_moves = self.half_moves;
        let captured = match move_ {
            Move::MovePiece { from, to } => self.move_piece(from, to),
            Move::MoveKing { from, to } => self.move_king(from, to),
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
            Move::MoveKing { from, to } => self.unmove_king(from, to, prev_move.captured),
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
            self.board.set_square(to, piece);
        }
    }

    fn unmove_king(&mut self, from: Square, to: Square, captured: Option<Piece>) {
        self.board.move_piece(to, from);
        if let Some(piece) = captured {
            self.board.set_square(to, piece);
        }
        match self.turn {
            Color::White => self.black_king = from,
            Color::Black => self.white_king = from,
        }
    }

    fn move_king(&mut self, from: Square, to: Square) -> Option<Piece> {
        self.castle.remove_castle_rights(self.turn);
        let (queen_rook, king_rook) = match self.turn {
            Color::White => (A8, H8),
            Color::Black => (A1, H1),
        };
        match to {
            f if f == queen_rook => self.castle.remove_queenside_castle_rights(!self.turn),
            f if f == king_rook => self.castle.remove_kingside_castle_rights(!self.turn),
            _ => (),
        }
        match self.turn {
            Color::White => self.white_king = to,
            Color::Black => self.black_king = to,
        }
        let captured = self.board.move_piece(from, to);
        if captured.is_some() {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }
        captured
    }

    fn move_piece(&mut self, from: Square, to: Square) -> Option<Piece> {
        let (queen_rook, king_rook) = match self.turn {
            Color::White => (A1, H1),
            Color::Black => (A8, H8),
        };
        match from {
            f if f == queen_rook => self.castle.remove_queenside_castle_rights(self.turn),
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
                .get_square(to)
                .map_or(false, |p| p.figure == Figure::Pawn)
        {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }
        captured
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
        let white_king = board.get_pieces(WHITE_KING).iter_forward().next().unwrap();
        let black_king = board.get_pieces(BLACK_KING).iter_forward().next().unwrap();

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

    pub fn perft(&mut self, depth: u32) -> u128 {
        fn perft_(game: &mut GameState, depth: u32) -> u128 {
            //self.validate_position();
            let mut perft = 0;
            let move_list: Vec<Move> = game
                .board
                .get_color(game.turn)
                .flat_map(|square| MoveIter::new(game, square))
                .filter(|m| game.check_move_legality(*m))
                .collect();
            if depth == 1 {
                return move_list.len() as u128;
            }
            for move_ in move_list {
                game.make_legal_move(move_);
                perft += game.perft(depth - 1);
                game.unmake_move();
            }
            perft
        }
        if depth == 0 {
            return 1;
        }
        perft_(self, depth)
    }

    fn validate_position(&self) {
        let white_king = self.board.get_pieces(WHITE_KING).iter_forward().next();
        assert_eq!(white_king, Some(self.white_king));
        let black_king = self.board.get_pieces(BLACK_KING).iter_forward().next();
        assert_eq!(black_king, Some(self.black_king));
        let opponent_king_square = self.get_king_sq(!self.turn);
        let opponent_is_in_check = self.board.is_attacked_by(opponent_king_square, self.turn);
        if opponent_is_in_check {
            self.board.print_board();
            println!(
                "Turn: {}::{:#?}, Moves: {:#?}",
                self.full_moves, self.turn, self.move_list
            );
            panic!();
        }
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
}
