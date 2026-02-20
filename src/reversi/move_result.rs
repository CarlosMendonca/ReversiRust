use crate::reversi::coord::Coord;

#[derive(Debug, PartialEq)]
pub enum MoveResult {
    Valid(PositionalOutcome),
    Invalid,
    OutOfBounds,
}

#[derive(Debug, PartialEq)]
pub struct PositionalOutcome {
    played_coord: Coord,
    changed_coords: Vec<Coord>,
}

impl PositionalOutcome {
    pub fn new(played_coord: Coord, changed_coords: Vec<Coord>) -> PositionalOutcome {
        PositionalOutcome {
            played_coord,
            changed_coords: changed_coords,
        }
    }

    pub fn coord(&self) -> &Coord {
        &self.played_coord
    }

    pub fn changed_coords(&self) -> &Vec<Coord> {
        &self.changed_coords
    }

    pub fn changed_coords_mut(&mut self) -> &mut Vec<Coord> {
        &mut self.changed_coords
    }
}
