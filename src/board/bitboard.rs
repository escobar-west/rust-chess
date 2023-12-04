mod constants;
pub use constants::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_AB_FILE: u64 = 0xfcfcfcfcfcfcfcfc;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
const NOT_GH_FILE: u64 = 0x3f3f3f3f3f3f3f3f;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub const fn knight_moves(self) -> Self {
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

    pub const fn king_moves(self) -> Self {
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
    use crate::board::Square;
    #[test]
    fn test_knight_moves() {
        let knight_moves = BitBoard::from(Square::new(0)).knight_moves();
        let expected = BitBoard::from(Square::new(10)) | BitBoard::from(Square::new(17));
        assert_eq!(knight_moves, expected);

        let knight_moves = BitBoard::from(Square::new(45)).knight_moves();
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
        let knight_moves = BitBoard::from(Square::new(0)).king_moves();
        let expected = BitBoard::from(Square::new(1))
            | BitBoard::from(Square::new(8))
            | BitBoard::from(Square::new(9));
        assert_eq!(knight_moves, expected);

        let knight_moves = BitBoard::from(Square::new(54)).king_moves();
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
}
