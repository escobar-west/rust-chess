use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

struct Row(u8);
impl Row {
    fn new(row: u8) -> Result<Self, &'static str> {
        if row & 7 != row {
            return Err("row must be in range of 0 and 7");
        }
        Ok(Self(row))
    }
}
struct File(u8);
impl File {
    fn new(file: u8) -> Result<Self, &'static str> {
        if file & 7 != file {
            return Err("file must be in range of 0 and 7");
        }
        Ok(Self(file))
    }
}
enum Color {
    White,
    Black,
}
enum Figure {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
struct Piece {
    color: Color,
    figure: Figure,
}

impl TryFrom<char> for Piece {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(Piece {
                color: Color::White,
                figure: Figure::Pawn,
            }),
            'N' => Ok(Piece {
                color: Color::White,
                figure: Figure::Knight,
            }),
            'B' => Ok(Piece {
                color: Color::White,
                figure: Figure::Bishop,
            }),
            'R' => Ok(Piece {
                color: Color::White,
                figure: Figure::Rook,
            }),
            'Q' => Ok(Piece {
                color: Color::White,
                figure: Figure::Queen,
            }),
            'K' => Ok(Piece {
                color: Color::White,
                figure: Figure::King,
            }),
            'p' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Pawn,
            }),
            'n' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Knight,
            }),
            'b' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Bishop,
            }),
            'r' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Rook,
            }),
            'q' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Queen,
            }),
            'k' => Ok(Piece {
                color: Color::Black,
                figure: Figure::King,
            }),
            _ => Err("Invalid char"),
        }
    }
}
struct Square {
    row: Row,
    file: File,
}
#[derive(Default)]
struct BitMap(u64);
impl BitMap {
    const fn new(int: u64) -> Self {
        Self(int)
    }
    const fn from_row(row: Row) -> Self {
        Self(0xff << (8 * row.0))
    }
    const fn from_file(file: File) -> Self {
        Self(0x0101_0101_0101_0101 << file.0)
    }
    const fn from_square(square: Square) -> Self {
        Self(1 << (square.file.0 + 8 * square.row.0))
    }
}
impl BitAnd for BitMap {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl BitOr for BitMap {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl BitAndAssign for BitMap {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl BitOrAssign for BitMap {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}
#[derive(Default)]
struct Board {
    white_pawns: BitMap,
    white_knights: BitMap,
    white_bishops: BitMap,
    white_rooks: BitMap,
    white_queens: BitMap,
    white_king: BitMap,
    black_pawns: BitMap,
    black_knights: BitMap,
    black_bishops: BitMap,
    black_rooks: BitMap,
    black_queens: BitMap,
    black_king: BitMap,
}

impl Board {
    fn get_bitmap_mut(&mut self, piece: Piece) -> &mut BitMap {
        match (piece.figure, piece.color) {
            (Figure::Pawn, Color::White) => &mut self.white_pawns,
            (Figure::Knight, Color::White) => &mut self.white_knights,
            (Figure::Bishop, Color::White) => &mut self.white_bishops,
            (Figure::Rook, Color::White) => &mut self.white_rooks,
            (Figure::Queen, Color::White) => &mut self.white_queens,
            (Figure::King, Color::White) => &mut self.white_king,
            (Figure::Pawn, Color::Black) => &mut self.black_pawns,
            (Figure::Knight, Color::Black) => &mut self.black_knights,
            (Figure::Bishop, Color::Black) => &mut self.black_bishops,
            (Figure::Rook, Color::Black) => &mut self.black_rooks,
            (Figure::Queen, Color::Black) => &mut self.black_queens,
            (Figure::King, Color::Black) => &mut self.black_king,
        }
    }
    fn get_bitmap(&self, piece: Piece) -> &BitMap {
        match (piece.figure, piece.color) {
            (Figure::Pawn, Color::White) => &self.white_pawns,
            (Figure::Knight, Color::White) => &self.white_knights,
            (Figure::Bishop, Color::White) => &self.white_bishops,
            (Figure::Rook, Color::White) => &self.white_rooks,
            (Figure::Queen, Color::White) => &self.white_queens,
            (Figure::King, Color::White) => &self.white_king,
            (Figure::Pawn, Color::Black) => &self.black_pawns,
            (Figure::Knight, Color::Black) => &self.black_knights,
            (Figure::Bishop, Color::Black) => &self.black_bishops,
            (Figure::Rook, Color::Black) => &self.black_rooks,
            (Figure::Queen, Color::Black) => &self.black_queens,
            (Figure::King, Color::Black) => &self.black_king,
        }
    }
    fn add_piece_to_square(&mut self, piece: Piece, square: Square) {
        let square_mask = BitMap::from_square(square);
        *self.get_bitmap_mut(piece) |= square_mask;
    }
    fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let position = fen.split_whitespace().next().ok_or("Empty Fen")?;
        let fen_positions: Vec<&str> = position.split("/").collect();
        if fen_positions.len() != 8 {
            return Err("Invalid number of rows");
        }
        let mut board = Self::default();
        for (row_idx, fen_row) in (0..8u8).rev().zip(fen_positions.iter()) {
            let mut file_idx = 0u8;
            for c in fen_row.chars() {
                if c.is_digit(10) {
                    file_idx += c.to_digit(10).unwrap() as u8;
                } else {
                    let piece = Piece::try_from(c)?;
                    let square = Square {
                        row: Row::new(row_idx)?,
                        file: File::new(file_idx)?,
                    };
                    board.add_piece_to_square(piece, square);
                    file_idx += 1;
                }
            }
        }
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
