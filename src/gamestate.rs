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
use moves::{Move, MoveIter};

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
pub struct MoveRecord {
    pub move_: Move,
    pub captured: Option<Piece>,
    pub castle_rights: CastleRights,
    pub half_move: u16,
}

impl MoveRecord {
    pub fn new(
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

    pub fn is_legal(&self, move_: Move) -> bool {
        move_._is_legal(self)
    }

    pub fn make_move(&mut self, move_: Move) {
        let castle_rights = self.castle;
        let half_moves = self.half_moves;
        let captured = move_._make_move(self);
        let record = MoveRecord::new(move_, captured, castle_rights, half_moves);
        self.move_list.push(record);
        if self.turn == Color::Black {
            self.full_moves += 1;
        }
        self.turn = !self.turn;
    }

    pub fn pop_move(&mut self) {
        let Some(prev_move) = self.move_list.pop() else {
            return;
        };
        prev_move.move_._unmake_move(self, prev_move.captured);
        self.castle = prev_move.castle_rights;
        self.half_moves = prev_move.half_move;
        if self.turn == Color::White {
            self.full_moves -= 1;
        }
        self.turn = !self.turn;
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

    pub fn to_fen(&self) -> Box<str> {
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

        fen.into_boxed_str()
    }

    pub fn perft(&mut self, depth: u32) -> u128 {
        fn perft_(game: &mut GameState, depth: u32) -> u128 {
            let mut perft = 0;
            let move_list: Vec<Move> = game
                .board
                .get_color(game.turn)
                .flat_map(|square| MoveIter::new(game, square))
                .filter(|m| game.is_legal(*m))
                .collect();
            if depth == 1 {
                return move_list.len() as u128;
            }
            for move_ in move_list {
                game.make_move(move_);
                perft += game.perft(depth - 1);
                game.pop_move();
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
