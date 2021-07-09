use std::fmt;

use crate::reversi::board::Board;
use crate::reversi::play::*;
use crate::reversi::piece::*;

pub struct Game {
    board: Board,
    current_player: Piece,
    last_valid_uncommited_play: PlayResult,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: Piece::White,
            last_valid_uncommited_play: PlayResult::Undefined,
        }
    }

    pub fn is_game_over(&self) -> bool {
        // game is not over as long as there is at least one valid play for current player

        // for each position
        //   check whether the position is valid (i.e. captures more than 0 pieces)
        //     match Play { 0 => continue; _ => return true }

        true
    }

    pub fn check_play(&mut self, coord: (usize, usize)) {
        match self.board.get_coord_square_at(coord).square() {
            BoardSquare::Played(_)   => self.last_valid_uncommited_play = PlayResult::Invalid,
            BoardSquare::OutOfBounds => self.last_valid_uncommited_play = PlayResult::Invalid,
            BoardSquare::Unplayed => {
                let mut captured_coords: Vec<(usize, usize)> = Vec::new();
                
                let vectors = Game::get_direction_vectors();
                for vector in vectors {
                    captured_coords.append(&mut self.get_captured_cords(coord, vector));
                }
                
                match captured_coords.len() {
                    0 => return self.last_valid_uncommited_play = PlayResult::Invalid,
                    _ => return self.last_valid_uncommited_play = PlayResult::ValidWithScore(ValidPlay::new(
                        captured_coords.len() + 1,
                        coord,
                        captured_coords,
                        self.current_player,
                    )),
                }
            }
        }
    }

    pub fn commit_last_play(&mut self) -> Result<(), PlayError> {
        match &self.last_valid_uncommited_play {
            PlayResult::Invalid | PlayResult::Undefined => Err(PlayError),
            PlayResult::ValidWithScore(play) => {
                let mut coords_to_flip: Vec<(usize, usize)> = play.changed_coords().clone();
                coords_to_flip.push(*play.coord());

                self.board.set_squares(&coords_to_flip, *play.player());
                self.last_valid_uncommited_play = PlayResult::Undefined;
                self.current_player = self.current_opponent();

                Ok(())
            }
        }
    }

    pub fn current_opponent(&self) -> Piece {
        match self.current_player {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }

    fn get_direction_vectors() -> [(isize, isize); 8] {
        [
            ( 0,  1),
            ( 1,  0),
            ( 1,  1),
            ( 0, -1),
            (-1,  0),
            (-1, -1),
            ( 1, -1),
            (-1,  1), ]
    }

    // TO-DO: nmeed to decide between coord and square
    fn get_captured_cords(&self, coord: (usize, usize), vector: (isize, isize)) -> Vec<(usize, usize)> {
        let mut hops: usize = 1;
        let mut switchable_coords: Vec<(usize, usize)> = Vec::new();

        loop {
            let current_coord_square = self.board.get_coord_square_towards(coord, vector, hops);

            match current_coord_square.square() { 
                BoardSquare::Played(piece) => {
                    if *piece == self.current_opponent() {
                        hops += 1;
                        switchable_coords.push(*current_coord_square.coord());
                    } else {
                        return switchable_coords;
                    }
                },
                _ => return Vec::new(),
            }
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.board)?;

        Ok(()) // if you got here, it means there were no errors
    }
}

#[derive(Debug, Clone)]
pub struct PlayError;

impl fmt::Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid play")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{BoardSquare, CoordinatedBoardSquare, Game, Piece, PlayResult, ValidPlay};

    #[test]
    fn can_initialize_game() {
        let game = Game::new();

        // Asserting play result
        assert_eq!(game.last_valid_uncommited_play, PlayResult::Undefined);

        // Assert first player, which is white
        assert_eq!(game.current_player, Piece::White);
    }

    #[test]
    fn checking_invalid_play_sets_status_correctly() {
        let mut game = Game::new();

        game.check_play((0, 0));

        // Asserting an invalid play (first row, first column on a starting board)
        assert_eq!(game.last_valid_uncommited_play, PlayResult::Invalid);
    }

    #[test]
    fn checking_valid_play_sets_state_correctly() {
        let mut game = Game::new();

        game.check_play((3, 2));
        assert_eq!(game.last_valid_uncommited_play, PlayResult::ValidWithScore(ValidPlay::new(
            2, // 2 because I add a new piece and capture a piece
            (3, 2),
            vec![(3, 3)],
            Piece::White)));

        game.check_play((2, 3));
        assert_eq!(game.last_valid_uncommited_play, PlayResult::ValidWithScore(ValidPlay::new(
            2,
            (2, 3),
            vec![(3, 3)],
            Piece::White)));

        game.check_play((4, 5));
        assert_eq!(game.last_valid_uncommited_play, PlayResult::ValidWithScore(ValidPlay::new(
            2,
            (4, 5),
            vec![(4, 4)],
            Piece::White)));

        game.check_play((5, 4));
        assert_eq!(game.last_valid_uncommited_play, PlayResult::ValidWithScore(ValidPlay::new(
            2,
            (5, 4),
            vec![(4, 4)],
            Piece::White)));
    }

    #[test]
    fn commiting_invalid_play_maintains_state_correctly() {
        let mut game = Game::new();

        game.check_play((0, 0));
        assert!(game.commit_last_play().is_err());
    }

    #[test]
    fn commiting_valid_play_changes_state_correctly() {
        let mut game = Game::new();

        game.check_play((3, 2));
        let result = game.commit_last_play();

        assert!(result.is_ok());

        assert_eq!(game.last_valid_uncommited_play, PlayResult::Undefined);
        assert_eq!(game.current_player, Piece::Black);

        assert_eq!(game.board.get_coord_square_at((3, 3)), CoordinatedBoardSquare::new((3, 3), BoardSquare::Played(Piece::White)));
        assert_eq!(game.board.get_coord_square_at((3, 2)), CoordinatedBoardSquare::new((3, 2), BoardSquare::Played(Piece::White)));
    }

    #[test]
    fn can_determine_opponent() {
        let mut game = Game::new();

        assert_eq!(game.current_opponent(), Piece::Black);
        
        game.current_player = Piece::Black;
        assert_eq!(game.current_opponent(), Piece::White);
    }
}