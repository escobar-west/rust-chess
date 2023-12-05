mod constants;
use super::Square;
pub use constants::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

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

    pub fn bitscan_forward(&self) -> Square {
        let self_inner: u64 = self.0;
        assert!(self_inner != 0);
        let lookup_idx = DEBRUIJN64.wrapping_mul(self_inner & self_inner.wrapping_neg()) >> 58;
        Square::new(FWDSCAN[lookup_idx as usize])
    }

    pub fn biscan_backward(&self) -> Square {
        let mut self_inner: u64 = self.0;
        assert!(self_inner != 0);
        self_inner |= self_inner >> 1;
        self_inner |= self_inner >> 2;
        self_inner |= self_inner >> 4;
        self_inner |= self_inner >> 8;
        self_inner |= self_inner >> 16;
        self_inner |= self_inner >> 32;
        let lookup_idx = self_inner.wrapping_mul(DEBRUIJN64) >> 58;
        Square::new(BACKSCAN[lookup_idx as usize])
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

    //TODO: fix east and west generation, which is not correct
    const fn gen_east_mask(self) -> Self {
        let self_inner = self.0;
        let mut mask: u64 = 0;
        mask |= (Self::gen_col_mask(0).0 & self_inner) << 1;
        mask |= (Self::gen_col_mask(1).0 & self_inner) << 1;
        mask |= (Self::gen_col_mask(2).0 & self_inner) << 1;
        mask |= (Self::gen_col_mask(3).0 & self_inner) << 1;
        mask |= (Self::gen_col_mask(4).0 & self_inner) << 1;
        mask |= (Self::gen_col_mask(5).0 & self_inner) << 1;
        mask |= (Self::gen_col_mask(6).0 & self_inner) << 1;
        Self(mask)
    }

    const fn gen_north_mask(self) -> Self {
        let self_inner = self.0;
        let mut mask: u64 = 0;
        mask |= self_inner << 8 * 1;
        mask |= self_inner << 8 * 2;
        mask |= self_inner << 8 * 3;
        mask |= self_inner << 8 * 4;
        mask |= self_inner << 8 * 5;
        mask |= self_inner << 8 * 6;
        mask |= self_inner << 8 * 7;
        Self(mask)
    }

    const fn gen_west_mask(self) -> Self {
        let self_inner = self.0;
        let mut mask: u64 = 0;
        mask |= (Self::gen_col_mask(7).0 & self_inner) >> 1;
        mask |= (Self::gen_col_mask(6).0 & self_inner) >> 1;
        mask |= (Self::gen_col_mask(5).0 & self_inner) >> 1;
        mask |= (Self::gen_col_mask(4).0 & self_inner) >> 1;
        mask |= (Self::gen_col_mask(3).0 & self_inner) >> 1;
        mask |= (Self::gen_col_mask(2).0 & self_inner) >> 1;
        mask |= (Self::gen_col_mask(1).0 & self_inner) >> 1;
        Self(mask)
    }

    const fn gen_south_mask(self) -> Self {
        let self_inner = self.0;
        let mut mask: u64 = 0;
        mask |= self_inner >> 8 * 1;
        mask |= self_inner >> 8 * 2;
        mask |= self_inner >> 8 * 3;
        mask |= self_inner >> 8 * 4;
        mask |= self_inner >> 8 * 5;
        mask |= self_inner >> 8 * 6;
        mask |= self_inner >> 8 * 7;
        Self(mask)
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
        let knight_moves = BitBoard::from(Square::new(0)).gen_king_mask();
        let expected = BitBoard::from(Square::new(1))
            | BitBoard::from(Square::new(8))
            | BitBoard::from(Square::new(9));
        assert_eq!(knight_moves, expected);

        let knight_moves = BitBoard::from(Square::new(54)).gen_king_mask();
        let expected = BitBoard::from(Square::new(45))
            | BitBoard::from(Square::new(46))
            | BitBoard::from(Square::new(47))
            | BitBoard::from(Square::new(53))
            | BitBoard::from(Square::new(55))
            | BitBoard::from(Square::new(61))
            | BitBoard::from(Square::new(62))
            | BitBoard::from(Square::new(63));
        assert_eq!(knight_moves, expected);
    }

    #[test]
    fn test_forward_bitscan() {
        let bitboard = BitBoard::from(Row::new(0));
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Square::new(0));

        let bitboard = BitBoard::from(Row::new(7));
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Square::new(56));

        let bitboard = BitBoard::from(Column::new(0));
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Square::new(0));

        let bitboard = BitBoard::from(Column::new(7));
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Square::new(7));

        let bitboard = BitBoard::from(Square::new(0));
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Square::new(0));

        let bitboard = BitBoard::from(Square::new(63));
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Square::new(63));

        let bitboard = BitBoard::new(u64::MAX);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Square::new(0));
    }

    #[test]
    fn test_backward_bitscan() {
        let bitboard = BitBoard::from(Row::new(0));
        let lsb = bitboard.biscan_backward();
        assert_eq!(lsb, Square::new(7));

        let bitboard = BitBoard::from(Row::new(7));
        let lsb = bitboard.biscan_backward();
        assert_eq!(lsb, Square::new(63));

        let bitboard = BitBoard::from(Column::new(0));
        let lsb = bitboard.biscan_backward();
        assert_eq!(lsb, Square::new(56));

        let bitboard = BitBoard::from(Column::new(7));
        let lsb = bitboard.biscan_backward();
        assert_eq!(lsb, Square::new(63));

        let bitboard = BitBoard::from(Square::new(0));
        let lsb = bitboard.biscan_backward();
        assert_eq!(lsb, Square::new(0));

        let bitboard = BitBoard::from(Square::new(63));
        let lsb = bitboard.biscan_backward();
        assert_eq!(lsb, Square::new(63));

        let bitboard = BitBoard::new(u64::MAX);
        let lsb = bitboard.biscan_backward();
        assert_eq!(lsb, Square::new(63));
    }
}
