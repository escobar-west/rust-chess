use crate::board::bitboard::BitBoard;

const fn from_row_idx(row_idx: u8) -> BitBoard {
    BitBoard::new(0xff << 8 * row_idx)
}

const fn from_col_idx(col_idx: u8) -> BitBoard {
    BitBoard::new(0x0101010101010101 << col_idx)
}

pub static ROWS: [BitBoard; 8] = [
    from_row_idx(0),
    from_row_idx(1),
    from_row_idx(2),
    from_row_idx(3),
    from_row_idx(4),
    from_row_idx(5),
    from_row_idx(6),
    from_row_idx(7),
];

pub static COLUMNS: [BitBoard; 8] = [
    from_col_idx(0),
    from_col_idx(1),
    from_col_idx(2),
    from_col_idx(3),
    from_col_idx(4),
    from_col_idx(5),
    from_col_idx(6),
    from_col_idx(7),
];

pub static SQUARES: [BitBoard; 64] = [
    BitBoard::new(1 << 0),
    BitBoard::new(1 << 1),
    BitBoard::new(1 << 2),
    BitBoard::new(1 << 3),
    BitBoard::new(1 << 4),
    BitBoard::new(1 << 5),
    BitBoard::new(1 << 6),
    BitBoard::new(1 << 7),
    BitBoard::new(1 << 8),
    BitBoard::new(1 << 9),
    BitBoard::new(1 << 10),
    BitBoard::new(1 << 11),
    BitBoard::new(1 << 12),
    BitBoard::new(1 << 13),
    BitBoard::new(1 << 14),
    BitBoard::new(1 << 15),
    BitBoard::new(1 << 16),
    BitBoard::new(1 << 17),
    BitBoard::new(1 << 18),
    BitBoard::new(1 << 19),
    BitBoard::new(1 << 20),
    BitBoard::new(1 << 21),
    BitBoard::new(1 << 22),
    BitBoard::new(1 << 23),
    BitBoard::new(1 << 24),
    BitBoard::new(1 << 25),
    BitBoard::new(1 << 26),
    BitBoard::new(1 << 27),
    BitBoard::new(1 << 28),
    BitBoard::new(1 << 29),
    BitBoard::new(1 << 30),
    BitBoard::new(1 << 31),
    BitBoard::new(1 << 32),
    BitBoard::new(1 << 33),
    BitBoard::new(1 << 34),
    BitBoard::new(1 << 35),
    BitBoard::new(1 << 36),
    BitBoard::new(1 << 37),
    BitBoard::new(1 << 38),
    BitBoard::new(1 << 39),
    BitBoard::new(1 << 40),
    BitBoard::new(1 << 41),
    BitBoard::new(1 << 42),
    BitBoard::new(1 << 43),
    BitBoard::new(1 << 44),
    BitBoard::new(1 << 45),
    BitBoard::new(1 << 46),
    BitBoard::new(1 << 47),
    BitBoard::new(1 << 48),
    BitBoard::new(1 << 49),
    BitBoard::new(1 << 50),
    BitBoard::new(1 << 51),
    BitBoard::new(1 << 52),
    BitBoard::new(1 << 53),
    BitBoard::new(1 << 54),
    BitBoard::new(1 << 55),
    BitBoard::new(1 << 56),
    BitBoard::new(1 << 57),
    BitBoard::new(1 << 58),
    BitBoard::new(1 << 59),
    BitBoard::new(1 << 60),
    BitBoard::new(1 << 61),
    BitBoard::new(1 << 62),
    BitBoard::new(1 << 63),
];

pub static KNIGHT_MOVES: [BitBoard; 64] = [
    SQUARES[0].knight_moves(),
    SQUARES[1].knight_moves(),
    SQUARES[2].knight_moves(),
    SQUARES[3].knight_moves(),
    SQUARES[4].knight_moves(),
    SQUARES[5].knight_moves(),
    SQUARES[6].knight_moves(),
    SQUARES[7].knight_moves(),
    SQUARES[8].knight_moves(),
    SQUARES[9].knight_moves(),
    SQUARES[10].knight_moves(),
    SQUARES[11].knight_moves(),
    SQUARES[12].knight_moves(),
    SQUARES[13].knight_moves(),
    SQUARES[14].knight_moves(),
    SQUARES[15].knight_moves(),
    SQUARES[16].knight_moves(),
    SQUARES[17].knight_moves(),
    SQUARES[18].knight_moves(),
    SQUARES[19].knight_moves(),
    SQUARES[20].knight_moves(),
    SQUARES[21].knight_moves(),
    SQUARES[22].knight_moves(),
    SQUARES[23].knight_moves(),
    SQUARES[24].knight_moves(),
    SQUARES[25].knight_moves(),
    SQUARES[26].knight_moves(),
    SQUARES[27].knight_moves(),
    SQUARES[28].knight_moves(),
    SQUARES[29].knight_moves(),
    SQUARES[30].knight_moves(),
    SQUARES[31].knight_moves(),
    SQUARES[32].knight_moves(),
    SQUARES[33].knight_moves(),
    SQUARES[34].knight_moves(),
    SQUARES[35].knight_moves(),
    SQUARES[36].knight_moves(),
    SQUARES[37].knight_moves(),
    SQUARES[38].knight_moves(),
    SQUARES[39].knight_moves(),
    SQUARES[40].knight_moves(),
    SQUARES[41].knight_moves(),
    SQUARES[42].knight_moves(),
    SQUARES[43].knight_moves(),
    SQUARES[44].knight_moves(),
    SQUARES[45].knight_moves(),
    SQUARES[46].knight_moves(),
    SQUARES[47].knight_moves(),
    SQUARES[48].knight_moves(),
    SQUARES[49].knight_moves(),
    SQUARES[50].knight_moves(),
    SQUARES[51].knight_moves(),
    SQUARES[52].knight_moves(),
    SQUARES[53].knight_moves(),
    SQUARES[54].knight_moves(),
    SQUARES[55].knight_moves(),
    SQUARES[56].knight_moves(),
    SQUARES[57].knight_moves(),
    SQUARES[58].knight_moves(),
    SQUARES[59].knight_moves(),
    SQUARES[60].knight_moves(),
    SQUARES[61].knight_moves(),
    SQUARES[62].knight_moves(),
    SQUARES[63].knight_moves(),
];

pub static KING_MOVES: [BitBoard; 64] = [
    SQUARES[0].king_moves(),
    SQUARES[1].king_moves(),
    SQUARES[2].king_moves(),
    SQUARES[3].king_moves(),
    SQUARES[4].king_moves(),
    SQUARES[5].king_moves(),
    SQUARES[6].king_moves(),
    SQUARES[7].king_moves(),
    SQUARES[8].king_moves(),
    SQUARES[9].king_moves(),
    SQUARES[10].king_moves(),
    SQUARES[11].king_moves(),
    SQUARES[12].king_moves(),
    SQUARES[13].king_moves(),
    SQUARES[14].king_moves(),
    SQUARES[15].king_moves(),
    SQUARES[16].king_moves(),
    SQUARES[17].king_moves(),
    SQUARES[18].king_moves(),
    SQUARES[19].king_moves(),
    SQUARES[20].king_moves(),
    SQUARES[21].king_moves(),
    SQUARES[22].king_moves(),
    SQUARES[23].king_moves(),
    SQUARES[24].king_moves(),
    SQUARES[25].king_moves(),
    SQUARES[26].king_moves(),
    SQUARES[27].king_moves(),
    SQUARES[28].king_moves(),
    SQUARES[29].king_moves(),
    SQUARES[30].king_moves(),
    SQUARES[31].king_moves(),
    SQUARES[32].king_moves(),
    SQUARES[33].king_moves(),
    SQUARES[34].king_moves(),
    SQUARES[35].king_moves(),
    SQUARES[36].king_moves(),
    SQUARES[37].king_moves(),
    SQUARES[38].king_moves(),
    SQUARES[39].king_moves(),
    SQUARES[40].king_moves(),
    SQUARES[41].king_moves(),
    SQUARES[42].king_moves(),
    SQUARES[43].king_moves(),
    SQUARES[44].king_moves(),
    SQUARES[45].king_moves(),
    SQUARES[46].king_moves(),
    SQUARES[47].king_moves(),
    SQUARES[48].king_moves(),
    SQUARES[49].king_moves(),
    SQUARES[50].king_moves(),
    SQUARES[51].king_moves(),
    SQUARES[52].king_moves(),
    SQUARES[53].king_moves(),
    SQUARES[54].king_moves(),
    SQUARES[55].king_moves(),
    SQUARES[56].king_moves(),
    SQUARES[57].king_moves(),
    SQUARES[58].king_moves(),
    SQUARES[59].king_moves(),
    SQUARES[60].king_moves(),
    SQUARES[61].king_moves(),
    SQUARES[62].king_moves(),
    SQUARES[63].king_moves(),
];