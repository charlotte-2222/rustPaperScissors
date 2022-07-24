mod game;
mod hands;

use game::{Game};


fn play_commandline(){
    let mut game=Game::new();

    game.game_loop()
}

fn main(){
    play_commandline();
}
