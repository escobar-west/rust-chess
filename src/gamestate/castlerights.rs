use crate::pieces::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CastleRights(u8);

impl CastleRights {
    pub fn new(wk: bool, wq: bool, bk: bool, bq: bool) -> Self {
        Self(u8::from(wk) + 2 * u8::from(wq) + 4 * u8::from(bk) + 8 * u8::from(bq))
    }

    pub fn remove_castle_rights(&mut self, color: Color) {
        self.0 &= match color {
            Color::White => 0b1100,
            Color::Black => 0b0011,
        }
    }

    pub fn remove_kingside_castle_rights(&mut self, color: Color) {
        self.0 &= match color {
            Color::White => 0b1110,
            Color::Black => 0b1011,
        }
    }

    pub fn remove_queenside_castle_rights(&mut self, color: Color) {
        self.0 &= match color {
            Color::White => 0b1101,
            Color::Black => 0b0111,
        }
    }

    pub fn can_castle_kingside(self, color: Color) -> bool {
        let king_mask = match color {
            Color::White => 0b0001,
            Color::Black => 0b0100,
        };
        self.0 & king_mask == king_mask
    }

    pub fn can_castle_queenside(self, color: Color) -> bool {
        let queen_mask = match color {
            Color::White => 0b0010,
            Color::Black => 0b1000,
        };
        self.0 & queen_mask == queen_mask
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
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

    pub fn to_fen(self) -> String {
        let fen = match self.0 {
            0b0000 => "-",
            0b0001 => "K",
            0b0010 => "Q",
            0b0011 => "KQ",
            0b0100 => "k",
            0b0101 => "Kk",
            0b0110 => "Qk",
            0b0111 => "KQk",
            0b1000 => "q",
            0b1001 => "Kq",
            0b1010 => "Qq",
            0b1011 => "KQq",
            0b1100 => "kq",
            0b1101 => "Kkq",
            0b1110 => "Qkq",
            0b1111 => "KQkq",
            _ => panic!(),
        };
        fen.into()
    }
}
