mod moves;
use crate::{
    board::{BitBoard, Board, Square, EMPTY_BOARD},
    pieces::{Color, Figure, Piece},
};
use moves::Move;

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CastleRights(u8);

impl CastleRights {
    pub fn new(wk: bool, wq: bool, bk: bool, bq: bool) -> Self {
        Self(u8::from(wk) + 2 * u8::from(wq) + 4 * u8::from(bk) + 8 * u8::from(bq))
    }

    fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
        let mut wk = false;
        let mut wq = false;
        let mut bk = false;
        let mut bq = false;
        for c in fen.chars() {
            match c {
                '-' => return Ok(Self(0)),
                'K' => wk = true,
                'Q' => wq = true,
                'k' => bk = true,
                'q' => bq = true,
                _ => return Err("invalid char"),
            }
        }
        Ok(Self::new(wk, wq, bk, bq))
    }

    fn to_fen(&self) -> String {
        let fen = match self.0 {
            0 => "-",
            1 => "K",
            2 => "Q",
            3 => "KQ",
            4 => "k",
            5 => "Kk",
            6 => "Qk",
            7 => "KQk",
            8 => "q",
            9 => "Kq",
            10 => "Qq",
            11 => "KQq",
            12 => "kq",
            13 => "Kkq",
            14 => "Qkq",
            15 => "KQkq",
            _ => panic!(),
        };
        fen.into()
    }
}

pub struct GameState {
    board: Board,
    turn: Color,
    castle: CastleRights,
    ep: Option<Square>,
    halfmoves: u32,
    fullmoves: u32,
}

impl GameState {
    pub fn get_legal_moves_at_square(&self, square: Square) -> BitBoard {
        let board = &self.board;
        let Some(piece) = board.get_square(square) else {
            return EMPTY_BOARD;
        };
        if piece.color != self.turn {
            return EMPTY_BOARD;
        }
        let move_mask = board.get_move_mask(square, piece);
        let pin_mask = board.get_pin_mask(square, piece.color);
        move_mask & pin_mask
    }

    pub fn make_move(&mut self, move_: Move) -> Result<Option<Piece>, &'static str> {
        let Move { from, to } = move_;
        let legal_moves = self.get_legal_moves_at_square(from);
        if (legal_moves & to.into()).is_empty() {
            return Err("Illegal Move");
        }
        let captured_piece = self.board.move_piece(from, to);
        if captured_piece.is_some() {
            self.halfmoves = 0;
        } else {
            self.halfmoves += 1;
        }
        if self.turn == Color::Black {
            self.fullmoves += 1;
        }
        self.turn = !self.turn;
        Ok(captured_piece)
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
        let mut fen_iter = fen.split(' ');

        let position_fen = fen_iter.next().ok_or("Empty Fen")?;
        let board = Board::try_from_fen(position_fen)?;

        let turn = match fen_iter.next() {
            Some("w") => Color::White,
            Some("b") => Color::Black,
            _ => return Err("Invalid Fen"),
        };

        let castle_fen = fen_iter.next().ok_or("Empty Fen")?;
        let castle = CastleRights::try_from_fen(castle_fen)?;

        let ep: Option<Square> = match fen_iter.next() {
            Some("-") => None,
            Some(coords) => Some(Square::try_from_notation(coords)?),
            None => return Err("Invalid Fen"),
        };

        let halfmoves = fen_iter.next().map(|x| x.parse::<u32>()).unwrap().unwrap();

        let fullmoves = fen_iter.next().map(|x| x.parse::<u32>()).unwrap().unwrap();

        Ok(Self {
            board,
            turn,
            castle,
            ep,
            halfmoves,
            fullmoves,
        })
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(25);

        // board
        fen.push_str(&self.board.to_fen());
        fen.push(' ');

        // turn
        let turn_char = match self.turn {
            Color::White => 'w',
            Color::Black => 'b',
        };
        fen.push(turn_char);
        fen.push(' ');

        // castle rights
        fen.push_str(&self.castle.to_fen());
        fen.push(' ');

        // en passant
        match self.ep {
            Some(s) => fen.push_str(&s.to_notation()),
            None => fen.push('-'),
        }
        fen.push(' ');

        // halfmove
        fen.push_str(&self.halfmoves.to_string());
        fen.push(' ');

        // fullmove
        fen.push_str(&self.fullmoves.to_string());

        fen
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::try_from_fen(DEFAULT_FEN).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen() {
        let gs = GameState::default();
        assert_eq!(gs.board, Board::try_from_fen(DEFAULT_FEN).unwrap());
        assert_eq!(gs.turn, Color::White);
        assert_eq!(gs.castle, CastleRights::new(true, true, true, true));
        assert_eq!(gs.ep, None);
        assert_eq!(gs.halfmoves, 0);
        assert_eq!(gs.fullmoves, 1);
    }

    #[test]
    fn test_legal_moves() {
        let fen = "7k/r7/8/1b1r4/6B1/8/8/3R3K w - - 0 1";
        let mut gs = GameState::try_from_fen(fen).unwrap();

        //Illegal bishop move (wrong color)
        let move_ = Move::try_from_notation("b5a6").unwrap();
        let error = gs.make_move(move_);
        assert_eq!(error, Err("Illegal Move"));

        //Rook take rook
        let move_ = Move::try_from_notation("d1d5").unwrap();
        let piece = gs.make_move(move_).unwrap();
        assert_eq!(
            piece,
            Some(Piece {
                color: Color::Black,
                figure: Figure::Rook
            })
        );

        //Bishop pin
        let move_ = Move::try_from_notation("b5c6").unwrap();
        let _ = gs.make_move(move_).unwrap();

        //Illegal rook move (pinned)
        let move_ = Move::try_from_notation("d5d7").unwrap();
        let error = gs.make_move(move_);
        assert_eq!(error, Err("Illegal Move"));

        //Break pin
        let move_ = Move::try_from_notation("g4f3").unwrap();
        let _ = gs.make_move(move_).unwrap();

        //Random rook move
        let move_ = Move::try_from_notation("a7d7").unwrap();
        let _ = gs.make_move(move_).unwrap();

        //Legal rook move (not pinned)
        let move_ = Move::try_from_notation("d5d7").unwrap();
        let piece = gs.make_move(move_).unwrap();
        assert_eq!(
            piece,
            Some(Piece {
                color: Color::Black,
                figure: Figure::Rook
            })
        );

        //Bishop takes rook
        let move_ = Move::try_from_notation("c6d7").unwrap();
        let piece = gs.make_move(move_).unwrap();
        assert_eq!(
            piece,
            Some(Piece {
                color: Color::White,
                figure: Figure::Rook
            })
        );
        let expected_fen = "7k/3b4/8/8/8/5B2/8/7K w - - 0 4";
        assert_eq!(gs.to_fen(), expected_fen);
    }
}
