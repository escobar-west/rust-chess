use super::bitboard::{BitBoard, COLUMNS, KING_MOVES, KNIGHT_MOVES, ROWS, SQUARES};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Row(u8);

impl Row {
    pub const fn new(value: u8) -> Self {
        assert!(value & 7 == value);
        Self(value)
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }

    pub const fn as_bitboard(self) -> BitBoard {
        BitBoard::new(0xff << (8 * self.0))
    }
}

impl From<Row> for BitBoard {
    fn from(value: Row) -> Self {
        ROWS[value.0 as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Column(u8);

impl Column {
    pub const fn new(value: u8) -> Self {
        assert!(value & 7 == value);
        Self(value)
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }

    pub const fn as_bitboard(self) -> BitBoard {
        BitBoard::new(0x0101010101010101 << self.0)
    }
}

impl From<Column> for BitBoard {
    fn from(value: Column) -> Self {
        COLUMNS[value.0 as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    pub const fn new(value: u8) -> Self {
        assert!(value & 63 == value);
        Self(value)
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }

    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    pub const fn as_bitboard(self) -> BitBoard {
        BitBoard::new(1 << self.0)
    }

    pub const fn get_row(self) -> Row {
        Row(self.0 >> 3)
    }

    pub const fn get_col(self) -> Column {
        Column(self.0 & 7)
    }

    pub const fn from_coords(row: Row, col: Column) -> Self {
        Self(col.0 + 8 * row.0)
    }

    pub fn get_knight_moves(self) -> BitBoard {
        KNIGHT_MOVES[usize::from(self)]
    }

    pub fn get_king_moves(self) -> BitBoard {
        KING_MOVES[usize::from(self)]
    }

    pub fn to_alg(self) -> String {
        let col = self.0 & 7;
        let row = self.0 >> 3;
        let col_char = char::from_u32(97 + col as u32).unwrap();
        let row_char = char::from_u32(49 + row as u32).unwrap();
        let mut notation = String::with_capacity(2);
        notation.push(col_char);
        notation.push(row_char);
        notation
    }

    pub fn try_from_alg(coords: &str) -> Result<Self, &'static str> {
        let mut iter = coords.chars();
        let col = match iter.next() {
            Some(c @ 'a'..='h') => u32::from(c) - 97u32,
            _ => return Err("invalid column"),
        };
        let row = match iter.next() {
            Some(r @ '1'..='8') => u32::from(r) - 49u32,
            _ => return Err("invalid row"),
        };
        Ok(Square::new((col + 8 * row) as u8))
    }

    #[cfg(test)]
    pub fn from_alg(coords: &str) -> Self {
        Self::try_from_alg(coords).unwrap()
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        SQUARES[value.0 as usize]
    }
}

impl From<Square> for u8 {
    fn from(value: Square) -> Self {
        value.0
    }
}

impl From<Square> for usize {
    fn from(value: Square) -> Self {
        value.0 as usize
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_alg())
    }
}
