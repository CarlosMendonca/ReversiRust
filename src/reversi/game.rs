use std::fmt;

use crate::reversi::board::Board;
use crate::reversi::coord::{Coord, Vector};
use crate::reversi::piece::*;
use crate::reversi::valid_move::*;

pub struct Game {
    board: Board,
    current_player: Piece,
    valid_moves_available: Vec<ValidMove>,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            board: Board::new(),
            current_player: Piece::White,
            valid_moves_available: Vec::new(),
        };

        game.calculate_valid_moves_available();
        game
    }

    pub fn is_game_over(&self) -> bool {
        self.valid_moves_available.is_empty()
    }

    pub fn check_move(&mut self, coord: Coord) -> MoveResult {
        let (_, square) = self.board.get_coord_square_at(coord);
        match square {
            BoardSquare::Played(_) => MoveResult::Invalid,
            BoardSquare::OutOfBounds => MoveResult::Invalid,
            BoardSquare::Unplayed => {
                let mut captured_coords: Vec<Coord> = Vec::new();

                let vectors = Game::get_direction_vectors();
                for vector in vectors {
                    captured_coords.append(&mut self.get_captured_cords(coord, vector));
                }

                match captured_coords.len() {
                    0 => MoveResult::Invalid,
                    _ => MoveResult::ValidWithScore(ValidMove::new(
                        captured_coords.len() + 1,
                        coord,
                        captured_coords,
                        self.current_player,
                    )),
                }
            }
        }
    }

    pub fn try_play(&mut self, move_coord: Coord) -> Result<(), PlayError> {
        // Should prob check if game is over at this point

        let confirmed_valid_move = self.valid_moves_available
            .iter()
            .find(|_move| *_move.coord() == move_coord)
            .ok_or(PlayError)?;

        // Place the new piece and flip captured pieces
        let mut coords_to_flip = confirmed_valid_move.changed_coords().clone(); // get pre-calculated coords to flip from valid play
        coords_to_flip.push(*confirmed_valid_move.coord()); // add the play itself; maybe this should already be inside the coords to flip
        self.board.set_squares(&coords_to_flip, *confirmed_valid_move.player()); // set the squares for that player; should the valid player be on the valid_play object?

        // Switch to opponent and check their available moves
        self.current_player = self.current_opponent();
        self.calculate_valid_moves_available();

        // If opponent has no moves, pass the turn back
        if self.valid_moves_available.is_empty() {
            self.current_player = self.current_opponent();
            self.calculate_valid_moves_available();
        }

        Ok(())
    }

    pub fn current_opponent(&self) -> Piece {
        match self.current_player {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }

    fn get_direction_vectors() -> [Vector; 8] {
        [
            ( 0,  1).into(),
            ( 1,  0).into(),
            ( 1,  1).into(),
            ( 0, -1).into(),
            (-1,  0).into(),
            (-1, -1).into(),
            ( 1, -1).into(),
            (-1,  1).into(),
        ]
    }

    fn get_captured_cords(
        &self,
        coord: Coord,
        vector: Vector,
    ) -> Vec<Coord> {
        let mut hops: usize = 1;
        let mut switchable_coords: Vec<Coord> = Vec::new();

        loop {
            let (current_coord, current_square) = self.board.get_coord_square_towards(coord, vector, hops);

            match current_square {
                BoardSquare::Played(piece) if piece == self.current_opponent() => {
                    hops += 1;
                    switchable_coords.push(current_coord);
                }
                BoardSquare::Played(_) => return switchable_coords,
                _ => return Vec::new(),
            }
        }
    }

    fn calculate_valid_moves_available(&mut self) {
        self.valid_moves_available = Vec::new(); // reset valid moves list

        // for every square in the board, check if playing that move as the current player is possible and cache it in a list of valid moves
        for row in 1..=Board::BOARD_SIZE {
            for column in 1..=Board::BOARD_SIZE {
                match self.check_move((row - 1, column - 1).into()) {
                    MoveResult::ValidWithScore(_move) => self.valid_moves_available.push(_move),
                    _ => (), // don't add the invalid plays
                }
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
    use super::{BoardSquare, Game, Piece, MoveResult, ValidMove};
    use crate::reversi::coord::Coord;

    #[test]
    fn can_initialize_game() {
        let game = Game::new();

        // Assert first player, which is white
        assert_eq!(game.current_player, Piece::White);
        assert!(!game.is_game_over());
    }

    #[test]
    fn checking_invalid_play_sets_status_correctly() {
        let mut game = Game::new();

        let play = game.check_move((0, 0).into());

        // Asserting an invalid play (first row, first column on a starting board)
        assert_eq!(play, MoveResult::Invalid);
    }

    #[test]
    fn checking_valid_play_sets_state_correctly() {
        let mut game = Game::new();
        let mut play: MoveResult;

        play = game.check_move((3, 2).into());
        assert_eq!(
            play,
            MoveResult::ValidWithScore(ValidMove::new(
                2, // 2 because I add a new piece and capture a piece
                (3, 2).into(),
                vec![(3, 3).into()],
                Piece::White
            ))
        );

        play = game.check_move((2, 3).into());
        assert_eq!(
            play,
            MoveResult::ValidWithScore(ValidMove::new(2, (2, 3).into(), vec![(3, 3).into()], Piece::White))
        );

        play = game.check_move((4, 5).into());
        assert_eq!(
            play,
            MoveResult::ValidWithScore(ValidMove::new(2, (4, 5).into(), vec![(4, 4).into()], Piece::White))
        );

        play = game.check_move((5, 4).into());
        assert_eq!(
            play,
            MoveResult::ValidWithScore(ValidMove::new(2, (5, 4).into(), vec![(4, 4).into()], Piece::White))
        );
    }

    #[test]
    fn playing_invalid_play_returns_error() {
        let mut game = Game::new();

        assert!(game.try_play((0, 0).into()).is_err());
    }

    #[test]
    fn playing_valid_play_advances_game() {
        let mut game = Game::new();

        assert!(game.try_play((3, 2).into()).is_ok());

        assert_eq!(game.current_player, Piece::Black);

        assert_eq!(
            game.board.get_coord_square_at((3, 3).into()),
            (Coord::from((3, 3)), BoardSquare::Played(Piece::White))
        );
        assert_eq!(
            game.board.get_coord_square_at((3, 2).into()),
            (Coord::from((3, 2)), BoardSquare::Played(Piece::White))
        );
    }

    #[test]
    fn can_determine_opponent() {
        let mut game = Game::new();

        assert_eq!(game.current_opponent(), Piece::Black);

        game.current_player = Piece::Black;
        assert_eq!(game.current_opponent(), Piece::White);
    }

    #[test]
    fn initialized_game_checks_available_positions_correctly() {
        let game = Game::new();

        assert_eq!(game.valid_moves_available.len(), 4);

        assert_eq!(*game.valid_moves_available[0].coord(), Coord::from((2, 3)));
        assert_eq!(*game.valid_moves_available[1].coord(), Coord::from((3, 2)));
        assert_eq!(*game.valid_moves_available[2].coord(), Coord::from((4, 5)));
        assert_eq!(*game.valid_moves_available[3].coord(), Coord::from((5, 4)));
    }

    #[test]
    fn game_checks_limits_of_the_board_correctly() {
        // This test catches the off-by-one bug where the loop uses
        // `1..Board::BOARD_SIZE` instead of `1..=Board::BOARD_SIZE`,
        // which would skip checking row 7 and column 7 (0-indexed).
        let mut game: Game = Game::new();

        // Minimal setup: White at (5,5), Black at (6,6)
        // White playing at (7,7) captures the Black piece via the diagonal
        game.board.set_squares(&vec![(5, 5).into()], Piece::White);
        game.board.set_squares(&vec![(6, 6).into()], Piece::Black);

        game.calculate_valid_moves_available();

        let corner_move = game
            .valid_moves_available
            .iter()
            .find(|play| *play.coord() == Coord::from((7, 7)));

        assert_eq!(
            corner_move,
            Some(&ValidMove::new(
                2,                    // score: 1 new piece + 1 capture
                (7, 7).into(),        // coord
                vec![(6, 6).into()],  // captured piece
                Piece::White,
            ))
        );
    }

    #[test]
    fn played_game_rechecks_available_positions_correctly() {
        let mut game = Game::new();

        let result = game.try_play((2, 3).into());
        assert!(result.is_ok());

        assert_eq!(game.valid_moves_available.len(), 3);

        assert_eq!(*game.valid_moves_available[0].coord(), Coord::from((2, 2)));
        assert_eq!(*game.valid_moves_available[1].coord(), Coord::from((2, 4)));
        assert_eq!(*game.valid_moves_available[2].coord(), Coord::from((4, 2)));
    }

    /// Creates a Game where all squares are White except for the given
    /// black and empty positions. Current player is set to White with
    /// available positions already calculated.
    fn create_endgame(
        black: &[Coord],
        empty: &[Coord],
    ) -> Game {
        let mut game = Game::new();

        let white: Vec<Coord> = (0..8)
            .flat_map(|r| (0..8).map(move |c| (r, c).into()))
            .filter(|pos| !black.contains(pos) && !empty.contains(pos))
            .collect();

        game.board.set_squares(&white, Piece::White);
        game.board.set_squares(&black.to_vec(), Piece::Black);
        game.current_player = Piece::White;
        game.calculate_valid_moves_available();

        game
    }

    #[test]
    fn turn_passes_when_opponent_has_no_moves() {
        // Nearly-full board:
        //      0  1  2  3  4  5  6  7
        //  0:  W  .  W  W  W  W  W  W
        //  1:  W  B  W  W  W  W  W  W
        //  2:  W  W  W  W  W  W  W  W
        //  3:  W  W  W  W  W  W  W  W
        //  4:  W  W  W  W  W  W  W  W
        //  5:  W  W  W  W  W  W  W  W
        //  6:  W  W  W  W  W  W  W  W
        //  7:  W  W  W  W  W  B  .  W
        //
        // White plays (7,6), capturing (7,5).
        // After: Black only at (1,1), empty only at (0,1).
        // Black cannot play (0,1) -- no direction has White leading to Black.
        // White CAN play (0,1) -- direction (1,0): (1,1)=B --> (2,1)=W.
        // So Black's turn is passed and White plays again.
        let mut game = create_endgame(
            &[(1, 1).into(), (7, 5).into()],
            &[(0, 1).into(), (7, 6).into()],
        );

        assert_eq!(game.current_player, Piece::White); // before playing, player is White
        let result = game.try_play((7, 6).into());
        assert!(result.is_ok());

        assert_eq!(game.current_player, Piece::White); // after playing, player is still White
        assert!(!game.is_game_over());
    }

    #[test]
    fn game_over_when_neither_player_has_moves() {
        // Nearly-full board (same as pass test but (0,0)=B instead of (1,1)=B):
        //      0  1  2  3  4  5  6  7
        //  0:  B  .  W  W  W  W  W  W
        //  1:  W  W  W  W  W  W  W  W
        //  2:  W  W  W  W  W  W  W  W
        //  3:  W  W  W  W  W  W  W  W
        //  4:  W  W  W  W  W  W  W  W
        //  5:  W  W  W  W  W  W  W  W
        //  6:  W  W  W  W  W  W  W  W
        //  7:  W  W  W  W  W  B  .  W
        //
        // White plays (7,6), capturing (7,5).
        // After: Black only at (0,0), empty only at (0,1).
        // Black cannot play (0,1) -- no direction has White leading to Black.
        // White cannot play (0,1) either -- (0,0) is in the corner, uncapturable.
        // Neither player can move --> game over.
        let mut game = create_endgame(
            &[(0, 0).into(), (7, 5).into()],
            &[(0, 1).into(), (7, 6).into()],
        );

        assert_eq!(game.current_player, Piece::White); // initial condition from the Board factory
        let result = game.try_play((7, 6).into());
        assert!(result.is_ok());

        assert!(game.is_game_over());
    }
}
