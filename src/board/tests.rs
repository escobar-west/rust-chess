use super::*;
use crate::gamestate::DEFAULT_FEN;

fn validate_board(board: Board) {
    assert!(board.white_occupied ^ board.black_occupied == board.occupied);
    assert!(board.white_occupied & board.black_occupied == EMPTY_BOARD);
    assert!(
        board.white_pieces.pawns
            ^ board.white_pieces.rooks
            ^ board.white_pieces.knights
            ^ board.white_pieces.bishops
            ^ board.white_pieces.queens
            ^ board.white_pieces.kings
            == board.white_occupied
    );
    for square in board.white_occupied.iter_forward() {
        assert_eq!(board.mailbox.get_sq(square).unwrap().color, Color::White);
    }
    for square in board.black_occupied.iter_forward() {
        assert_eq!(board.mailbox.get_sq(square).unwrap().color, Color::Black);
    }
    for square in board.white_pieces.pawns.iter_forward() {
        assert_eq!(board.mailbox.get_sq(square).unwrap(), WHITE_PAWN);
    }
    for square in board.white_pieces.rooks.iter_forward() {
        assert_eq!(board.mailbox.get_sq(square).unwrap(), WHITE_ROOK);
    }
    for square in board.white_pieces.knights.iter_forward() {
        assert_eq!(board.mailbox.get_sq(square).unwrap(), WHITE_KNIGHT);
    }
    for square in board.white_pieces.bishops.iter_forward() {
        assert_eq!(board.mailbox.get_sq(square).unwrap(), WHITE_BISHOP);
    }
    for square in board.white_pieces.queens.iter_forward() {
        assert_eq!(board.mailbox.get_sq(square).unwrap(), WHITE_QUEEN);
    }
    for square in board.white_pieces.kings.iter_forward() {
        assert_eq!(board.mailbox.get_sq(square).unwrap(), WHITE_KING);
    }
    for square in (!board.occupied).iter_forward() {
        assert!(board.mailbox.get_sq(square).is_none());
    }
}

#[test]
fn test_default_fen() {
    let fen = DEFAULT_FEN;
    let board = Board::try_from_fen(fen).unwrap();

    let pawn_mask = BitBoard::from(Row::new(1)) | BitBoard::from(Row::new(6));
    assert_eq!(
        board.white_pieces.pawns | board.black_pieces.pawns,
        pawn_mask
    );

    let rook_mask = BitBoard::from(Square::new(0))
        | BitBoard::from(Square::new(7))
        | BitBoard::from(Square::new(56))
        | BitBoard::from(Square::new(63));
    assert_eq!(
        board.white_pieces.rooks | board.black_pieces.rooks,
        rook_mask
    );

    let knight_mask = BitBoard::from(Square::new(1))
        | BitBoard::from(Square::new(6))
        | BitBoard::from(Square::new(57))
        | BitBoard::from(Square::new(62));
    assert_eq!(
        board.white_pieces.knights | board.black_pieces.knights,
        knight_mask
    );

    let bishop_mask = BitBoard::from(Square::new(2))
        | BitBoard::from(Square::new(5))
        | BitBoard::from(Square::new(58))
        | BitBoard::from(Square::new(61));
    assert_eq!(
        board.white_pieces.bishops | board.black_pieces.bishops,
        bishop_mask
    );

    let queen_mask = BitBoard::from(Square::new(3)) | BitBoard::from(Square::new(59));
    assert_eq!(
        board.white_pieces.queens | board.black_pieces.queens,
        queen_mask
    );

    let king_mask = BitBoard::from(Square::new(4)) | BitBoard::from(Square::new(60));
    assert_eq!(
        board.white_pieces.kings | board.black_pieces.kings,
        king_mask
    );

    let white_mask = BitBoard::from(Row::new(0)) | BitBoard::from(Row::new(1));
    assert_eq!(board.white_occupied, white_mask);

    let black_mask = BitBoard::from(Row::new(6)) | BitBoard::from(Row::new(7));
    assert_eq!(board.black_occupied, black_mask);

    let occ_mask = white_mask | black_mask;
    assert_eq!(board.occupied, occ_mask);

    let to_fen = board.to_fen();
    assert_eq!(to_fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
}

#[test]
fn test_clear_and_set_sq() {
    let fen = DEFAULT_FEN;
    let mut board = Board::try_from_fen(fen).unwrap();
    board.clear_sq(Square::new(56));

    let rook_mask = BitBoard::from(Square::new(0))
        | BitBoard::from(Square::new(7))
        | BitBoard::from(Square::new(63));
    assert_eq!(
        board.white_pieces.rooks | board.black_pieces.rooks,
        rook_mask
    );

    let piece = board.set_sq(
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
    assert_eq!(
        board.white_pieces.rooks | board.black_pieces.rooks,
        rook_mask
    );

    let queen_mask = BitBoard::from(Square::new(3))
        | BitBoard::from(Square::new(7))
        | BitBoard::from(Square::new(59));
    assert_eq!(
        board.white_pieces.queens | board.black_pieces.queens,
        queen_mask
    );

    let white_mask = (BitBoard::from(Row::new(0)) | BitBoard::from(Row::new(1)))
        ^ BitBoard::from(Square::new(7));
    assert_eq!(board.white_occupied, white_mask);

    let black_mask = (BitBoard::from(Row::new(6))
        | BitBoard::from(Row::new(7))
        | BitBoard::from(Square::new(7)))
        ^ BitBoard::from(Square::new(56));
    assert_eq!(board.black_occupied, black_mask);
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
    validate_board(board);
}

#[test]
fn test_pawn_moves() {
    let fen = "4k3/4p3/5pp1/7Q/8/B1r2P2/1P2P3/4K3 w - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    // b2
    let pawn_sq = Square::try_from_notation("b2").unwrap();
    let pawn_moves = board.get_move_mask(
        pawn_sq,
        Piece {
            color: Color::White,
            figure: Figure::Pawn,
        },
    );
    assert_eq!(
        pawn_moves,
        BitBoard::from(Square::new(17))
            | BitBoard::from(Square::new(18))
            | BitBoard::from(Square::new(25))
    );
    // e2
    let pawn_sq = Square::try_from_notation("e2").unwrap();
    let pawn_moves = board.get_move_mask(
        pawn_sq,
        Piece {
            color: Color::White,
            figure: Figure::Pawn,
        },
    );
    assert_eq!(
        pawn_moves,
        BitBoard::from(Square::new(20)) | BitBoard::from(Square::new(28))
    );
    // f3
    let pawn_sq = Square::try_from_notation("f3").unwrap();
    let pawn_moves = board.get_move_mask(
        pawn_sq,
        Piece {
            color: Color::White,
            figure: Figure::Pawn,
        },
    );
    assert_eq!(pawn_moves, BitBoard::from(Square::new(29)));
    // e7
    let pawn_sq = Square::try_from_notation("e7").unwrap();
    let pawn_moves = board.get_move_mask(
        pawn_sq,
        Piece {
            color: Color::Black,
            figure: Figure::Pawn,
        },
    );
    assert_eq!(
        pawn_moves,
        BitBoard::from(Square::new(36)) | BitBoard::from(Square::new(44))
    );
    // f6
    let pawn_sq = Square::try_from_notation("f6").unwrap();
    let pawn_moves = board.get_move_mask(
        pawn_sq,
        Piece {
            color: Color::Black,
            figure: Figure::Pawn,
        },
    );
    assert_eq!(pawn_moves, BitBoard::from(Square::new(37)));
}

#[test]
fn test_rook_moves() {
    let fen = "8/8/3r4/1R1R1R2/8/3R4/8/8 w - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let rook_sq = Square::try_from_notation("d5").unwrap();

    let rook_moves = board.get_move_mask(
        rook_sq,
        Piece {
            color: Color::White,
            figure: Figure::Rook,
        },
    );
    assert_eq!(
        rook_moves,
        ((STRAIGHT_MOVES[33][Direction::East as usize]
            & STRAIGHT_MOVES[37][Direction::West as usize])
            | (STRAIGHT_MOVES[19][Direction::North as usize]
                & STRAIGHT_MOVES[51][Direction::South as usize]))
            ^ rook_sq.into()
    );
}

#[test]
fn test_straight_pin_east() {
    let fen = "8/8/6k1/K3B2r/2q5/8/8/8 w - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(36);
    let king_sq = Square::try_from_notation("a5").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::White);
    let expected = BitBoard::from(Row::new(4)) ^ Square::new(32).into();
    assert_eq!(pin_mask, expected);
}

#[test]
fn test_straight_no_pin_east() {
    let fen = "8/8/6k1/K2pB2r/2q5/8/8/8 w - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(36);
    let king_sq = Square::try_from_notation("a5").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::White);
    let expected = FULL_BOARD;
    assert_eq!(pin_mask, expected);
}

#[test]
fn test_straight_pin_east_north() {
    let fen = "k6q/8/8/4B3/8/8/1K6/8 w - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(36);
    let king_sq = Square::try_from_notation("b2").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::White);
    let expected = DIAG_MOVES[9][Direction::East as usize];
    assert_eq!(pin_mask, expected);
}

#[test]
fn test_straight_pin_north() {
    let fen = "qP6/1r6/1P6/1r6/NP6/1r6/1P6/K6k w - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(24);
    let king_sq = Square::try_from_notation("a1").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::White);
    let expected = BitBoard::from(Column::new(0)) ^ Square::new(0).into();
    assert_eq!(pin_mask, expected);
}

#[test]
fn test_straight_pin_north_west() {
    let fen = "8/8/2q5/3R4/8/5K2/8/8 w - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(35);
    let king_sq = Square::try_from_notation("f3").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::White);
    let expected =
        DIAG_MOVES[21][Direction::North as usize] ^ DIAG_MOVES[42][Direction::North as usize];
    assert_eq!(pin_mask, expected);
}

#[test]
fn test_straight_pin_west() {
    let fen = "4R1nk/8/8/8/8/8/8/K7 w - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(62);
    let king_sq = Square::try_from_notation("h8").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::Black);
    let expected = BitBoard::from(pin_sq) | Square::new(61).into() | Square::new(60).into();
    assert_eq!(pin_mask, expected);
}

#[test]
fn test_straight_pin_west_south() {
    let fen = "5k2/8/8/8/1q6/B7/8/7K b - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(25);
    let king_sq = Square::try_from_notation("f8").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::Black);
    let expected = DIAG_MOVES[61][Direction::West as usize];
    assert_eq!(pin_mask, expected);
}

#[test]
fn test_straight_pin_south() {
    let fen = "5NNn/5Nkn/5RqR/5pRp/5ppp/8/8/K7 b - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(46);
    let king_sq = Square::try_from_notation("g7").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::Black);
    let expected = BitBoard::from(pin_sq) | Square::new(38).into();
    assert_eq!(pin_mask, expected);
}

#[test]
fn test_straight_pin_south_east() {
    let fen = "8/1k6/8/8/4p3/5Q2/6K1/8 b - - 0 1";
    let board = Board::try_from_fen(fen).unwrap();
    let pin_sq = Square::new(28);
    let king_sq = Square::try_from_notation("b7").unwrap();
    let pin_mask = board.get_pin_mask(pin_sq, king_sq, Color::Black);
    let expected =
        DIAG_MOVES[49][Direction::South as usize] ^ DIAG_MOVES[21][Direction::South as usize];
    assert_eq!(pin_mask, expected);
}
