use crate::reversi::piece::Piece;

pub struct Board {
    pub current_player: Piece
}

impl Board {
    pub fn new() -> Board {
        let board = Board {
            current_player: Piece::White };

        board
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, Piece};

    #[test]
    fn can_initialize_board() {
        let board = Board::new();

        assert_eq!(Piece::White, board.current_player);
    }
}