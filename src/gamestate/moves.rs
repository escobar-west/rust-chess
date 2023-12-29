use super::CastleRights;
use crate::{board::Square, pieces::Piece};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    KingsideCastle,
    QueensideCastle,
    MoveKing {
        from: Square,
        to: Square,
    },
    MovePiece {
        from: Square,
        to: Square,
    },
    MovePawnTwice {
        from: Square,
        to: Square,
    },
    PromotePawn {
        from: Square,
        to: Square,
        piece: Piece,
    },
    EnPassant {
        from: Square,
        to: Square,
        ep: Square,
    },
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
