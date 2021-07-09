use crate::reversi::board::Board;
use crate::reversi::play::*;
use crate::reversi::piece::*;

pub struct Game {
    board: Board,
    last_valid_uncommited_play: PlayResult,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
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

    pub fn check_play(&mut self, coord: (usize, usize)) -> PlayResult {
        match self.board.get_square_at(coord) {
            BoardSquare::Played(_)   => self.last_valid_uncommited_play = PlayResult::Invalid,
            BoardSquare::OutOfBounds => self.last_valid_uncommited_play = PlayResult::Invalid,
            BoardSquare::Unplayed => {
                let mut score: usize = 0;

                let vectors = Game::get_all_direction_vectors();

                for vector in vectors {
                    let captured_pieces = self.count_captured_pieces_towards(coord, vector);
                    score += captured_pieces;
                }
                
                match score {
                    0 => self.last_valid_uncommited_play = PlayResult::Invalid,
                    _ => self.last_valid_uncommited_play = PlayResult::ValidWithScore(ValidPlay::new(score, coord)),
                }
            }
        }

        self.last_valid_uncommited_play
    }

    fn get_all_direction_vectors() -> [(isize, isize); 8] {
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

    fn count_captured_pieces_towards(&self, coord: (usize, usize), vector: (isize, isize)) -> usize {
        let mut hops: usize = 1;

        loop {
            let current_square = self.board.get_square_towards(coord, vector, hops);

            match current_square { 
                BoardSquare::Played(piece) => {
                    if piece == self.board.current_opponent() {
                        hops += 1;
                    } else {
                        return hops - 1;
                    }
                },
                _ => return 0,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Game, PlayResult};

    #[test]
    fn can_initialize_game() {
        let game = Game::new();

        // Asserting play result
        assert_eq!(game.last_valid_uncommited_play, PlayResult::Undefined);
    }
}