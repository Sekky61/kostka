mod dice_m;
use dice_m::*;

fn main() {
    println!("Rolling...");

    let hand = Hand::with_dices(6);
    //hand.roll();

    println!("Rolled:\n{}", hand);
}
