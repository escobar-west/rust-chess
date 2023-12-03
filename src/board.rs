mod bitmap;
mod components;
mod constants;
use crate::pieces::{Color, Figure, Piece};
use bitmap::BitMap;
use components::{Column, Row, Square};

#[derive(Default)]
pub struct Board {
    pawns: BitMap,
    rooks: BitMap,
    knights: BitMap,
    bishops: BitMap,
    queens: BitMap,
    kings: BitMap,
    white: BitMap,
    black: BitMap,
    occupied: BitMap,
}

impl Board {
    fn set_mask_to_piece(&mut self, mask: BitMap, piece: Piece) {
        let piece_bitmap = match piece.figure {
            Figure::Pawn => &mut self.pawns,
            Figure::Rook => &mut self.rooks,
            Figure::Knight => &mut self.knights,
            Figure::Bishop => &mut self.bishops,
            Figure::Queen => &mut self.queens,
            Figure::King => &mut self.kings,
        };
        *piece_bitmap |= mask;
        let color_bitmap = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };
        *color_bitmap |= mask;
        self.occupied |= mask;
    }

    fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let position = fen.split_whitespace().next().ok_or("Empty Fen")?;
        let fen_positions: Vec<&str> = position.split("/").collect();
        if fen_positions.len() != 8 {
            return Err("Invalid number of rows");
        }
        let mut board = Self::default();
        for (row_idx, fen_row) in (0..8u8).rev().zip(fen_positions.iter()) {
            println!("{:#?}, {:#?}", row_idx, fen_row);
            let mut col_idx = 0u8;
            for c in fen_row.chars() {
                if c.is_digit(10) {
                    col_idx += c.to_digit(10).unwrap() as u8;
                } else {
                    let piece = Piece::try_from(c)?;
                    let square = Square::new(Row(row_idx), Column(col_idx));
                    let square: BitMap = square.into();
                    board.set_mask_to_piece(square, piece);
                    col_idx += 1;
                }
            }
        }
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let result = Board::from_fen(fen).unwrap();

        let pawn_mask = BitMap::from(Row(1)) | BitMap::from(Row(6));
        assert_eq!(result.pawns, pawn_mask);

        let rook_mask = BitMap::from(Square(0))
            | BitMap::from(Square(7))
            | BitMap::from(Square(56))
            | BitMap::from(Square(63));
        assert_eq!(result.rooks, rook_mask);

        let knight_mask = BitMap::from(Square(1))
            | BitMap::from(Square(6))
            | BitMap::from(Square(57))
            | BitMap::from(Square(62));
        assert_eq!(result.knights, knight_mask);

        let bishop_mask = BitMap::from(Square(2))
            | BitMap::from(Square(5))
            | BitMap::from(Square(58))
            | BitMap::from(Square(61));
        assert_eq!(result.bishops, bishop_mask);

        let queen_mask = BitMap::from(Square(3)) | BitMap::from(Square(59));
        assert_eq!(result.queens, queen_mask);

        let king_mask = BitMap::from(Square(4)) | BitMap::from(Square(60));
        assert_eq!(result.kings, king_mask);

        let white_mask = BitMap::from(Row(0)) | BitMap::from(Row(1));
        assert_eq!(result.white, white_mask);

        let black_mask = BitMap::from(Row(6)) | BitMap::from(Row(7));
        assert_eq!(result.black, black_mask);

        let occ_mask = white_mask | black_mask;
        assert_eq!(result.occupied, occ_mask);
    }
}
