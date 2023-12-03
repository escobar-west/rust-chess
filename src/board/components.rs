#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Row(pub u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Column(pub u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(pub u8);

impl Square {
    pub fn new(row: Row, col: Column) -> Self {
        Self(col.0 + 8 * row.0)
    }
}
