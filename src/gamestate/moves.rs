use crate::{board::Square, pieces::Piece};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    KingsideCastle,
    QueensideCastle,
    MovePiece {
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
