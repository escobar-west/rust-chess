use crate::board::{
    components::{Column, Row, Square},
    constants::{COLUMNS, ROWS, SQUARES},
};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct BitMap(u64);
impl BitMap {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl From<Row> for BitMap {
    fn from(value: Row) -> Self {
        ROWS[value.0 as usize]
    }
}

impl From<Column> for BitMap {
    fn from(value: Column) -> Self {
        COLUMNS[value.0 as usize]
    }
}

impl From<Square> for BitMap {
    fn from(value: Square) -> Self {
        SQUARES[value.0 as usize]
    }
}

impl BitAnd for BitMap {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for BitMap {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAndAssign for BitMap {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl BitOrAssign for BitMap {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}
