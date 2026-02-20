use crate::reversi::coord::Coord;
use crate::reversi::piece::Piece;

#[derive(Debug, PartialEq)]
pub enum MoveResult {
    ValidWithScore(ValidMove),
    Invalid,
    Undefined,
} 

#[derive(Debug, PartialEq)]
pub struct ValidMove {
    score: usize,
    coord: Coord,
    changed_coords: Vec<Coord>,
    player: Piece,
}

impl ValidMove {
    pub fn new(score: usize, coord: Coord, changed_coords: Vec<Coord>, player: Piece) -> ValidMove {
        ValidMove {
            score: score,
            coord: coord,
            changed_coords: changed_coords,
            player: player,
        }
    }

    pub fn coord(&self) -> &Coord {
        &self.coord
    }

    pub fn changed_coords(&self) -> &Vec<Coord> {
        &self.changed_coords
    }

    pub fn changed_coords_mut(&mut self) -> &mut Vec<Coord> {
        &mut self.changed_coords
    }

    pub fn player(&self) -> &Piece {
        &self.player
    }
}