#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Figure {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub figure: Figure,
}

impl Into<char> for Piece {
    fn into(self) -> char {
        let c = match self.figure {
            Figure::Pawn => 'P',
            Figure::Rook => 'R',
            Figure::Knight => 'N',
            Figure::Bishop => 'B',
            Figure::Queen => 'Q',
            Figure::King => 'K',
        };
        if self.color == Color::Black {
            return c.to_lowercase().next().unwrap();
        }
        c
    }
}

impl TryFrom<char> for Piece {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(Piece {
                color: Color::White,
                figure: Figure::Pawn,
            }),
            'R' => Ok(Piece {
                color: Color::White,
                figure: Figure::Rook,
            }),
            'N' => Ok(Piece {
                color: Color::White,
                figure: Figure::Knight,
            }),
            'B' => Ok(Piece {
                color: Color::White,
                figure: Figure::Bishop,
            }),
            'Q' => Ok(Piece {
                color: Color::White,
                figure: Figure::Queen,
            }),
            'K' => Ok(Piece {
                color: Color::White,
                figure: Figure::King,
            }),
            'p' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Pawn,
            }),
            'r' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Rook,
            }),
            'n' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Knight,
            }),
            'b' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Bishop,
            }),
            'q' => Ok(Piece {
                color: Color::Black,
                figure: Figure::Queen,
            }),
            'k' => Ok(Piece {
                color: Color::Black,
                figure: Figure::King,
            }),
            _ => Err("Invalid char"),
        }
    }
}
