mod bitboard;
mod components;
mod mailbox;

use crate::pieces::{constants::*, Color, Figure, Piece};
pub use bitboard::{BitBoard, EMPTY_BOARD, FULL_BOARD};
use bitboard::{
    Direction, BLACK_PAWN_ATTACKS, DIAG_MOVES, KING_MOVES, KNIGHT_MOVES, STRAIGHT_MOVES,
    WHITE_PAWN_ATTACKS,
};
pub use components::{Column, Row, Square};
use mailbox::MailBox;

type BitBoardRayTable = [[BitBoard; 4]; 64];

#[derive(Debug, Default, PartialEq, Eq)]
struct ColorBoard {
    pawns: BitBoard,
    rooks: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Board {
    white_pieces: ColorBoard,
    black_pieces: ColorBoard,
    white_occupied: BitBoard,
    black_occupied: BitBoard,
    occupied: BitBoard,
    mailbox: MailBox,
}

impl Board {
    pub fn get_pieces(&self, piece: Piece) -> BitBoard {
        match piece {
            WHITE_PAWN => self.white_pieces.pawns,
            WHITE_ROOK => self.white_pieces.rooks,
            WHITE_KNIGHT => self.white_pieces.knights,
            WHITE_BISHOP => self.white_pieces.bishops,
            WHITE_QUEEN => self.white_pieces.queens,
            WHITE_KING => self.white_pieces.kings,
            BLACK_PAWN => self.black_pieces.pawns,
            BLACK_ROOK => self.black_pieces.rooks,
            BLACK_KNIGHT => self.black_pieces.knights,
            BLACK_BISHOP => self.black_pieces.bishops,
            BLACK_QUEEN => self.black_pieces.queens,
            BLACK_KING => self.black_pieces.kings,
        }
    }

    pub fn get_colors(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white_occupied,
            Color::Black => self.black_occupied,
        }
    }

    pub fn get_straight_attackers(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white_pieces.rooks | self.white_pieces.queens,
            Color::Black => self.black_pieces.rooks | self.black_pieces.queens,
        }
    }

    pub fn get_diag_attackers(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white_pieces.bishops | self.white_pieces.queens,
            Color::Black => self.black_pieces.bishops | self.black_pieces.queens,
        }
    }

    pub fn get_square(&self, square: Square) -> Option<Piece> {
        self.mailbox.get_square(square)
    }

    pub fn clear_square(&mut self, square: Square) -> Option<Piece> {
        let piece = self.mailbox.clear_square(square);
        if let Some(p) = piece {
            self.clear_mask_by_piece(square.into(), p)
        }
        piece
    }

    pub fn set_square(&mut self, square: Square, piece: Piece) -> Option<Piece> {
        let old_piece = self.mailbox.set_square(square, piece);
        let square_mask: BitBoard = square.into();
        if let Some(old_piece) = old_piece {
            self.clear_mask_by_piece(square_mask, old_piece)
        }
        self.set_mask_by_piece(square_mask, piece);
        old_piece
    }

    pub fn move_piece(&mut self, from: Square, to: Square) -> Option<Piece> {
        self.clear_square(from).and_then(|p| self.set_square(to, p))
    }

    pub fn get_move_mask(&self, square: Square, piece: Piece) -> BitBoard {
        let move_mask = match piece.figure {
            Figure::Pawn => self.get_pawn_moves(square, piece.color),
            Figure::Rook => self.get_ray_moves(square, &STRAIGHT_MOVES),
            Figure::Knight => self.get_knight_moves(square),
            Figure::Bishop => self.get_ray_moves(square, &DIAG_MOVES),
            Figure::Queen => {
                self.get_ray_moves(square, &STRAIGHT_MOVES)
                    | self.get_ray_moves(square, &DIAG_MOVES)
            }
            Figure::King => self.get_king_moves(square),
        };
        move_mask & !self.get_colors(piece.color)
    }

    pub fn is_attacked_by(&self, square: Square, attack_color: Color) -> bool {
        match attack_color {
            Color::White => {
                let straight_rays = self.white_pieces.rooks | self.white_pieces.queens;
                if (self.get_ray_moves(square, &STRAIGHT_MOVES) & straight_rays).is_not_empty() {
                    return true;
                }
                let diag_rays = self.white_pieces.bishops | self.white_pieces.queens;
                if (self.get_ray_moves(square, &DIAG_MOVES) & diag_rays).is_not_empty() {
                    return true;
                }
                if (self.get_knight_moves(square) & self.white_pieces.knights).is_not_empty() {
                    return true;
                }
                if (self.get_pawn_attacks(square, Color::Black) & self.white_pieces.pawns)
                    .is_not_empty()
                {
                    return true;
                }
                if (self.get_king_moves(square) & self.white_pieces.kings).is_not_empty() {
                    return true;
                }
                false
            }
            Color::Black => {
                let straight_rays = self.black_pieces.rooks | self.black_pieces.queens;
                if (self.get_ray_moves(square, &STRAIGHT_MOVES) & straight_rays).is_not_empty() {
                    return true;
                }
                let diag_rays = self.black_pieces.bishops | self.black_pieces.queens;
                if (self.get_ray_moves(square, &DIAG_MOVES) & diag_rays).is_not_empty() {
                    return true;
                }
                if (self.get_knight_moves(square) & self.black_pieces.knights).is_not_empty() {
                    return true;
                }
                if (self.get_pawn_attacks(square, Color::White) & self.black_pieces.pawns)
                    .is_not_empty()
                {
                    return true;
                }
                if (self.get_king_moves(square) & self.black_pieces.kings).is_not_empty() {
                    return true;
                }
                false
            }
        }
    }

    pub fn get_pin_mask(
        &self,
        pin_square: Square,
        target_square: Square,
        target_color: Color,
    ) -> BitBoard {
        let pin_square_mask = BitBoard::from(pin_square);
        let straight_pinner_mask = self.get_straight_attackers(!target_color);
        let diag_pinner_mask = self.get_diag_attackers(!target_color);
        for (rays_arr, pinner_mask) in [
            (STRAIGHT_MOVES, straight_pinner_mask),
            (DIAG_MOVES, diag_pinner_mask),
        ] {
            let ray_masks = rays_arr[usize::from(target_square)];
            let pin_participants = pin_square_mask | pinner_mask;
            for dir in [
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::South,
            ] {
                let king_ray_mask = ray_masks[dir as usize];
                let check_mask = king_ray_mask & pin_participants;
                if check_mask.is_empty() {
                    continue;
                }
                let first_blocker = match dir {
                    Direction::East | Direction::North => {
                        (king_ray_mask & self.occupied).bitscan_forward()
                    }
                    Direction::West | Direction::South => {
                        (king_ray_mask & self.occupied).bitscan_backward()
                    }
                };
                if first_blocker != Some(pin_square) {
                    continue;
                }
                let pin_ray_mask = rays_arr[usize::from(pin_square)][dir as usize];
                let second_blocker = match dir {
                    Direction::East | Direction::North => {
                        (pin_ray_mask & self.occupied).bitscan_forward()
                    }
                    Direction::West | Direction::South => {
                        (pin_ray_mask & self.occupied).bitscan_backward()
                    }
                };
                let Some(second_blocker) = second_blocker else {
                    continue;
                };
                if (pinner_mask & second_blocker.into()).is_empty() {
                    continue;
                }
                return king_ray_mask ^ rays_arr[usize::from(second_blocker)][dir as usize];
            }
        }
        FULL_BOARD
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
        let position = fen.split(' ').next().ok_or("Empty Fen")?;
        let fen_positions: Vec<&str> = position.split('/').collect();
        if fen_positions.len() != 8 {
            return Err("Invalid number of rows");
        }
        let mut board = Self::default();
        for (row_idx, fen_row) in (0..8u8).rev().zip(fen_positions.iter()) {
            let mut col_idx = 0u8;
            for c in fen_row.chars() {
                if c.is_ascii_digit() {
                    col_idx += c.to_digit(10).unwrap() as u8;
                } else {
                    let piece = Piece::try_from(c)?;
                    let square = Square::from_coords(Row::new(row_idx), Column::new(col_idx));
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
                let square = Square::from_coords(Row::new(row_idx), Column::new(col_idx));
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

    fn get_pieces_mut(&mut self, piece: Piece) -> &mut BitBoard {
        match piece {
            WHITE_PAWN => &mut self.white_pieces.pawns,
            WHITE_ROOK => &mut self.white_pieces.rooks,
            WHITE_KNIGHT => &mut self.white_pieces.knights,
            WHITE_BISHOP => &mut self.white_pieces.bishops,
            WHITE_QUEEN => &mut self.white_pieces.queens,
            WHITE_KING => &mut self.white_pieces.kings,
            BLACK_PAWN => &mut self.black_pieces.pawns,
            BLACK_ROOK => &mut self.black_pieces.rooks,
            BLACK_KNIGHT => &mut self.black_pieces.knights,
            BLACK_BISHOP => &mut self.black_pieces.bishops,
            BLACK_QUEEN => &mut self.black_pieces.queens,
            BLACK_KING => &mut self.black_pieces.kings,
        }
    }

    fn get_colors_mut(&mut self, color: Color) -> &mut BitBoard {
        match color {
            Color::White => &mut self.white_occupied,
            Color::Black => &mut self.black_occupied,
        }
    }

    fn clear_mask_by_piece(&mut self, mask: BitBoard, piece: Piece) {
        let clear_mask = !mask;
        *self.get_pieces_mut(piece) &= clear_mask;
        *self.get_colors_mut(piece.color) &= clear_mask;
        self.occupied &= clear_mask;
    }

    fn set_mask_by_piece(&mut self, mask: BitBoard, piece: Piece) {
        *self.get_pieces_mut(piece) |= mask;
        *self.get_colors_mut(piece.color) |= mask;
        self.occupied |= mask;
    }

    fn get_pawn_attacks(&self, square: Square, color: Color) -> BitBoard {
        match color {
            Color::White => WHITE_PAWN_ATTACKS[usize::from(square)],
            Color::Black => BLACK_PAWN_ATTACKS[usize::from(square)],
        }
    }

    fn get_pawn_moves(&self, square: Square, color: Color) -> BitBoard {
        const WHITE_PAWN_START: Row = Row::new(1);
        const BLACK_PAWN_START: Row = Row::new(6);
        let unoccupied = !self.occupied;
        match color {
            Color::White => {
                let mut output_mask = unoccupied & (BitBoard::from(square) << 8);
                if (square.get_row() == WHITE_PAWN_START) & output_mask.is_not_empty() {
                    output_mask |= (output_mask << 8) & unoccupied;
                }
                output_mask |= self.occupied & WHITE_PAWN_ATTACKS[usize::from(square)];
                output_mask
            }
            Color::Black => {
                let mut output_mask = unoccupied & (BitBoard::from(square) >> 8);
                if (square.get_row() == BLACK_PAWN_START) & output_mask.is_not_empty() {
                    output_mask |= (output_mask >> 8) & unoccupied;
                }
                output_mask |= self.occupied & BLACK_PAWN_ATTACKS[usize::from(square)];
                output_mask
            }
        }
    }

    fn get_ray_moves(&self, square: Square, rays: &BitBoardRayTable) -> BitBoard {
        let mut output_mask = EMPTY_BOARD;
        let ray_masks = rays[usize::from(square)];
        for dir in [
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ] {
            let mut ray_mask = ray_masks[dir as usize];
            let blocker = match dir {
                Direction::East | Direction::North => (ray_mask & self.occupied).bitscan_forward(),
                Direction::West | Direction::South => (ray_mask & self.occupied).bitscan_backward(),
            };
            if let Some(blocker) = blocker {
                ray_mask ^= rays[usize::from(blocker)][dir as usize];
            }
            output_mask |= ray_mask;
        }
        output_mask
    }

    fn get_knight_moves(&self, square: Square) -> BitBoard {
        KNIGHT_MOVES[usize::from(square)]
    }

    fn get_king_moves(&self, square: Square) -> BitBoard {
        KING_MOVES[usize::from(square)]
    }
}

#[cfg(test)]
mod tests;
