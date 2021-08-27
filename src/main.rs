mod dice_m;
use dice_m::*;

fn main() {
    println!("Starting game");

    let mut game = Game::new();
    game.add_player(Player::new("p1", PlayerType::Human));
    game.add_player(Player::new("p2", PlayerType::AI));

    game.play();

    println!("Game:\n{:?}", game);
}
