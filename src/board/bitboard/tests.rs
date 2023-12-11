use super::*;
use crate::board::{Column, Row};
#[test]
fn test_straight_moves() {
    let east_moves = BitBoard::from(Square::new(9)).gen_east_mask();
    let expected = BitBoard::from(Row::new(1))
        ^ BitBoard::from(Square::new(8))
        ^ BitBoard::from(Square::new(9));
    assert_eq!(east_moves, expected);

    let north_moves = BitBoard::from(Square::new(9)).gen_north_mask();
    let expected = BitBoard::from(Column::new(1))
        ^ BitBoard::from(Square::new(1))
        ^ BitBoard::from(Square::new(9));
    assert_eq!(north_moves, expected);

    let west_moves = BitBoard::from(Square::new(9)).gen_west_mask();
    let expected = BitBoard::from(Square::new(8));
    assert_eq!(west_moves, expected);

    let south_moves = BitBoard::from(Square::new(9)).gen_south_mask();
    let expected = BitBoard::from(Square::new(1));
    assert_eq!(south_moves, expected);
}

#[test]
fn test_knight_moves() {
    let knight_moves = BitBoard::from(Square::new(0)).gen_knight_mask();
    let expected = BitBoard::from(Square::new(10)) | BitBoard::from(Square::new(17));
    assert_eq!(knight_moves, expected);

    let knight_moves = BitBoard::from(Square::new(45)).gen_knight_mask();
    let expected = BitBoard::from(Square::new(28))
        | BitBoard::from(Square::new(30))
        | BitBoard::from(Square::new(35))
        | BitBoard::from(Square::new(39))
        | BitBoard::from(Square::new(51))
        | BitBoard::from(Square::new(55))
        | BitBoard::from(Square::new(60))
        | BitBoard::from(Square::new(62));
    assert_eq!(knight_moves, expected);
}

#[test]
fn test_king_moves() {
    let king_moves = BitBoard::from(Square::new(0)).gen_king_mask();
    let expected = BitBoard::from(Square::new(1))
        | BitBoard::from(Square::new(8))
        | BitBoard::from(Square::new(9));
    assert_eq!(king_moves, expected);

    let king_moves = BitBoard::from(Square::new(54)).gen_king_mask();
    let expected = BitBoard::from(Square::new(45))
        | BitBoard::from(Square::new(46))
        | BitBoard::from(Square::new(47))
        | BitBoard::from(Square::new(53))
        | BitBoard::from(Square::new(55))
        | BitBoard::from(Square::new(61))
        | BitBoard::from(Square::new(62))
        | BitBoard::from(Square::new(63));
    assert_eq!(king_moves, expected);
}

#[test]
fn test_forward_bitscan() {
    let bitboard = BitBoard::from(Row::new(0));
    let lsb = bitboard.bitscan_forward().unwrap();
    assert_eq!(lsb, Square::new(0));

    let bitboard = BitBoard::from(Row::new(7));
    let lsb = bitboard.bitscan_forward().unwrap();
    assert_eq!(lsb, Square::new(56));

    let bitboard = BitBoard::from(Column::new(0));
    let lsb = bitboard.bitscan_forward().unwrap();
    assert_eq!(lsb, Square::new(0));

    let bitboard = BitBoard::from(Column::new(7));
    let lsb = bitboard.bitscan_forward().unwrap();
    assert_eq!(lsb, Square::new(7));

    let bitboard = BitBoard::from(Square::new(0));
    let lsb = bitboard.bitscan_forward().unwrap();
    assert_eq!(lsb, Square::new(0));

    let bitboard = BitBoard::from(Square::new(63));
    let lsb = bitboard.bitscan_forward().unwrap();
    assert_eq!(lsb, Square::new(63));

    let bitboard = FULL_BOARD;
    let lsb = bitboard.bitscan_forward().unwrap();
    assert_eq!(lsb, Square::new(0));
}

#[test]
fn test_backward_bitscan() {
    let bitboard = BitBoard::from(Row::new(0));
    let lsb = bitboard.bitscan_backward().unwrap();
    assert_eq!(lsb, Square::new(7));

    let bitboard = BitBoard::from(Row::new(7));
    let lsb = bitboard.bitscan_backward().unwrap();
    assert_eq!(lsb, Square::new(63));

    let bitboard = BitBoard::from(Column::new(0));
    let lsb = bitboard.bitscan_backward().unwrap();
    assert_eq!(lsb, Square::new(56));

    let bitboard = BitBoard::from(Column::new(7));
    let lsb = bitboard.bitscan_backward().unwrap();
    assert_eq!(lsb, Square::new(63));

    let bitboard = BitBoard::from(Square::new(0));
    let lsb = bitboard.bitscan_backward().unwrap();
    assert_eq!(lsb, Square::new(0));

    let bitboard = BitBoard::from(Square::new(63));
    let lsb = bitboard.bitscan_backward().unwrap();
    assert_eq!(lsb, Square::new(63));

    let bitboard = FULL_BOARD;
    let lsb = bitboard.bitscan_backward().unwrap();
    assert_eq!(lsb, Square::new(63));
}

impl BitBoard {
    pub fn from_alg(coords: &str) -> Self {
        BitBoard::from(Square::from_alg(coords))
    }
}
