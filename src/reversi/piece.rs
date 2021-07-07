use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Piece {
    Black,
    White,
    Undefined
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Piece::Black => write!(f, "⚫"),
           Piece::White => write!(f, "⚪"),
           Piece::Undefined => write!(f, "?"),
       }
    }
}