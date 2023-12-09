mod constants;
mod iters;
use super::Square;
pub use constants::*;
use iters::BitBoardFwdIter;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum Direction {
    East = 0,
    North,
    West,
    South,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn print_board(&self) {
        let mut char_board: [char; 64] = ['.'; 64];
        for square in self.iter_forward() {
            char_board[usize::from(square)] = 'X';
        }
        let mut out_str = String::new();
        for i in (0..8).rev() {
            let offset = 8 * i as usize;
            let row: String = char_board[offset..offset + 8].iter().collect();
            out_str.push_str(&row);
            out_str.push('\n')
        }
        println!("{}", out_str);
    }

    pub fn iter_forward(&self) -> BitBoardFwdIter<'_> {
        BitBoardFwdIter::new(self)
    }

    pub fn bitscan_forward(&self) -> Option<Square> {
        let self_inner: u64 = self.0;
        if self_inner == 0 {
            return None;
        }
        let lookup_idx = DEBRUIJN64.wrapping_mul(self_inner & self_inner.wrapping_neg()) >> 58;
        Some(Square::new(FWDSCAN[lookup_idx as usize]))
    }

    pub fn bitscan_backward(&self) -> Option<Square> {
        let mut self_inner: u64 = self.0;
        if self_inner == 0 {
            return None;
        }
        self_inner |= self_inner >> 1;
        self_inner |= self_inner >> 2;
        self_inner |= self_inner >> 4;
        self_inner |= self_inner >> 8;
        self_inner |= self_inner >> 16;
        self_inner |= self_inner >> 32;
        let lookup_idx = self_inner.wrapping_mul(DEBRUIJN64) >> 58;
        Some(Square::new(BACKSCAN[lookup_idx as usize]))
    }

    const fn gen_square_mask(square: u64) -> Self {
        Self(1 << square)
    }

    const fn gen_row_mask(row: u8) -> Self {
        Self(0xff << 8 * row)
    }

    const fn gen_col_mask(col: u8) -> Self {
        Self(0x0101010101010101 << col)
    }

    const fn gen_east_mask(self) -> Self {
        let mut mask = self.0;
        mask |= NOT_A_FILE & (mask << 1);
        mask |= NOT_AB_FILE & (mask << 2);
        mask |= NOT_ABCD_FILE & (mask << 4);
        Self(mask ^ self.0)
    }

    const fn gen_east_north_mask(self) -> Self {
        const pr0: u64 = NOT_A_FILE;
        const pr1: u64 = pr0 & (pr0 << 9);
        const pr2: u64 = pr1 & (pr1 << 18);
        let mut mask = self.0;
        mask |= pr0 & (mask << 9);
        mask |= pr1 & (mask << 18);
        mask |= pr2 & (mask << 36);
        Self(mask ^ self.0)
    }

    const fn gen_north_mask(self) -> Self {
        let mut mask = self.0;
        mask |= mask << 8;
        mask |= mask << 16;
        mask |= mask << 32;
        Self(mask ^ self.0)
    }

    const fn gen_north_west_mask(self) -> Self {
        const pr0: u64 = NOT_A_FILE;
        const pr1: u64 = pr0 & (pr0 << 7);
        const pr2: u64 = pr1 & (pr1 << 14);
        let mut mask = self.0;
        mask |= pr0 & (mask << 7);
        mask |= pr1 & (mask << 14);
        mask |= pr2 & (mask << 28);
        Self(mask ^ self.0)
    }

    const fn gen_west_mask(self) -> Self {
        let mut mask = self.0;
        mask |= NOT_H_FILE & (mask >> 1);
        mask |= NOT_GH_FILE & (mask >> 2);
        mask |= NOT_EFGH_FILE & (mask >> 4);
        Self(mask ^ self.0)
    }

    const fn gen_west_south_mask(self) -> Self {
        const pr0: u64 = NOT_A_FILE;
        const pr1: u64 = pr0 & (pr0 >> 9);
        const pr2: u64 = pr1 & (pr1 >> 18);
        let mut mask = self.0;
        mask |= pr0 & (mask >> 9);
        mask |= pr1 & (mask >> 18);
        mask |= pr2 & (mask >> 36);
        Self(mask ^ self.0)
    }

    const fn gen_south_mask(self) -> Self {
        let mut mask = self.0;
        mask |= mask >> 8;
        mask |= mask >> 16;
        mask |= mask >> 32;
        Self(mask ^ self.0)
    }

    const fn gen_south_east_mask(self) -> Self {
        const pr0: u64 = NOT_A_FILE;
        const pr1: u64 = pr0 & (pr0 >> 7);
        const pr2: u64 = pr1 & (pr1 >> 14);
        let mut mask = self.0;
        mask |= pr0 & (mask >> 7);
        mask |= pr1 & (mask >> 14);
        mask |= pr2 & (mask >> 28);
        Self(mask ^ self.0)
    }

    const fn gen_knight_mask(self) -> Self {
        let self_inner: u64 = self.0;
        Self(
            ((self_inner << 17) & NOT_A_FILE)
                | ((self_inner << 10) & NOT_AB_FILE)
                | ((self_inner >> 6) & NOT_AB_FILE)
                | ((self_inner >> 15) & NOT_A_FILE)
                | ((self_inner << 15) & NOT_H_FILE)
                | ((self_inner << 6) & NOT_GH_FILE)
                | ((self_inner >> 10) & NOT_GH_FILE)
                | ((self_inner >> 17) & NOT_H_FILE),
        )
    }

    const fn gen_king_mask(self) -> Self {
        let self_inner: u64 = self.0;
        let lateral_mask = ((self_inner << 1) & NOT_A_FILE) | ((self_inner >> 1) & NOT_H_FILE);
        let screen_mask = lateral_mask | self_inner;
        BitBoard::new(lateral_mask | (screen_mask << 8) | (screen_mask >> 8))
    }
}

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Shl<u8> for BitBoard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u8> for BitBoard {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{Column, Row};
    #[test]
    fn test_straight_moves() {
        let east_moves = BitBoard::from(Square::new(9)).gen_east_mask();
        let expected = BitBoard::from(Row::new(1))
            ^ BitBoard::from(Square::new(8))
            ^ BitBoard::from(Square::new(9));
        assert_eq!(east_moves, expected);

        let north_moves = BitBoard::from(Square::new(9)).gen_north_mask();
        let expected = BitBoard::from(Column::new(1))
            ^ BitBoard::from(Square::new(1))
            ^ BitBoard::from(Square::new(9));
        assert_eq!(north_moves, expected);

        let west_moves = BitBoard::from(Square::new(9)).gen_west_mask();
        let expected = BitBoard::from(Square::new(8));
        assert_eq!(west_moves, expected);

        let south_moves = BitBoard::from(Square::new(9)).gen_south_mask();
        let expected = BitBoard::from(Square::new(1));
        assert_eq!(south_moves, expected);
    }

    #[test]
    fn test_knight_moves() {
        let knight_moves = BitBoard::from(Square::new(0)).gen_knight_mask();
        let expected = BitBoard::from(Square::new(10)) | BitBoard::from(Square::new(17));
        assert_eq!(knight_moves, expected);

        let knight_moves = BitBoard::from(Square::new(45)).gen_knight_mask();
        let expected = BitBoard::from(Square::new(28))
            | BitBoard::from(Square::new(30))
            | BitBoard::from(Square::new(35))
            | BitBoard::from(Square::new(39))
            | BitBoard::from(Square::new(51))
            | BitBoard::from(Square::new(55))
            | BitBoard::from(Square::new(60))
            | BitBoard::from(Square::new(62));
        assert_eq!(knight_moves, expected);
    }

    #[test]
    fn test_king_moves() {
        let king_moves = BitBoard::from(Square::new(0)).gen_king_mask();
        let expected = BitBoard::from(Square::new(1))
            | BitBoard::from(Square::new(8))
            | BitBoard::from(Square::new(9));
        assert_eq!(king_moves, expected);

        let king_moves = BitBoard::from(Square::new(54)).gen_king_mask();
        let expected = BitBoard::from(Square::new(45))
            | BitBoard::from(Square::new(46))
            | BitBoard::from(Square::new(47))
            | BitBoard::from(Square::new(53))
            | BitBoard::from(Square::new(55))
            | BitBoard::from(Square::new(61))
            | BitBoard::from(Square::new(62))
            | BitBoard::from(Square::new(63));
        assert_eq!(king_moves, expected);
    }

    #[test]
    fn test_forward_bitscan() {
        let bitboard = BitBoard::from(Row::new(0));
        let lsb = bitboard.bitscan_forward().unwrap();
        assert_eq!(lsb, Square::new(0));

        let bitboard = BitBoard::from(Row::new(7));
        let lsb = bitboard.bitscan_forward().unwrap();
        assert_eq!(lsb, Square::new(56));

        let bitboard = BitBoard::from(Column::new(0));
        let lsb = bitboard.bitscan_forward().unwrap();
        assert_eq!(lsb, Square::new(0));

        let bitboard = BitBoard::from(Column::new(7));
        let lsb = bitboard.bitscan_forward().unwrap();
        assert_eq!(lsb, Square::new(7));

        let bitboard = BitBoard::from(Square::new(0));
        let lsb = bitboard.bitscan_forward().unwrap();
        assert_eq!(lsb, Square::new(0));

        let bitboard = BitBoard::from(Square::new(63));
        let lsb = bitboard.bitscan_forward().unwrap();
        assert_eq!(lsb, Square::new(63));

        let bitboard = BitBoard::new(u64::MAX);
        let lsb = bitboard.bitscan_forward().unwrap();
        assert_eq!(lsb, Square::new(0));
    }

    #[test]
    fn test_backward_bitscan() {
        let bitboard = BitBoard::from(Row::new(0));
        let lsb = bitboard.bitscan_backward().unwrap();
        assert_eq!(lsb, Square::new(7));

        let bitboard = BitBoard::from(Row::new(7));
        let lsb = bitboard.bitscan_backward().unwrap();
        assert_eq!(lsb, Square::new(63));

        let bitboard = BitBoard::from(Column::new(0));
        let lsb = bitboard.bitscan_backward().unwrap();
        assert_eq!(lsb, Square::new(56));

        let bitboard = BitBoard::from(Column::new(7));
        let lsb = bitboard.bitscan_backward().unwrap();
        assert_eq!(lsb, Square::new(63));

        let bitboard = BitBoard::from(Square::new(0));
        let lsb = bitboard.bitscan_backward().unwrap();
        assert_eq!(lsb, Square::new(0));

        let bitboard = BitBoard::from(Square::new(63));
        let lsb = bitboard.bitscan_backward().unwrap();
        assert_eq!(lsb, Square::new(63));

        let bitboard = BitBoard::new(u64::MAX);
        let lsb = bitboard.bitscan_backward().unwrap();
        assert_eq!(lsb, Square::new(63));
    }
}
