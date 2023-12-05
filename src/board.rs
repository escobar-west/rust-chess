mod bitboard;
mod components;
mod mailbox;

use crate::pieces::{Color, Figure, Piece};
use bitboard::{BitBoard, KING_MOVES, KNIGHT_MOVES};
pub use components::{Column, Row, Square};
use mailbox::MailBox;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Board {
    pawns: BitBoard,
    rooks: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
    white: BitBoard,
    black: BitBoard,
    occupied: BitBoard,
    mailbox: MailBox,
}

impl Board {
    fn get_piece_map(&self, figure: Figure) -> BitBoard {
        match figure {
            Figure::Pawn => self.pawns,
            Figure::Rook => self.rooks,
            Figure::Knight => self.knights,
            Figure::Bishop => self.bishops,
            Figure::Queen => self.queens,
            Figure::King => self.kings,
        }
    }
    fn get_piece_map_mut(&mut self, figure: Figure) -> &mut BitBoard {
        match figure {
            Figure::Pawn => &mut self.pawns,
            Figure::Rook => &mut self.rooks,
            Figure::Knight => &mut self.knights,
            Figure::Bishop => &mut self.bishops,
            Figure::Queen => &mut self.queens,
            Figure::King => &mut self.kings,
        }
    }

    fn get_color_map(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }

    fn get_color_map_mut(&mut self, color: Color) -> &mut BitBoard {
        match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }

    fn clear_mask_by_piece(&mut self, mask: BitBoard, piece: Piece) {
        let clear_mask = !mask;
        *self.get_piece_map_mut(piece.figure) &= clear_mask;
        *self.get_color_map_mut(piece.color) &= clear_mask;
        self.occupied &= clear_mask;
    }

    fn set_mask_by_piece(&mut self, mask: BitBoard, piece: Piece) {
        *self.get_piece_map_mut(piece.figure) |= mask;
        *self.get_color_map_mut(piece.color) |= mask;
        self.occupied |= mask;
    }

    pub fn get_piece_at_square(&self, square: Square) -> Option<Piece> {
        self.mailbox.get_piece_at_square(square)
    }

    pub fn clear_square(&mut self, square: Square) -> Option<Piece> {
        let old_piece = self.mailbox.clear_square(square);
        old_piece.map(|p| self.clear_mask_by_piece(square.into(), p));
        old_piece
    }

    pub fn set_piece_at_square(&mut self, square: Square, piece: Piece) -> Option<Piece> {
        let old_piece = self.mailbox.set_piece_at_square(square, piece);
        let square_mask: BitBoard = square.into();
        old_piece.map(|old_p| self.clear_mask_by_piece(square_mask, old_p));
        self.set_mask_by_piece(square_mask, piece);
        old_piece
    }

    pub fn move_piece(&mut self, from: Square, to: Square) -> Option<Piece> {
        self.clear_square(from)
            .map(|p| self.set_piece_at_square(to, p))
            .flatten()
    }

    pub fn get_legal_moves_at_square(&self, square: Square) -> BitBoard {
        let piece = match self.get_piece_at_square(square) {
            Some(p) => p,
            None => return BitBoard::new(0),
        };
        let move_mask = match piece.figure {
            Figure::Rook => self.get_rook_moves(square),
            Figure::Knight => self.get_knight_moves(square),
            Figure::King => self.get_king_moves(square),
            _ => BitBoard::new(0),
        };
        move_mask & !self.get_color_map(piece.color)
    }

    pub fn get_rook_moves(&self, square: Square) -> BitBoard {
        BitBoard::new(0)
    }

    pub fn get_knight_moves(&self, square: Square) -> BitBoard {
        KNIGHT_MOVES[usize::from(square)]
    }

    pub fn get_king_moves(&self, square: Square) -> BitBoard {
        KING_MOVES[usize::from(square)]
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
        let position = fen.split(" ").next().ok_or("Empty Fen")?;
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
                    let square = Square::from_coords(Row::new(row_idx), Column::new(col_idx));
                    board.set_piece_at_square(square, piece);
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
                let square = Square::from_coords(Row::new(row_idx), Column::new(col_idx));
                match self.mailbox.get_piece_at_square(square) {
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

        let pawn_mask = BitBoard::from(Row::new(1)) | BitBoard::from(Row::new(6));
        assert_eq!(board.pawns, pawn_mask);

        let rook_mask = BitBoard::from(Square::new(0))
            | BitBoard::from(Square::new(7))
            | BitBoard::from(Square::new(56))
            | BitBoard::from(Square::new(63));
        assert_eq!(board.rooks, rook_mask);

        let knight_mask = BitBoard::from(Square::new(1))
            | BitBoard::from(Square::new(6))
            | BitBoard::from(Square::new(57))
            | BitBoard::from(Square::new(62));
        assert_eq!(board.knights, knight_mask);

        let bishop_mask = BitBoard::from(Square::new(2))
            | BitBoard::from(Square::new(5))
            | BitBoard::from(Square::new(58))
            | BitBoard::from(Square::new(61));
        assert_eq!(board.bishops, bishop_mask);

        let queen_mask = BitBoard::from(Square::new(3)) | BitBoard::from(Square::new(59));
        assert_eq!(board.queens, queen_mask);

        let king_mask = BitBoard::from(Square::new(4)) | BitBoard::from(Square::new(60));
        assert_eq!(board.kings, king_mask);

        let white_mask = BitBoard::from(Row::new(0)) | BitBoard::from(Row::new(1));
        assert_eq!(board.white, white_mask);

        let black_mask = BitBoard::from(Row::new(6)) | BitBoard::from(Row::new(7));
        assert_eq!(board.black, black_mask);

        let occ_mask = white_mask | black_mask;
        assert_eq!(board.occupied, occ_mask);

        let to_fen = board.to_fen();
        assert_eq!(to_fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    #[test]
    fn test_clear_and_set_piece_at_square() {
        let fen = DEFAULT_FEN;
        let mut board = Board::try_from_fen(fen).unwrap();
        board.clear_square(Square::new(56));

        let rook_mask = BitBoard::from(Square::new(0))
            | BitBoard::from(Square::new(7))
            | BitBoard::from(Square::new(63));
        assert_eq!(board.rooks, rook_mask);

        let piece = board.set_piece_at_square(
            Square::new(7),
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

        let rook_mask = BitBoard::from(Square::new(0)) | BitBoard::from(Square::new(63));
        assert_eq!(board.rooks, rook_mask);

        let queen_mask = BitBoard::from(Square::new(3))
            | BitBoard::from(Square::new(7))
            | BitBoard::from(Square::new(59));
        assert_eq!(board.queens, queen_mask);

        let white_mask = (BitBoard::from(Row::new(0)) | BitBoard::from(Row::new(1)))
            ^ BitBoard::from(Square::new(7));
        assert_eq!(board.white, white_mask);

        let black_mask = (BitBoard::from(Row::new(6))
            | BitBoard::from(Row::new(7))
            | BitBoard::from(Square::new(7)))
            ^ BitBoard::from(Square::new(56));
        assert_eq!(board.black, black_mask);
    }

    #[test]
    fn test_move_piece() {
        let fen = DEFAULT_FEN;
        let mut board = Board::try_from_fen(fen).unwrap();
        board.move_piece(Square::new(12), Square::new(28));
        let new_fen = board.to_fen();
        assert_eq!(new_fen, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR");
        board.move_piece(Square::new(50), Square::new(34));
        let new_fen = board.to_fen();
        assert_eq!(new_fen, "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR");
        board.move_piece(Square::new(0), Square::new(63));
        let new_fen = board.to_fen();
        assert_eq!(new_fen, "rnbqkbnR/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/1NBQKBNR");
    }
}
