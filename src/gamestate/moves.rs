use crate::board::Square;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

impl Move {
    pub fn try_from_alg(coords: &str) -> Result<Self, &'static str> {
        let mut iter = coords.chars();
        let from = Square::try_from_alg(&iter.by_ref().take(2).collect::<String>())?;
        let to = Square::try_from_alg(&iter.take(2).collect::<String>())?;
        Ok(Move { from, to })
    }

    #[cfg(test)]
    pub fn from_alg(coords: &str) -> Self {
        Self::try_from_alg(coords).unwrap()
    }
}
