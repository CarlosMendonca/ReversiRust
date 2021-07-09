use crate::reversi::piece::Piece;

#[derive(Debug, PartialEq)]
pub enum PlayResult {
    ValidWithScore(ValidPlay),
    Invalid,
    Undefined,
} 

#[derive(Debug, PartialEq)]
pub struct ValidPlay { 
    score: usize,
    coord: (usize, usize),
    changed_coords: Vec<(usize, usize)>,
    player: Piece,
}

impl ValidPlay {
    pub fn new(score: usize, coord: (usize, usize), changed_coords: Vec<(usize, usize)>, player: Piece) -> ValidPlay {
        ValidPlay { 
            score: score,
            coord: coord,
            changed_coords: changed_coords,
            player: player,
        }
    }

    pub fn changed_coords(&self) -> &Vec<(usize, usize)> {
        &self.changed_coords
    }

    pub fn player(&self) -> &Piece {
        &self.player
    }
}