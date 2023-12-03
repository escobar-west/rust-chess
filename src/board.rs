mod bitmap;
mod components;
mod constants;
mod mailbox;
use crate::pieces::{Color, Figure, Piece};
use bitmap::BitMap;
use components::{Column, Row, Square};
use mailbox::MailBox;

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
    mailbox: MailBox,
}

impl Board {
    fn get_piece_map(&mut self, figure: Figure) -> &mut BitMap {
        match figure {
            Figure::Pawn => &mut self.pawns,
            Figure::Rook => &mut self.rooks,
            Figure::Knight => &mut self.knights,
            Figure::Bishop => &mut self.bishops,
            Figure::Queen => &mut self.queens,
            Figure::King => &mut self.kings,
        }
    }

    fn get_color_map(&mut self, color: Color) -> &mut BitMap {
        match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }

    fn clear_mask_by_piece(&mut self, mask: BitMap, piece: Piece) {
        let clear_mask = !mask;
        *self.get_piece_map(piece.figure) &= clear_mask;
        *self.get_color_map(piece.color) &= clear_mask;
        self.occupied &= clear_mask;
    }

    fn set_mask_by_piece(&mut self, mask: BitMap, piece: Piece) {
        *self.get_piece_map(piece.figure) |= mask;
        *self.get_color_map(piece.color) |= mask;
        self.occupied |= mask;
    }

    pub fn get_square(&self, square: Square) -> Option<Piece> {
        self.mailbox.get_square(square)
    }

    pub fn clear_square(&mut self, square: Square) -> Option<Piece> {
        let piece = self.mailbox.clear_square(square);
        piece.map(|p| self.clear_mask_by_piece(square.into(), p));
        piece
    }

    pub fn set_square(&mut self, square: Square, piece: Piece) -> Option<Piece> {
        let old_piece = self.mailbox.set_square(square, piece);
        if old_piece == Some(piece) {
            return old_piece;
        }
        let square_mask: BitMap = square.into();
        old_piece.map(|old_p| self.clear_mask_by_piece(square_mask, old_p));
        self.set_mask_by_piece(square_mask, piece);
        old_piece
    }

    pub fn move_piece(&mut self, from: Square, to: Square) -> Option<Piece> {
        if from == to {
            return None;
        }
        self.clear_square(from)
            .map(|p| self.set_square(to, p))
            .flatten()
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
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
                    board.set_square(square, piece);
                    col_idx += 1;
                }
            }
        }
        Ok(board)
    }

    pub fn to_fen(&self) -> String {
        let mut fen_row_list: Vec<String> = Vec::with_capacity(8);
        for row_idx in (0..8u8).rev() {
            let mut none_count = 0u8;
            let mut fen_row: String = "".into();
            for col_idx in 0..8u8 {
                let square = Square::new(Row(row_idx), Column(col_idx));
                match self.mailbox.get_square(square) {
                    Some(p) => {
                        if none_count != 0 {
                            fen_row.push_str(&format!("{}", none_count));
                            none_count = 0;
                        }
                        fen_row.push(p.into());
                    }
                    None => {
                        none_count += 1;
                    }
                }
            }
            if none_count != 0 {
                fen_row.push_str(&format!("{}", none_count));
            }
            fen_row_list.push(fen_row);
        }
        fen_row_list.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gamestate::DEFAULT_FEN;
    #[test]
    fn test_default_fen() {
        let fen = DEFAULT_FEN;
        let board = Board::try_from_fen(fen).unwrap();

        let pawn_mask = BitMap::from(Row(1)) | BitMap::from(Row(6));
        assert_eq!(board.pawns, pawn_mask);

        let rook_mask = BitMap::from(Square(0))
            | BitMap::from(Square(7))
            | BitMap::from(Square(56))
            | BitMap::from(Square(63));
        assert_eq!(board.rooks, rook_mask);

        let knight_mask = BitMap::from(Square(1))
            | BitMap::from(Square(6))
            | BitMap::from(Square(57))
            | BitMap::from(Square(62));
        assert_eq!(board.knights, knight_mask);

        let bishop_mask = BitMap::from(Square(2))
            | BitMap::from(Square(5))
            | BitMap::from(Square(58))
            | BitMap::from(Square(61));
        assert_eq!(board.bishops, bishop_mask);

        let queen_mask = BitMap::from(Square(3)) | BitMap::from(Square(59));
        assert_eq!(board.queens, queen_mask);

        let king_mask = BitMap::from(Square(4)) | BitMap::from(Square(60));
        assert_eq!(board.kings, king_mask);

        let white_mask = BitMap::from(Row(0)) | BitMap::from(Row(1));
        assert_eq!(board.white, white_mask);

        let black_mask = BitMap::from(Row(6)) | BitMap::from(Row(7));
        assert_eq!(board.black, black_mask);

        let occ_mask = white_mask | black_mask;
        assert_eq!(board.occupied, occ_mask);

        let to_fen = board.to_fen();
        assert_eq!(to_fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    #[test]
    fn test_clear_and_set_square() {
        let fen = DEFAULT_FEN;
        let mut board = Board::try_from_fen(fen).unwrap();
        board.clear_square(Square(56));

        let rook_mask =
            BitMap::from(Square(0)) | BitMap::from(Square(7)) | BitMap::from(Square(63));
        assert_eq!(board.rooks, rook_mask);

        let piece = board.set_square(
            Square(7),
            Piece {
                color: Color::Black,
                figure: Figure::Queen,
            },
        );
        assert_eq!(
            piece,
            Some(Piece {
                color: Color::White,
                figure: Figure::Rook
            })
        );

        let rook_mask = BitMap::from(Square(0)) | BitMap::from(Square(63));
        assert_eq!(board.rooks, rook_mask);

        let queen_mask =
            BitMap::from(Square(3)) | BitMap::from(Square(7)) | BitMap::from(Square(59));
        assert_eq!(board.queens, queen_mask);

        let white_mask = (BitMap::from(Row(0)) | BitMap::from(Row(1))) ^ BitMap::from(Square(7));
        assert_eq!(board.white, white_mask);

        let black_mask = (BitMap::from(Row(6)) | BitMap::from(Row(7)) | BitMap::from(Square(7)))
            ^ BitMap::from(Square(56));
        assert_eq!(board.black, black_mask);
    }

    #[test]
    fn test_move_piece() {
        let fen = DEFAULT_FEN;
        let mut board = Board::try_from_fen(fen).unwrap();
        board.move_piece(Square(12), Square(28));
        let new_fen = board.to_fen();
        assert_eq!(new_fen, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR");
        board.move_piece(Square(50), Square(34));
        let new_fen = board.to_fen();
        assert_eq!(new_fen, "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR");
        board.move_piece(Square(0), Square(63));
        let new_fen = board.to_fen();
        assert_eq!(new_fen, "rnbqkbnR/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/1NBQKBNR");
    }
}
