mod dice_m;
//use dice_m::*;

mod game_m;
use game_m::*;

fn main() {
    println!("Starting game");

    let mut game = Game::new();
    let p1 = Player::human("p1");
    game.add_player(p1);

    let p2 = Player::human("p2");
    game.add_player(p2);

    game.play();

    //println!("Game:\n{:?}", game);
}
