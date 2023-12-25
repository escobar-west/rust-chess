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
    Row::new(0).as_bitboard(),
    Row::new(1).as_bitboard(),
    Row::new(2).as_bitboard(),
    Row::new(3).as_bitboard(),
    Row::new(4).as_bitboard(),
    Row::new(5).as_bitboard(),
    Row::new(6).as_bitboard(),
    Row::new(7).as_bitboard(),
];

pub static COLUMNS: [BitBoard; 8] = [
    Column::new(0).as_bitboard(),
    Column::new(1).as_bitboard(),
    Column::new(2).as_bitboard(),
    Column::new(3).as_bitboard(),
    Column::new(4).as_bitboard(),
    Column::new(5).as_bitboard(),
    Column::new(6).as_bitboard(),
    Column::new(7).as_bitboard(),
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
        array[counter as usize] = Square::new(counter).as_bitboard();
        counter += 1;
    }
    array
}

const fn gen_white_pawn_attacks() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).as_bitboard();
        array[counter as usize] = square.gen_white_pawn_mask();
        counter += 1;
    }
    array
}

const fn gen_black_pawn_attacks() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).as_bitboard();
        array[counter as usize] = square.gen_black_pawn_mask();
        counter += 1;
    }
    array
}

const fn gen_straight_moves() -> [[BitBoard; 4]; 64] {
    let mut array = [[EMPTY_BOARD; 4]; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).as_bitboard();
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
        let square = Square::new(counter).as_bitboard();
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
        let square = Square::new(counter).as_bitboard();
        array[counter as usize] = square.gen_knight_mask();
        counter += 1;
    }
    array
}

const fn gen_king_moves() -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let mut counter = 0;
    while counter < 64 {
        let square = Square::new(counter).as_bitboard();
        array[counter as usize] = square.gen_king_mask();
        counter += 1;
    }
    array
}

const fn gen_straight_segments() -> [[BitBoard; 64]; 64] {
    let mut array = [[EMPTY_BOARD; 64]; 64];
    let mut square_counter: u8 = 0;
    while square_counter < 64 {
        let from_square = Square::new(square_counter);
        array[from_square.as_usize()] = gen_straight_segments_from_square(from_square);
        square_counter += 1;
    }
    array
}

const fn gen_diag_segments() -> [[BitBoard; 64]; 64] {
    let mut array = [[EMPTY_BOARD; 64]; 64];
    let mut square_counter: u8 = 0;
    while square_counter < 64 {
        let from_square = Square::new(square_counter);
        array[from_square.as_usize()] = gen_diag_segments_from_square(from_square);
        square_counter += 1;
    }
    array
}

const fn gen_straight_segments_from_square(from_square: Square) -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let from_row = from_square.get_row();
    let from_col = from_square.get_col();
    let mut row_counter = 0;
    while row_counter < from_row.as_u8() {
        let to_square = Square::from_coords(Row::new(row_counter), from_col);
        let segment = from_square.as_bitboard().gen_south_mask().as_u64()
            ^ to_square.as_bitboard().gen_south_mask().as_u64();
        array[to_square.as_usize()] = BitBoard(segment);
        row_counter += 1;
    }
    row_counter = from_row.as_u8() + 1;
    while row_counter < 8 {
        let to_square = Square::from_coords(Row::new(row_counter), from_col);
        let segment = from_square.as_bitboard().gen_north_mask().as_u64()
            ^ to_square.as_bitboard().gen_north_mask().as_u64();
        array[to_square.as_usize()] = BitBoard(segment);
        row_counter += 1;
    }
    let mut col_counter = 0;
    while col_counter < from_col.as_u8() {
        let to_square = Square::from_coords(from_row, Column::new(col_counter));
        let segment = from_square.as_bitboard().gen_west_mask().as_u64()
            ^ to_square.as_bitboard().gen_west_mask().as_u64();
        array[to_square.as_usize()] = BitBoard(segment);
        col_counter += 1;
    }
    col_counter = from_col.as_u8() + 1;
    while col_counter < 8 {
        let to_square = Square::from_coords(from_row, Column::new(col_counter));
        let segment = from_square.as_bitboard().gen_east_mask().as_u64()
            ^ to_square.as_bitboard().gen_east_mask().as_u64();
        array[to_square.as_usize()] = BitBoard(segment);
        col_counter += 1;
    }
    array
}

const fn gen_diag_segments_from_square(from_square: Square) -> [BitBoard; 64] {
    let mut array = [EMPTY_BOARD; 64];
    let from_row = from_square.get_row();
    let from_col = from_square.get_col();
    let east_north_limit = if from_col.as_u8() < from_row.as_u8() {
        8 - from_row.as_u8()
    } else {
        8 - from_col.as_u8()
    };
    let mut east_north_counter = 1;
    while east_north_counter < east_north_limit {
        let to_row = Row::new(from_row.as_u8() + east_north_counter);
        let to_col = Column::new(from_col.as_u8() + east_north_counter);
        let to_square = Square::from_coords(to_row, to_col);
        let segment = from_square.as_bitboard().gen_east_north_mask().as_u64()
            ^ to_square.as_bitboard().gen_east_north_mask().as_u64();
        array[to_square.as_usize()] = BitBoard(segment);
        east_north_counter += 1;
    }
    let north_west_limit = if 7 - from_col.as_u8() < from_row.as_u8() {
        8 - from_row.as_u8()
    } else {
        from_col.as_u8() + 1
    };
    let mut north_west_counter = 1;
    while north_west_counter < north_west_limit {
        let to_row = Row::new(from_row.as_u8() + north_west_counter);
        let to_col = Column::new(from_col.as_u8() - north_west_counter);
        let to_square = Square::from_coords(to_row, to_col);
        let segment = from_square.as_bitboard().gen_north_west_mask().as_u64()
            ^ to_square.as_bitboard().gen_north_west_mask().as_u64();
        array[to_square.as_usize()] = BitBoard(segment);
        north_west_counter += 1;
    }
    let west_south_limit = if from_col.as_u8() < from_row.as_u8() {
        from_col.as_u8() + 1
    } else {
        from_row.as_u8() + 1
    };
    let mut west_south_counter = 1;
    while west_south_counter < west_south_limit {
        let to_row = Row::new(from_row.as_u8() - west_south_counter);
        let to_col = Column::new(from_col.as_u8() - west_south_counter);
        let to_square = Square::from_coords(to_row, to_col);
        let segment = from_square.as_bitboard().gen_west_south_mask().as_u64()
            ^ to_square.as_bitboard().gen_west_south_mask().as_u64();
        array[to_square.as_usize()] = BitBoard(segment);
        west_south_counter += 1;
    }
    let south_east_limit = if from_col.as_u8() < 7 - from_row.as_u8() {
        from_row.as_u8() + 1
    } else {
        8 - from_col.as_u8()
    };
    let mut south_east_counter = 1;
    while south_east_counter < south_east_limit {
        let to_row = Row::new(from_row.as_u8() - south_east_counter);
        let to_col = Column::new(from_col.as_u8() + south_east_counter);
        let to_square = Square::from_coords(to_row, to_col);
        let segment = from_square.as_bitboard().gen_south_east_mask().as_u64()
            ^ to_square.as_bitboard().gen_south_east_mask().as_u64();
        array[to_square.as_usize()] = BitBoard(segment);
        south_east_counter += 1;
    }
    array
}
