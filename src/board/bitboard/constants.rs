use super::BitBoard;
use crate::board::{Column, Row, Square};

pub const NOT_A_FILE: u64 = 0xfefefefefefefefe;
pub const NOT_AB_FILE: u64 = NOT_A_FILE & (NOT_A_FILE << 1);
pub const NOT_ABCD_FILE: u64 = NOT_AB_FILE & (NOT_AB_FILE << 2);
pub const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
pub const NOT_GH_FILE: u64 = NOT_H_FILE & (NOT_H_FILE >> 1);
pub const NOT_EFGH_FILE: u64 = NOT_GH_FILE & (NOT_GH_FILE >> 2);
pub const FULL_BOARD: BitBoard = BitBoard::new(u64::MAX);
pub const EMPTY_BOARD: BitBoard = BitBoard::new(0);

pub static ROWS: [BitBoard; 8] = [
    Row::new(0).to_bitboard(),
    Row::new(1).to_bitboard(),
    Row::new(2).to_bitboard(),
    Row::new(3).to_bitboard(),
    Row::new(4).to_bitboard(),
    Row::new(5).to_bitboard(),
    Row::new(6).to_bitboard(),
    Row::new(7).to_bitboard(),
];

pub static COLUMNS: [BitBoard; 8] = [
    Column::new(0).to_bitboard(),
    Column::new(1).to_bitboard(),
    Column::new(2).to_bitboard(),
    Column::new(3).to_bitboard(),
    Column::new(4).to_bitboard(),
    Column::new(5).to_bitboard(),
    Column::new(6).to_bitboard(),
    Column::new(7).to_bitboard(),
];

pub static SQUARES: [BitBoard; 64] = gen_sqs();

pub static WHITE_PAWN_ATTACKS: [BitBoard; 64] = gen_white_pawn_attacks();

pub static BLACK_PAWN_ATTACKS: [BitBoard; 64] = gen_black_pawn_attacks();

pub static STRAIGHT_RAYS: [[BitBoard; 4]; 64] = gen_straight_moves();

pub static DIAG_RAYS: [[BitBoard; 4]; 64] = gen_diag_moves();

pub static KNIGHT_MOVES: [BitBoard; 64] = gen_knight_moves();

pub static KING_MOVES: [BitBoard; 64] = gen_king_moves();

pub static STRAIGHT_SEGMENTS: [[BitBoard; 64]; 64] = gen_straight_segments();

pub static DIAG_SEGMENTS: [[BitBoard; 64]; 64] = gen_diag_segments();

const fn gen_sqs() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        array[counter as usize] = Square::new(counter).to_bitboard();
        counter += 1;
    }
    array
}

const fn gen_white_pawn_attacks() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).to_bitboard();
        array[counter as usize] = square.gen_white_pawn_mask();
        counter += 1;
    }
    array
}

const fn gen_black_pawn_attacks() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).to_bitboard();
        array[counter as usize] = square.gen_black_pawn_mask();
        counter += 1;
    }
    array
}

const fn gen_straight_moves() -> [[BitBoard; 4]; 64] {
    let mut array = [[EMPTY_BOARD; 4]; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).to_bitboard();
        array[counter as usize] = [
            square.gen_east_mask(),
            square.gen_north_mask(),
            square.gen_west_mask(),
            square.gen_south_mask(),
        ];
        counter += 1;
    }
    array
}

const fn gen_diag_moves() -> [[BitBoard; 4]; 64] {
    let mut array = [[EMPTY_BOARD; 4]; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).to_bitboard();
        array[counter as usize] = [
            square.gen_east_north_mask(),
            square.gen_north_west_mask(),
            square.gen_west_south_mask(),
            square.gen_south_east_mask(),
        ];
        counter += 1;
    }
    array
}

const fn gen_knight_moves() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).to_bitboard();
        array[counter as usize] = square.gen_knight_mask();
        counter += 1;
    }
    array
}

const fn gen_king_moves() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).to_bitboard();
        array[counter as usize] = square.gen_king_mask();
        counter += 1;
    }
    array
}

const fn gen_straight_segments() -> [[BitBoard; 64]; 64] {
    let array = [[EMPTY_BOARD; 64]; 64];
    array
}

const fn gen_diag_segments() -> [[BitBoard; 64]; 64] {
    let array = [[EMPTY_BOARD; 64]; 64];
    array
}
