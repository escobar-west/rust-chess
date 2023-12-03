use crate::{board::Square, pieces::Piece};
pub struct MailBox(Box<[Option<Piece>; 64]>);

impl MailBox {
    pub fn get_square(&self, square: Square) -> Option<Piece> {
        self.0[square.0 as usize]
    }

    pub fn clear_square(&mut self, square: Square) -> Option<Piece> {
        self.0[square.0 as usize].take()
    }

    pub fn set_square(&mut self, square: Square, piece: Piece) -> Option<Piece> {
        self.0[square.0 as usize].replace(piece)
    }
}

impl Default for MailBox {
    fn default() -> Self {
        MailBox(Box::new([None; 64]))
    }
}
