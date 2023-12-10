use super::BitBoard;

pub const NOT_A_FILE: u64 = 0xfefefefefefefefe;
pub const NOT_AB_FILE: u64 = NOT_A_FILE & (NOT_A_FILE << 1);
pub const NOT_ABCD_FILE: u64 = NOT_AB_FILE & (NOT_AB_FILE << 2);
pub const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
pub const NOT_GH_FILE: u64 = NOT_H_FILE & (NOT_H_FILE >> 1);
pub const NOT_EFGH_FILE: u64 = NOT_GH_FILE & (NOT_GH_FILE >> 2);
pub const DEBRUIJN64: u64 = 0x03f79d71b4cb0a89;
pub const FULL_BOARD: BitBoard = BitBoard::new(u64::MAX);
pub const EMPTY_BOARD: BitBoard = BitBoard::new(0);

#[rustfmt::skip]
pub static FWDSCAN: [u8; 64] = [
     0,  1, 48,  2, 57, 49, 28,  3,
    61, 58, 50, 42, 38, 29, 17,  4,
    62, 55, 59, 36, 53, 51, 43, 22,
    45, 39, 33, 30, 24, 18, 12,  5,
    63, 47, 56, 27, 60, 41, 37, 16,
    54, 35, 52, 21, 44, 32, 23, 11,
    46, 26, 40, 15, 34, 20, 31, 10,
    25, 14, 19,  9, 13,  8,  7,  6
];

#[rustfmt::skip]
pub static BACKSCAN: [u8; 64] = [
     0, 47,  1, 56, 48, 27,  2, 60,
    57, 49, 41, 37, 28, 16,  3, 61,
    54, 58, 35, 52, 50, 42, 21, 44,
    38, 32, 29, 23, 17, 11,  4, 62,
    46, 55, 26, 59, 40, 36, 15, 53,
    34, 51, 20, 43, 31, 22, 10, 45,
    25, 39, 14, 33, 19, 30,  9, 24,
    13, 18,  8, 12,  7,  6,  5, 63
];

pub static ROWS: [BitBoard; 8] = [
    BitBoard::gen_row_mask(0),
    BitBoard::gen_row_mask(1),
    BitBoard::gen_row_mask(2),
    BitBoard::gen_row_mask(3),
    BitBoard::gen_row_mask(4),
    BitBoard::gen_row_mask(5),
    BitBoard::gen_row_mask(6),
    BitBoard::gen_row_mask(7),
];

pub static COLUMNS: [BitBoard; 8] = [
    BitBoard::gen_col_mask(0),
    BitBoard::gen_col_mask(1),
    BitBoard::gen_col_mask(2),
    BitBoard::gen_col_mask(3),
    BitBoard::gen_col_mask(4),
    BitBoard::gen_col_mask(5),
    BitBoard::gen_col_mask(6),
    BitBoard::gen_col_mask(7),
];

pub static SQUARES: [BitBoard; 64] = gen_squares();

pub static WHITE_PAWN_ATTACKS: [BitBoard; 64] = gen_white_pawn_attacks();

pub static BLACK_PAWN_ATTACKS: [BitBoard; 64] = gen_black_pawn_attacks();

pub static STRAIGHT_MOVES: [[BitBoard; 4]; 64] = gen_straight_moves();

pub static DIAG_MOVES: [[BitBoard; 4]; 64] = gen_diag_moves();

pub static KNIGHT_MOVES: [BitBoard; 64] = gen_knight_moves();

pub static KING_MOVES: [BitBoard; 64] = gen_king_moves();

const fn gen_squares() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        array[counter as usize] = BitBoard::gen_square_mask(counter);
        counter += 1;
    }
    array
}

const fn gen_white_pawn_attacks() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = BitBoard::gen_square_mask(counter);
        array[counter as usize] = square.gen_white_pawn_mask();
        counter += 1;
    }
    array
}

const fn gen_black_pawn_attacks() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = BitBoard::gen_square_mask(counter);
        array[counter as usize] = square.gen_black_pawn_mask();
        counter += 1;
    }
    array
}

const fn gen_straight_moves() -> [[BitBoard; 4]; 64] {
    let mut array = [[EMPTY_BOARD; 4]; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = BitBoard::gen_square_mask(counter);
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
        let square = BitBoard::gen_square_mask(counter);
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
        let square = BitBoard::gen_square_mask(counter);
        array[counter as usize] = square.gen_knight_mask();
        counter += 1;
    }
    array
}

const fn gen_king_moves() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = BitBoard::gen_square_mask(counter);
        array[counter as usize] = square.gen_king_mask();
        counter += 1;
    }
    array
}
