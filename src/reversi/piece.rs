use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    Black,
    White,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Piece::Black => write!(f, "B")?,
           Piece::White => write!(f, "W")?,
       }

       Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardSquare {
    Played(Piece),
    Unplayed,
    OutOfBounds,
}

impl fmt::Display for BoardSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           BoardSquare::Played(p) => write!(f, "{}", p)?,
           BoardSquare::Unplayed => write!(f, " ")?,
           BoardSquare::OutOfBounds => write!(f, "?")?,
       }

       Ok(())
    }
}
