use super::BitBoard;
use crate::board::Square;

pub struct BitBoardFwdIter<'a> {
    rem_mask: BitBoard,
    _bit_ref: &'a BitBoard,
}

impl<'a> BitBoardFwdIter<'a> {
    pub fn new(bitboard: &'a BitBoard) -> Self {
        Self {
            rem_mask: *bitboard,
            _bit_ref: bitboard,
        }
    }
}

impl<'a> Iterator for BitBoardFwdIter<'a> {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        self.rem_mask.bitscan_forward().map(|lsb| {
            self.rem_mask ^= BitBoard::from(lsb);
            lsb
        })
    }
}
