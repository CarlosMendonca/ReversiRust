pub mod reversi;

use reversi::board::Board;

fn main() {
    let board = Board::new();
    print!("{}", &board.current_player);
}
