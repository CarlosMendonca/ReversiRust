use std::fmt;
use array2d::Array2D;

use crate::reversi::piece::Piece;

pub struct Board {
    current_player: Piece,
    position: Array2D<Piece>
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            current_player: Piece::White,
            position: Array2D::filled_with(Piece::Undefined, 8, 8) };

        board.position[(3, 3)] = Piece::Black;
        board.position[(4, 4)] = Piece::Black;
        board.position[(3, 4)] = Piece::White;
        board.position[(4, 3)] = Piece::White;

        board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const HEADER:      &'static str = "   | A | B | C | D | E | F | G | H |";
        const ROW_DIVIDER: &'static str = "---+---+---+---+---+---+---+---+---+";

        writeln!(f, "{}", HEADER)?;
        writeln!(f, "{}", ROW_DIVIDER)?;

        for (i, row_element) in self.position.rows_iter().enumerate() {
            write!(f, " {} |", i+1)?;

            for element in row_element {
                write!(f, " {} |", element)?;
            }

            writeln!(f)?;
            writeln!(f, "{}", ROW_DIVIDER)?;
        }

        Ok(()) // if you got here, it means there were no errors
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, Piece};

    #[test]
    fn can_initialize_board() {
        let board = Board::new();

        // Asserting first player
        assert_eq!(Piece::White, board.current_player, "first player must be white");

        // Asserting initial positions
        assert_eq!(Piece::Black, board.position[(3, 3)]);
        assert_eq!(Piece::Black, board.position[(4, 4)]);
        assert_eq!(Piece::White, board.position[(3, 4)]);
        assert_eq!(Piece::White, board.position[(4, 3)]);

        // Asserting board size
        assert_eq!(8, board.position.num_columns());
        assert_eq!(8, board.position.num_rows());
    }
}