mod bitboard;
mod components;
mod mailbox;

use crate::pieces::{constants::*, Color, Figure, Piece};
pub use bitboard::{BitBoard, EMPTY_BOARD, FULL_BOARD};
use bitboard::{
    Direction, BLACK_PAWN_ATTACKS, DIAG_RAYS, DIAG_SEGMENTS, KING_MOVES, KNIGHT_MOVES, NOT_H_FILE,
    STRAIGHT_RAYS, STRAIGHT_SEGMENTS, WHITE_PAWN_ATTACKS,
};
pub use components::{Column, Row, Square};
use mailbox::MailBox;

type BitBoardRayTable = [[BitBoard; 4]; 64];

#[derive(Debug, Default, PartialEq, Eq)]
struct PieceSet {
    pawns: BitBoard,
    rooks: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Board {
    white_pieces: PieceSet,
    black_pieces: PieceSet,
    white_occupied: BitBoard,
    black_occupied: BitBoard,
    occupied: BitBoard,
    mailbox: MailBox,
}

impl Board {
    pub fn get_occupied(&self) -> BitBoard {
        self.occupied
    }

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

    pub fn get_color(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white_occupied,
            Color::Black => self.black_occupied,
        }
    }

    pub fn get_square(&self, square: Square) -> Option<Piece> {
        self.mailbox.get_square(square)
    }

    pub fn clear_square(&mut self, square: Square) -> Option<Piece> {
        let piece = self.mailbox.clear_square(square);
        if let Some(p) = piece {
            self.clear_bitboards(square.into(), p)
        }
        piece
    }

    pub fn set_square(&mut self, square: Square, piece: Piece) -> Option<Piece> {
        let old_piece = self.mailbox.set_square(square, piece);
        let square_mask: BitBoard = square.into();
        if let Some(old_piece) = old_piece {
            self.clear_bitboards(square_mask, old_piece)
        }
        self.set_bitboards(square_mask, piece);
        old_piece
    }

    pub fn move_piece(&mut self, from: Square, to: Square) -> Option<Piece> {
        self.clear_square(from).and_then(|p| self.set_square(to, p))
    }

    pub fn get_moves(&self, square: Square) -> BitBoard {
        let Some(piece) = self.get_square(square) else {
            return EMPTY_BOARD;
        };
        !self.get_color(piece.color)
            & match piece.figure {
                Figure::Pawn => self.get_pawn_moves(square, piece.color),
                Figure::Rook => self.get_straight_moves(square),
                Figure::Knight => square.get_knight_moves(),
                Figure::Bishop => self.get_diag_moves(square),
                Figure::Queen => self.get_straight_moves(square) | self.get_diag_moves(square),
                Figure::King => square.get_king_moves(),
            }
    }

    pub fn get_safe_squares(&self, player_square: Square, player_color: Color) -> BitBoard {
        let attack_color = !player_color;
        let attackers = self.get_piece_set(attack_color);
        let blockers = self.occupied & !BitBoard::from(player_square);
        let attack_mask = get_all_attacks_mask(attackers, attack_color, blockers);
        !attack_mask
    }

    pub fn get_straight_moves(&self, square: Square) -> BitBoard {
        get_blocked_rays(square, self.occupied, &STRAIGHT_RAYS)
    }

    pub fn get_diag_moves(&self, square: Square) -> BitBoard {
        get_blocked_rays(square, self.occupied, &DIAG_RAYS)
    }

    pub fn get_pin_mask(
        &self,
        pinned_sq: Square,
        target_sq: Square,
        target_color: Color,
    ) -> BitBoard {
        let pinned_sq = pinned_sq.as_bitboard();
        let attackers = self.get_piece_set(!target_color);
        let straight_attackers = attackers.rooks | attackers.queens;
        for attacker_sq in straight_attackers.iter_forward() {
            let segment = get_straight_segment(target_sq, attacker_sq);
            if segment & self.occupied == pinned_sq | attacker_sq.as_bitboard() {
                return segment;
            }
        }
        let diag_attackers = attackers.bishops | attackers.queens;
        for attacker_sq in diag_attackers.iter_forward() {
            let segment = get_diag_segment(target_sq, attacker_sq);
            if segment & self.occupied == pinned_sq | attacker_sq.as_bitboard() {
                return segment;
            }
        }
        FULL_BOARD
    }

    pub fn get_check_stops(&self, target_sq: Square, target_color: Color) -> BitBoard {
        let mut stop_moves = FULL_BOARD;
        let attackers = self.get_piece_set(!target_color);
        let straight_attackers = attackers.rooks | attackers.queens;
        for attacker_sq in straight_attackers.iter_forward() {
            let segment = get_straight_segment(target_sq, attacker_sq);
            if segment & self.occupied == attacker_sq.as_bitboard() {
                stop_moves &= segment;
            }
        }
        let diag_attackers = attackers.bishops | attackers.queens;
        for attacker_sq in diag_attackers.iter_forward() {
            let segment = get_diag_segment(target_sq, attacker_sq);
            if segment & self.occupied == attacker_sq.as_bitboard() {
                stop_moves &= segment;
            }
        }
        let knight_moves = target_sq.get_knight_moves();
        for attacker_sq in attackers.knights.iter_forward() {
            let attacker_bb = attacker_sq.as_bitboard();
            if knight_moves & attacker_bb == attacker_bb {
                stop_moves &= attacker_bb;
            }
        }
        let pawn_moves = self.get_pawn_attacks(target_sq, target_color);
        for attacker_sq in attackers.pawns.iter_forward() {
            let attacker_bb = attacker_sq.as_bitboard();
            if pawn_moves & attacker_bb == attacker_bb {
                stop_moves &= attacker_bb;
            }
        }
        stop_moves
    }

    pub fn is_attacked_by(&self, target_sq: Square, attack_color: Color) -> bool {
        let attackers = self.get_piece_set(attack_color);
        let straight_attackers = attackers.rooks | attackers.queens;
        for attacker_sq in straight_attackers.iter_forward() {
            let segment = get_straight_segment(attacker_sq, target_sq);
            if segment & self.occupied == target_sq.as_bitboard() {
                return true;
            }
        }
        let diag_attackers = attackers.bishops | attackers.queens;
        for attacker_sq in diag_attackers.iter_forward() {
            let segment = get_diag_segment(attacker_sq, target_sq);
            if segment & self.occupied == target_sq.as_bitboard() {
                return true;
            }
        }
        if (target_sq.get_knight_moves() & attackers.knights).is_not_empty() {
            return true;
        }
        if (self.get_pawn_attacks(target_sq, !attack_color) & attackers.pawns).is_not_empty() {
            return true;
        }
        if (target_sq.get_king_moves() & attackers.kings).is_not_empty() {
            return true;
        }
        false
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

    pub fn to_fen(&self) -> Box<str> {
        let mut fen_row_list: Vec<String> = Vec::with_capacity(8);
        for row_idx in (0..8u8).rev() {
            let mut none_count = 0u8;
            let mut fen_row: String = "".into();
            for col_idx in 0..8u8 {
                let square = Square::from_coords(Row::new(row_idx), Column::new(col_idx));
                match self.get_square(square) {
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
        fen_row_list.join("/").into_boxed_str()
    }

    pub fn print_board(&self) {
        let mut char_board: [char; 64] = ['☐'; 64];
        for piece in [
            WHITE_PAWN,
            WHITE_ROOK,
            WHITE_KNIGHT,
            WHITE_BISHOP,
            WHITE_QUEEN,
            WHITE_KING,
            BLACK_PAWN,
            BLACK_ROOK,
            BLACK_KNIGHT,
            BLACK_BISHOP,
            BLACK_QUEEN,
            BLACK_KING,
        ] {
            let c: char = piece.into();
            let bb = self.get_pieces(piece);
            for square in bb.iter_forward() {
                char_board[usize::from(square)] = c;
            }
        }
        let mut out_str = String::new();
        for i in (0..8).rev() {
            let offset = 8 * i as usize;
            let row: String = char_board[offset..offset + 8].iter().collect();
            out_str.push_str(&row);
            out_str.push('\n')
        }
        println!("{}", out_str);
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

    fn get_color_mut(&mut self, color: Color) -> &mut BitBoard {
        match color {
            Color::White => &mut self.white_occupied,
            Color::Black => &mut self.black_occupied,
        }
    }

    fn get_piece_set(&self, color: Color) -> &PieceSet {
        match color {
            Color::White => &self.white_pieces,
            Color::Black => &self.black_pieces,
        }
    }

    fn clear_bitboards(&mut self, mask: BitBoard, piece: Piece) {
        let clear_mask = !mask;
        *self.get_pieces_mut(piece) &= clear_mask;
        *self.get_color_mut(piece.color) &= clear_mask;
        self.occupied &= clear_mask;
    }

    fn set_bitboards(&mut self, mask: BitBoard, piece: Piece) {
        *self.get_pieces_mut(piece) |= mask;
        *self.get_color_mut(piece.color) |= mask;
        self.occupied |= mask;
    }

    fn get_pawn_attacks(&self, square: Square, color: Color) -> BitBoard {
        match color {
            Color::White => WHITE_PAWN_ATTACKS[usize::from(square)],
            Color::Black => BLACK_PAWN_ATTACKS[usize::from(square)],
        }
    }

    fn get_pawn_moves(&self, square: Square, color: Color) -> BitBoard {
        match color {
            Color::White => {
                let mut output_mask = !self.occupied & (BitBoard::from(square) << 8);
                output_mask |= self.occupied & WHITE_PAWN_ATTACKS[usize::from(square)];
                output_mask
            }
            Color::Black => {
                let mut output_mask = !self.occupied & (BitBoard::from(square) >> 8);
                output_mask |= self.occupied & BLACK_PAWN_ATTACKS[usize::from(square)];
                output_mask
            }
        }
    }
}

fn get_straight_segment(from: Square, to: Square) -> BitBoard {
    STRAIGHT_SEGMENTS[from.as_usize()][to.as_usize()]
}

fn get_diag_segment(from: Square, to: Square) -> BitBoard {
    DIAG_SEGMENTS[from.as_usize()][to.as_usize()]
}

fn get_blocked_rays(square: Square, blockers: BitBoard, ray_table: &BitBoardRayTable) -> BitBoard {
    let mut attack_mask = EMPTY_BOARD;
    let ray_masks = ray_table[usize::from(square)];
    for dir in [
        Direction::East,
        Direction::North,
        Direction::West,
        Direction::South,
    ] {
        let mut ray_mask = ray_masks[dir as usize];
        let blocker = match dir {
            Direction::East | Direction::North => (ray_mask & blockers).bitscan_forward(),
            Direction::West | Direction::South => (ray_mask & blockers).bitscan_backward(),
        };
        if let Some(blocker) = blocker {
            ray_mask ^= ray_table[usize::from(blocker)][dir as usize];
        }
        attack_mask |= ray_mask;
    }
    attack_mask
}

fn get_all_attacks_mask(attackers: &PieceSet, attack_color: Color, blockers: BitBoard) -> BitBoard {
    let mut attack_mask = EMPTY_BOARD;
    let straight_pieces = attackers.rooks | attackers.queens;
    for attack_sq in straight_pieces.iter_forward() {
        attack_mask |= get_blocked_rays(attack_sq, blockers, &STRAIGHT_RAYS);
    }
    let diag_pieces = attackers.bishops | attackers.queens;
    for attack_sq in diag_pieces.iter_forward() {
        attack_mask |= get_blocked_rays(attack_sq, blockers, &DIAG_RAYS);
    }
    for attack_sq in attackers.knights.iter_forward() {
        attack_mask |= attack_sq.get_knight_moves()
    }
    for attack_sq in attackers.kings.iter_forward() {
        attack_mask |= attack_sq.get_king_moves()
    }
    attack_mask |= match attack_color {
        Color::White => attackers.pawns.gen_white_pawn_mask(),
        Color::Black => attackers.pawns.gen_black_pawn_mask(),
    };
    attack_mask
}

#[cfg(test)]
mod tests;
