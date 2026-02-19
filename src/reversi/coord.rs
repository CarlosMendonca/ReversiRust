#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub row: isize,
    pub col: isize,
}

impl From<(usize, usize)> for Coord {
    fn from(tuple: (usize, usize)) -> Self {
        Coord { row: tuple.0, col: tuple.1 }
    }
}

impl From<(isize, isize)> for Vector {
    fn from(tuple: (isize, isize)) -> Self {
        Vector { row: tuple.0, col: tuple.1 }
    }
}

impl Coord {
    pub fn towards(&self, vector: &Vector, hops: usize) -> Option<Coord> {
        let row = self.row as isize + vector.row * hops as isize;
        let col = self.col as isize + vector.col * hops as isize;
        if row < 0 || col < 0 {
            None
        } else {
            Some(Coord {
                row: row as usize,
                col: col as usize,
            })
        }
    }
}
