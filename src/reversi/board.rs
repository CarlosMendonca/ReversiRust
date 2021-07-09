use std::fmt;
use array2d::Array2D;

use crate::reversi::piece::{BoardSquare, CoordinatedBoardSquare, Piece};
use crate::reversi::play::{ValidPlay};

pub struct Board {
    current_player: Piece,
    squares: Array2D<BoardSquare>,
}

impl Board {
    const BOARD_SIZE: usize = 8; // keep this low to avoid problems with isize <-> conversions 

    pub fn new() -> Board {
        let mut board = Board {
            current_player: Piece::White,
            squares: Array2D::filled_with(BoardSquare::Unplayed, Board::BOARD_SIZE, Board::BOARD_SIZE), };

        board.squares[(3, 3)] = BoardSquare::Played(Piece::Black);
        board.squares[(4, 4)] = BoardSquare::Played(Piece::Black);
        board.squares[(3, 4)] = BoardSquare::Played(Piece::White);
        board.squares[(4, 3)] = BoardSquare::Played(Piece::White);

        board
    }

    pub fn get_coord_square_at(&self, coord: (usize, usize)) -> CoordinatedBoardSquare {
        return Board::get_coord_square_towards(self, coord, (0, 0), 0)
    }

    // This method will fail catastrophically for big board sizes (isize::MAX+1)!
    pub fn get_coord_square_towards(&self, coord: (usize, usize), vector: (isize, isize), hops: usize) -> CoordinatedBoardSquare {
        let row = vector.0 * (hops as isize) + coord.0 as isize;
        let column = vector.1 * (hops as isize) + coord.1 as isize;

        if row < 0 || column < 0 || row > self.squares.num_rows() as isize - 1 || column > self.squares.num_columns() as isize - 1 {
            return CoordinatedBoardSquare::new((row as usize, column as usize), BoardSquare::OutOfBounds);
        }

        CoordinatedBoardSquare::new(
            (row as usize, column as usize),
            self.squares[(row as usize, column as usize)],
        )
    }

    // This is a naive that could put the board in an invalid state, which is okay because it optimizes the code by
    // avoiding roundtrips and makes it so that the Board doens't know the rules of the game
    pub fn capture_squares(&mut self, play: &ValidPlay) {
        for coord in play.changed_coords() {
            self.squares[*coord] = BoardSquare::Played(*play.player())
        }
    }

    // Not sure this is idiomatic
    pub fn current_player(&self) -> &Piece {
        &self.current_player
    }

    // Not sure this is idiomatic
    pub fn current_player_mut(&mut self) -> &mut Piece {
        &mut self.current_player
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const HEADER:      &'static str = "   | A | B | C | D | E | F | G | H |";
        const ROW_DIVIDER: &'static str = "---+---+---+---+---+---+---+---+---+";

        writeln!(f, "{}", HEADER)?;
        writeln!(f, "{}", ROW_DIVIDER)?;

        for (i, positions_in_row) in self.squares.rows_iter().enumerate() {
            write!(f, " {} |", i+1)?;

            for position in positions_in_row {
                write!(f, " {} |", position)?;
            }

            writeln!(f)?;
            writeln!(f, "{}", ROW_DIVIDER)?;
        }

        Ok(()) // if you got here, it means there were no errors
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, Piece, BoardSquare};

    #[test]
    fn can_initialize_board() {
        let board = Board::new();

        // Asserting first player
        assert_eq!(board.current_player, Piece::White, "first player must be white");

        // Asserting initial positions
        assert_eq!(board.squares[(3, 3)], BoardSquare::Played(Piece::Black));
        assert_eq!(board.squares[(4, 4)], BoardSquare::Played(Piece::Black));
        assert_eq!(board.squares[(3, 4)], BoardSquare::Played(Piece::White));
        assert_eq!(board.squares[(4, 3)], BoardSquare::Played(Piece::White));

        // Asserting board size, which must be 8 to avoid problems with isize <-> conversions
        assert_eq!(board.squares.num_columns(), 8);
        assert_eq!(board.squares.num_rows(), 8);
    }
}