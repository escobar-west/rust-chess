use super::BitBoard;
use crate::board::Square;

pub struct BitBoardFwdIter {
    rem_mask: BitBoard,
}

impl BitBoardFwdIter {
    pub fn new(bitboard: BitBoard) -> Self {
        Self { rem_mask: bitboard }
    }
}

impl Iterator for BitBoardFwdIter {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        self.rem_mask.bitscan_forward().map(|lsb| {
            self.rem_mask ^= BitBoard::from(lsb);
            lsb
        })
    }
}
