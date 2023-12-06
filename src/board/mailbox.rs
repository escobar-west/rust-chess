use super::Square;
use crate::pieces::Piece;

#[derive(Debug, PartialEq, Eq)]
pub struct MailBox(Box<[Option<Piece>; 64]>);

impl MailBox {
    pub fn get_piece_at_square(&self, square: Square) -> Option<Piece> {
        self.0[usize::from(square)]
    }

    pub fn clear_square(&mut self, square: Square) -> Option<Piece> {
        self.0[usize::from(square)].take()
    }

    pub fn set_piece_at_square(&mut self, square: Square, piece: Piece) -> Option<Piece> {
        self.0[usize::from(square)].replace(piece)
    }
}

impl Default for MailBox {
    fn default() -> Self {
        MailBox(Box::new([None; 64]))
    }
}
