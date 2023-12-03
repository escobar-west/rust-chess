use crate::board::Board;
pub struct GameState {
    board: Board,
}

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Default for GameState {
    fn default() -> Self {
        Self::try_from_fen(DEFAULT_FEN).unwrap()
    }
}

impl GameState {
    fn try_from_fen(fen: &str) -> Result<Self, &'static str> {
        let board = Board::try_from_fen(fen)?;
        Ok(Self { board })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let _gs = GameState::default();
    }
}
