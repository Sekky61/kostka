use crate::dice_m::{Hand, TakeOption};

#[derive(Debug, PartialEq, Eq)]
pub enum GameAction {
    Continue,
    Stop,
}

pub trait Player {
    fn get_name(&self) -> &str;
    fn get_score(&self) -> i32;
    fn add_score(&mut self, score: i32);

    fn pick_take(&self, hand: Hand) -> Option<TakeOption>;

    fn continue_or_stop(&self, dices_left: usize) -> GameAction;

    fn play(&self) -> Option<i32> {
        let mut score = 0;
        let mut dices_available = 6;

        println!(
            "---------------------\nPlayer: {} | Score: {}",
            self.get_name(),
            self.get_score()
        );

        loop {
            let hand = Hand::with_dices(dices_available);

            print!("score: {} | dices: ", score);

            for dice in hand.get_dices() {
                print!(" {}", dice);
            }

            println!();

            let take = self.pick_take(hand);

            match take {
                Some(take) => {
                    score += take.value;
                    dices_available -= take.dices_count();
                }
                None => return None, // no move possible
            };

            println!("score: {}", score);
            let action = self.continue_or_stop(dices_available);

            if action == GameAction::Stop {
                break;
            }
        }

        Some(score)
    }
}
