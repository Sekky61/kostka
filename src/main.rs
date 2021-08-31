mod dice_m;
//use dice_m::*;

mod game_m;
use game_m::*;

fn main() {
    println!("Starting game");

    let mut game = Game::new();
    game.add_player(HumanPlayer::new("p1"));
    game.add_player(HumanPlayer::new("p2"));

    game.play();

    //println!("Game:\n{:?}", game);
}
