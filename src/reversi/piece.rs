use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Piece {
    Black,
    White,
    Undefined
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Piece::Black => write!(f, "B"),
           Piece::White => write!(f, "W"),
           Piece::Undefined => write!(f, "?")
       }
    }
}