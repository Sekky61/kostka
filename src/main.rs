mod dice_m;
use dice_m::*;

fn main() {
    println!("Hello, world!"); 

    let mut hand = Hand::new();
    hand.roll();

    println!("Rolled {}", hand);
}
