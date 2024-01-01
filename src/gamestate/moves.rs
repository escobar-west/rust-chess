use super::CastleRights;
use super::GameState;
use crate::{
    board::{Column, Row, Square},
    pieces::{Color, Figure, Piece},
};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    MovePiece {
        from: Square,
        to: Square,
    },
    MoveKing {
        from: Square,
        to: Square,
    },
    MovePawn {
        from: Square,
        to: Square,
    },
    PromotePawn {
        from: Square,
        to: Square,
        promotion: Piece,
    },
    MovePawnDouble {
        from: Square,
        to: Square,
    },
    EnPassant {
        from: Square,
        to: Square,
        ep: Option<Square>,
    },
}

impl Move {
    pub fn check_move_legality(self, game: &GameState) -> bool {
        use check_move::*;
        match self {
            Move::MovePiece { from, to } => check_move_piece_legality(game, from, to),
            Move::MoveKing { from, to } => check_move_king_legality(game, from, to),
            _ => unimplemented!(),
        }
    }

    pub fn make_move(self, game: &mut GameState) {
        use make_move::*;
        let castle_rights = game.castle;
        let half_moves = game.half_moves;
        let captured = match self {
            Move::MovePiece { from, to } => move_piece(game, from, to),
            Move::MoveKing { from, to } => move_king(game, from, to),
            _ => unimplemented!(),
        };
        let record = MoveRecord::new(self, captured, castle_rights, half_moves);
        game.move_list.push(record);
        if game.turn == Color::Black {
            game.full_moves += 1;
        }
        game.turn = !game.turn;
    }

    pub fn unmake_move(self, game: &mut GameState, captured: Option<Piece>) {
        use unmake_move::*;
        match self {
            Move::MovePiece { from, to } => unmove_piece(game, from, to, captured),
            Move::MoveKing { from, to } => unmove_king(game, from, to, captured),
            _ => unimplemented!(),
        }
    }
}

mod check_move {
    use super::*;

    pub fn check_move_piece_legality(game: &GameState, from: Square, to: Square) -> bool {
        let to = to.as_bitboard();
        let king_square = game.get_king_sq(game.turn);
        let pin_mask = game.board.get_pin_mask(from, king_square, game.turn);
        let stop_check_mask = game.board.get_check_stops(king_square, game.turn);
        to & pin_mask & stop_check_mask == to
    }

    pub fn check_move_king_legality(game: &GameState, from: Square, to: Square) -> bool {
        let to = to.as_bitboard();
        let safe_mask = game.board.get_safe_squares(from, game.turn);
        to & safe_mask == to
    }
}

mod make_move {
    use super::*;

    pub fn move_piece(game: &mut GameState, from: Square, to: Square) -> Option<Piece> {
        let (queen_rook, king_rook) = match game.turn {
            Color::White => (A1, H1),
            Color::Black => (A8, H8),
        };
        match from {
            f if f == queen_rook => game.castle.remove_queenside_castle_rights(game.turn),
            f if f == king_rook => game.castle.remove_kingside_castle_rights(game.turn),
            _ => (),
        }
        let (queen_rook, king_rook) = match game.turn {
            Color::White => (A8, H8),
            Color::Black => (A1, H1),
        };
        match to {
            f if f == queen_rook => game.castle.remove_queenside_castle_rights(!game.turn),
            f if f == king_rook => game.castle.remove_kingside_castle_rights(!game.turn),
            _ => (),
        }
        let captured = game.board.move_piece(from, to);
        if captured.is_some()
            || game
                .board
                .get_square(to)
                .map_or(false, |p| p.figure == Figure::Pawn)
        {
            game.half_moves = 0;
        } else {
            game.half_moves += 1;
        }
        captured
    }

    pub fn move_king(game: &mut GameState, from: Square, to: Square) -> Option<Piece> {
        game.castle.remove_castle_rights(game.turn);
        let (queen_rook, king_rook) = match game.turn {
            Color::White => (A8, H8),
            Color::Black => (A1, H1),
        };
        match to {
            f if f == queen_rook => game.castle.remove_queenside_castle_rights(!game.turn),
            f if f == king_rook => game.castle.remove_kingside_castle_rights(!game.turn),
            _ => (),
        }
        match game.turn {
            Color::White => game.white_king = to,
            Color::Black => game.black_king = to,
        }
        let captured = game.board.move_piece(from, to);
        if captured.is_some() {
            game.half_moves = 0;
        } else {
            game.half_moves += 1;
        }
        captured
    }
}

mod unmake_move {
    use super::*;
    pub fn unmove_piece(game: &mut GameState, from: Square, to: Square, captured: Option<Piece>) {
        game.board.move_piece(to, from);
        if let Some(captured) = captured {
            game.board.set_square(to, captured);
        }
    }

    pub fn unmove_king(game: &mut GameState, from: Square, to: Square, captured: Option<Piece>) {
        game.board.move_piece(to, from);
        match game.turn {
            Color::White => game.black_king = from,
            Color::Black => game.white_king = from,
        }
        if let Some(captured) = captured {
            game.board.set_square(to, captured);
        }
    }
}

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
