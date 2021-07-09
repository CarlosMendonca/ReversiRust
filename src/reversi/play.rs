#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayResult {
    ValidWithScore(ValidPlay),
    Invalid,
    Undefined,
} 

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ValidPlay { 
    score: usize,
    coord: (usize, usize),
}

impl ValidPlay {
    pub fn new(score: usize, coord: (usize, usize)) -> ValidPlay {
        ValidPlay { 
            score: score,
            coord: coord,
        }
    }
}