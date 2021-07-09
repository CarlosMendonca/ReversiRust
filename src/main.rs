pub mod reversi;

use reversi::game::Game;

fn main() {
    let mut game = Game::new();
    
    println!("{}", game);
    
    game.check_play((3, 2));
    game.commit_last_play();

    println!("{}", game);
}
