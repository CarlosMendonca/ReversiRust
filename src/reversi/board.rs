use std::fmt;
use array2d::Array2D;

use crate::reversi::coord::{Coord, Vector};
use crate::reversi::piece::{BoardSquare, CoordinatedBoardSquare, Piece};

pub struct Board {
    squares: Array2D<BoardSquare>,
}

impl Board {
    pub const BOARD_SIZE: usize = 8; // keep this low to avoid problems with isize <-> conversions

    pub fn new() -> Board {
        let mut board = Board {
            squares: Array2D::filled_with(BoardSquare::Unplayed, Board::BOARD_SIZE, Board::BOARD_SIZE), };

        board.squares[(3, 3)] = BoardSquare::Played(Piece::Black);
        board.squares[(4, 4)] = BoardSquare::Played(Piece::Black);
        board.squares[(3, 4)] = BoardSquare::Played(Piece::White);
        board.squares[(4, 3)] = BoardSquare::Played(Piece::White);

        board
    }

    pub fn get_coord_square_at(&self, coord: Coord) -> CoordinatedBoardSquare {
        return Board::get_coord_square_towards(self, coord, (0, 0).into(), 0)
    }

    // This method will fail catastrophically for big board sizes (isize::MAX+1)!
    pub fn get_coord_square_towards(&self, coord: Coord, vector: Vector, hops: usize) -> CoordinatedBoardSquare {
        match coord.towards(&vector, hops) {
            Some(dest) if dest.row < self.squares.num_rows() && dest.col < self.squares.num_columns() => {
                CoordinatedBoardSquare::new(
                    dest,
                    self.squares[(dest.row, dest.col)],
                )
            }
            _ => CoordinatedBoardSquare::new(coord, BoardSquare::OutOfBounds),
        }
    }

    // This is a naive method that could put the board in an invalid state,
    // which is okay because it optimizes the code by avoiding roundtrips and
    // makes it so that the Board doesn't know the rules of the game
    pub fn set_squares(&mut self, coords: &Vec<Coord>, player: Piece) {
        for coord in coords {
            self.squares[(coord.row, coord.col)] = BoardSquare::Played(player);
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const HEADER:      &str = "   | A | B | C | D | E | F | G | H |";
        const ROW_DIVIDER: &str = "---+---+---+---+---+---+---+---+---+";

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
    use crate::reversi::coord::Coord;

    #[test]
    fn can_initialize_board() {
        let board = Board::new();

        // Asserting initial positions
        assert_eq!(board.squares[(3, 3)], BoardSquare::Played(Piece::Black));
        assert_eq!(board.squares[(4, 4)], BoardSquare::Played(Piece::Black));
        assert_eq!(board.squares[(3, 4)], BoardSquare::Played(Piece::White));
        assert_eq!(board.squares[(4, 3)], BoardSquare::Played(Piece::White));

        // Asserting board size, which must be 8 to avoid problems with isize <-> conversions
        assert_eq!(board.squares.num_columns(), 8);
        assert_eq!(board.squares.num_rows(), 8);
    }

    #[test]
    fn capture_pieces_correctly() {
        let mut board = Board::new();
        let coords: &mut Vec<Coord> = &mut Vec::new();
        coords.push((3, 2).into());
        coords.push((3, 3).into());

        board.set_squares(coords, Piece::White);

        assert_eq!(board.squares[(3,2)], BoardSquare::Played(Piece::White));
        assert_eq!(board.squares[(3,3)], BoardSquare::Played(Piece::White));
    }
}
