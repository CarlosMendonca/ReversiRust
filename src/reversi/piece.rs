use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    Black,
    White,
}

// TODO create a test for this
impl Piece {
    pub fn opponent(&self) -> Piece {
        match self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::reversi::piece::Piece;

    #[test]
    fn can_determine_opponent() {
        assert_eq!(Piece::White.opponent(), Piece::Black);
        assert_eq!(Piece::Black.opponent(), Piece::White);
    }
}