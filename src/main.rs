mod dice_m;
use dice_m::*;

fn main() {
    println!("Rolling...");

    let hand = Hand::with_dices(6);

    if hand.can_take_all() {
        println!("CAN TAKE ALL");
    }

    println!("Rolled:\n{}", hand);
}
