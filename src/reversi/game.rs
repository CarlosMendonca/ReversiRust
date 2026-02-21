use std::fmt;

use crate::reversi::board::Board;
use crate::reversi::coord::{Coord, Vector};
use crate::reversi::piece::*;
use crate::reversi::move_result::*;

pub struct Turn {
    pub player: Piece,
    pub valid_moves: Vec<PositionalOutcome>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    New,
    Played,
    PlayedAndPassed,
    GameOver,
}

pub struct Game {
    board: Board,
    current_turn: Turn,
    state: GameState,
}

impl Game {
    pub fn new() -> Game {
        // Initialize game
        let mut game = Game {
            board: Board::new(),
            current_turn: Turn { player: Piece::White, valid_moves: Vec::new() },
            state: GameState::New,
        };
        
        // Advance to next turn knows how to handle a new game
        game.advance_to_next_turn();
        game
    }

    pub fn current_turn(&self) -> &Turn {
        &self.current_turn
    }

    fn check_move_for(&self, player: Piece, at_coord: Coord) -> MoveResult {
        let (_, square) = self.board.get_coord_square_at(at_coord);
        match square {
            BoardSquare::Played(_) => MoveResult::Invalid,
            BoardSquare::OutOfBounds => MoveResult::OutOfBounds,
            BoardSquare::Unplayed => {
                let mut captured_coords: Vec<Coord> = Vec::new();

                let vectors = Game::get_direction_vectors();
                for vector in vectors {
                    let x = self.get_captured_coords_for(player, at_coord, vector);
                    captured_coords.extend(x);
                }

                match captured_coords.len() {
                    0 => MoveResult::Invalid,
                    _ => MoveResult::Valid(PositionalOutcome::new(
                        at_coord,
                        captured_coords,
                    )),
                }
            }
        }
    }

    pub fn try_play(&mut self, move_coord: Coord) -> Result<(), PlayError> {
        // Should prob check if game is over at this point

        let confirmed_valid_move = self.current_turn.valid_moves
            .iter()
            .find(|_move| *_move.coord() == move_coord)
            .ok_or(PlayError)?;

        // Place the new piece and flip captured pieces
        let mut coords_to_flip = confirmed_valid_move.changed_coords().clone(); // get pre-calculated coords to flip from valid play
        coords_to_flip.push(*confirmed_valid_move.coord()); // add the play itself; maybe this should already be inside the coords to flip
        self.board.set_squares(&coords_to_flip, self.current_turn.player);

        self.state = GameState::Played;

        self.advance_to_next_turn();
        if self.current_turn_has_valid_moves() {
            return Ok(());
        }

        // process another turn
        self.state = GameState::PlayedAndPassed;
        self.advance_to_next_turn();
        if self.current_turn_has_valid_moves() {
            return Ok(());
        }

        // return game over
        self.state = GameState::GameOver;
        return Ok(());
    }

    fn advance_to_next_turn(&mut self) {
        debug_assert_ne!(self.state, GameState::GameOver, "advance_to_next_turn called in GameOver state");

        let next_turn_player = match self.state {
            GameState::New => self.current_turn.player,
            GameState::Played | GameState::PlayedAndPassed => self.current_turn.player.opponent(),
            GameState::GameOver => unreachable!(),
        };

        self.current_turn = Turn {
            player: next_turn_player,
            valid_moves: self.calculate_valid_moves_for(next_turn_player),
        };
    }

    fn current_turn_has_valid_moves(&self) -> bool {
        !self.current_turn.valid_moves.is_empty()
    }

    fn calculate_valid_moves_for(&self, player: Piece) -> Vec<PositionalOutcome> {
        let mut valid_moves = Vec::new();

        // for every square in the board, check if playing that move as the current player is possible and cache it in a list of valid moves
        for row in 1..=Board::BOARD_SIZE {
            for column in 1..=Board::BOARD_SIZE {
                match self.check_move_for(player, (row - 1, column - 1).into()) {
                    MoveResult::Valid(_move) => valid_moves.push(_move),
                    _ => (), // don't add the invalid plays
                }
            }
        }

        valid_moves        
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

    fn get_captured_coords_for(&self, player: Piece, at_coord: Coord, for_vector: Vector) -> Vec<Coord> {
        let mut hops: usize = 1;
        let mut switchable_coords: Vec<Coord> = Vec::new();

        loop {
            let (current_coord, current_square) = self.board.get_coord_square_towards(at_coord, for_vector, hops);

            match current_square {
                BoardSquare::Played(piece) if piece == player.opponent() => {
                    hops += 1;
                    switchable_coords.push(current_coord);
                }
                BoardSquare::Played(_) => return switchable_coords,
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
    use super::{BoardSquare, Game, Piece, MoveResult, PositionalOutcome, Turn};
    use crate::reversi::{coord::Coord, game::GameState};

    #[test]
    fn can_initialize_game() {
        let game = Game::new();

        // Assert first player, which is white
        assert_eq!(game.current_turn.player, Piece::White);
        assert_eq!(game.state, GameState::New);
    }

    #[test]
    fn checking_invalid_play_sets_status_correctly() {
        let game = Game::new();

        let move_1 = game.check_move_for(game.current_turn.player, (0, 0).into());
        let move_2 = game.check_move_for(game.current_turn.player, (9, 9).into()); 

        // Asserting an invalid play (first row, first column on a starting board)
        assert_eq!(move_1, MoveResult::Invalid);
        assert_eq!(move_2, MoveResult::OutOfBounds);
    }

    #[test]
    fn checking_valid_play_sets_state_correctly() {
        let game = Game::new();
        let mut play: MoveResult;

        play = game.check_move_for(game.current_turn.player,(3, 2).into());
        assert_eq!(
            play,
            MoveResult::Valid(PositionalOutcome::new(
                (3, 2).into(),
                vec![(3, 3).into()],
            ))
        );

        play = game.check_move_for(game.current_turn.player,(2, 3).into());
        assert_eq!(
            play,
            MoveResult::Valid(PositionalOutcome::new((2, 3).into(), vec![(3, 3).into()]))
        );

        play = game.check_move_for(game.current_turn.player,(4, 5).into());
        assert_eq!(
            play,
            MoveResult::Valid(PositionalOutcome::new((4, 5).into(), vec![(4, 4).into()]))
        );

        play = game.check_move_for(game.current_turn.player,(5, 4).into());
        assert_eq!(
            play,
            MoveResult::Valid(PositionalOutcome::new((5, 4).into(), vec![(4, 4).into()]))
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

        assert_eq!(game.current_turn.player, Piece::Black);

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
    fn initialized_game_checks_available_positions_correctly() {
        let game = Game::new();

        assert_eq!(game.current_turn.valid_moves.len(), 4);

        assert_eq!(*game.current_turn.valid_moves[0].coord(), Coord::from((2, 3)));
        assert_eq!(*game.current_turn.valid_moves[1].coord(), Coord::from((3, 2)));
        assert_eq!(*game.current_turn.valid_moves[2].coord(), Coord::from((4, 5)));
        assert_eq!(*game.current_turn.valid_moves[3].coord(), Coord::from((5, 4)));
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

        let next_turn_player = Piece::White;
        game.current_turn = Turn {
            player: next_turn_player,
            valid_moves: game.calculate_valid_moves_for(next_turn_player),
        };

        let corner_move = game
            .current_turn.valid_moves
            .iter()
            .find(|play| *play.coord() == Coord::from((7, 7)));

        assert_eq!(
            corner_move,
            Some(&PositionalOutcome::new(
                (7, 7).into(),        // coord
                vec![(6, 6).into()],  // captured piece
            ))
        );
    }

    #[test]
    fn played_game_rechecks_available_positions_correctly() {
        let mut game = Game::new();

        assert_eq!(game.current_turn.player, Piece::White);

        let result = game.try_play((2, 3).into());
        assert!(result.is_ok());

        assert_eq!(game.current_turn.valid_moves.len(), 3);
        assert_eq!(game.current_turn.player, Piece::Black);

        assert_eq!(*game.current_turn.valid_moves[0].coord(), Coord::from((2, 2)));
        assert_eq!(*game.current_turn.valid_moves[1].coord(), Coord::from((2, 4)));
        assert_eq!(*game.current_turn.valid_moves[2].coord(), Coord::from((4, 2)));
    }

    /// Creates a Game where all squares are White except for the given
    /// black and empty positions. Current player is set to White with
    /// available positions already calculated.
    fn generate_endgame(
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

        let next_turn_player = Piece::White;
        game.current_turn = Turn {
            player: next_turn_player,
            valid_moves: game.calculate_valid_moves_for(next_turn_player),
        };

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
        let mut game = generate_endgame(
            &[(1, 1).into(), (7, 5).into()],
            &[(0, 1).into(), (7, 6).into()],
        );

        assert_eq!(game.current_turn.player, Piece::White); // before playing, player is White
        let result = game.try_play((7, 6).into());
        assert!(result.is_ok());

        assert_eq!(game.current_turn.player, Piece::White); // after playing, player is still White
        assert_eq!(game.state, GameState::PlayedAndPassed);
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
        let mut game = generate_endgame(
            &[(0, 0).into(), (7, 5).into()],
            &[(0, 1).into(), (7, 6).into()],
        );

        assert_eq!(game.current_turn.player, Piece::White); // initial condition from the Board factory
        let result = game.try_play((7, 6).into());
        assert!(result.is_ok());

        assert_eq!(game.state, GameState::GameOver);
    }
}
