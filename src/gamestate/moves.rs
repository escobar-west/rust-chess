use super::CastleRights;
use super::GameState;
use crate::gamestate::moves::make_move::move_pawn_double;
use crate::{
    board::{BitBoard, Column, Row, Square},
    pieces::{Color, Figure, Piece},
};

const A1: Square = Square::from_coords(Row::new(0), Column::new(0));
const C1: Square = Square::from_coords(Row::new(0), Column::new(2));
const D1: Square = Square::from_coords(Row::new(0), Column::new(3));
const E1: Square = Square::from_coords(Row::new(0), Column::new(4));
const F1: Square = Square::from_coords(Row::new(0), Column::new(5));
const G1: Square = Square::from_coords(Row::new(0), Column::new(6));
const H1: Square = Square::from_coords(Row::new(0), Column::new(7));
const A2: Square = Square::from_coords(Row::new(1), Column::new(0));
const A3: Square = Square::from_coords(Row::new(2), Column::new(0));
const A6: Square = Square::from_coords(Row::new(5), Column::new(0));
const A7: Square = Square::from_coords(Row::new(6), Column::new(0));
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
        ep: Square,
    },
    KingSideCastle,
    QueenSideCastle,
}

impl Move {
    pub fn _is_legal(self, game: &GameState) -> bool {
        use check_move::*;
        match self {
            Move::MovePiece { from, to } => check_move_piece_legality(game, from, to),
            Move::MoveKing { from, to } => check_move_king_legality(game, from, to),
            Move::MovePawnDouble { from, to } => todo!(),
            _ => unimplemented!(),
        }
    }

    pub fn _make_move(self, game: &mut GameState) -> Option<Piece> {
        use make_move::*;
        match self {
            Move::MovePiece { from, to } => move_piece(game, from, to),
            Move::MoveKing { from, to } => move_king(game, from, to),
            Move::MovePawnDouble { from, to } => {
                move_pawn_double(game, from, to);
                None
            }
            _ => unimplemented!(),
        }
    }

    pub fn _unmake_move(self, game: &mut GameState, captured: Option<Piece>) {
        use unmake_move::*;
        match self {
            Move::MovePiece { from, to } => unmove_piece(game, from, to, captured),
            Move::MoveKing { from, to } => unmove_king(game, from, to, captured),
            Move::MovePawnDouble { from, to } => todo!(),
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
        game.ep = None;
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
        game.ep = None;
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

    pub fn move_pawn_double(game: &mut GameState, from: Square, to: Square) {
        game.ep = Some(to);
        game.half_moves = 0;
        game.board.move_piece(from, to);
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
pub enum MoveIterator<'a> {
    PawnMoveIterator {
        from: Square,
        to: BitBoard,
        game: &'a GameState,
        check_double_flag: bool,
        ep_square: Option<Square>,
    },
    PawnPromotionIterator {
        from: Square,
        to: BitBoard,
        game: &'a GameState,
        next_promotion: Option<Figure>,
    },
    KingMoveIterator {
        from: Square,
        to: BitBoard,
        game: &'a GameState,
    },
    PieceMoveIterator {
        from: Square,
        to: BitBoard,
        game: &'a GameState,
    },
    EmptyIterator,
}

impl<'a> MoveIterator<'a> {
    pub fn new(game: &'a GameState, from: Square) -> Self {
        let Some(piece) = game.board.get_square(from) else {
            return Self::EmptyIterator;
        };
        if piece.color != game.turn {
            return Self::EmptyIterator;
        }
        match piece.figure {
            Figure::Pawn => Self::PawnMoveIterator {
                from,
                to: game.board.get_moves(from),
                game,
                check_double_flag: true,
                ep_square: game.ep,
            },
            Figure::King => Self::KingMoveIterator {
                from,
                to: game.board.get_moves(from),
                game,
            },
            _ => Self::PieceMoveIterator {
                from,
                to: game.board.get_moves(from),
                game,
            },
        }
    }
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::PawnMoveIterator {
                from,
                to,
                game,
                check_double_flag,
                ep_square,
            } => {
                if std::mem::take(check_double_flag) {
                    let double_move = get_double_pawn_move(*from, game);
                    if double_move.is_some() {
                        return double_move;
                    }
                }
                if let Some(ep_square) = ep_square.take().filter(|ep| {
                    from.get_row() == ep.get_row() && from.get_col().is_adjacent(ep.get_col())
                }) {
                    let col = ep_square.get_col();
                    let row = match game.turn {
                        Color::White => ep_square.get_row() + 1,
                        Color::Black => ep_square.get_row() - 1,
                    };
                    let to = Square::from_coords(row, col);
                    return Some(Move::EnPassant {
                        from: *from,
                        to,
                        ep: ep_square,
                    });
                }
                to.next().map(|to| Move::MovePiece { from: *from, to })
            }
            Self::PieceMoveIterator { from, to, game: _ } => {
                to.next().map(|to| Move::MovePiece { from: *from, to })
            }
            Self::KingMoveIterator { from, to, game: _ } => {
                to.next().map(|to| Move::MoveKing { from: *from, to })
            }
            Self::EmptyIterator => None,
            _ => None,
        }
    }
}

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

fn get_double_pawn_move(from: Square, game: &GameState) -> Option<Move> {
    if from.get_row().as_u8()
        != match game.turn {
            Color::White => 1,
            Color::Black => 6,
        }
    {
        return None;
    }
    let double_mask: BitBoard = BitBoard::from(A2) | BitBoard::from(A3);
    let block_mask = game.board.get_occupied()
        & match game.turn {
            Color::White => double_mask << from.get_col().as_u8(),
            Color::Black => double_mask << 16 + from.get_col().as_u8(),
        };
    if block_mask.is_not_empty() {
        return None;
    }
    let row = match game.turn {
        Color::White => Row::new(3),
        Color::Black => Row::new(4),
    };
    Some(Move::MovePawnDouble {
        from,
        to: Square::from_coords(row, from.get_col()),
    })
}
