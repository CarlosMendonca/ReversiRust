pub mod reversi;

use reversi::game::Game;

fn main() {
    let mut game = Game::new();
    
    println!("{}", game);
    
    game.try_play((3, 2));

    println!("{}", game);

    game.try_play((4, 2));

    println!("{}", game);
}
