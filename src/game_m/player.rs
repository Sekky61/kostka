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

            let action = self.continue_or_stop(dices_available);

            if action == GameAction::Stop {
                break;
            }

            // match dices_available {
            //     1 | 2 => {
            //         match self.player_type {
            //             PlayerType::AI => break,
            //             PlayerType::Human => {
            //                 println!("score: {}", score);
            //                 println!("Do you want to end your turn? (y/n)");

            //                 let mut input = String::new();
            //                 io::stdin().read_line(&mut input).expect("Stdin error");
            //                 let trimmed_line = input.trim();
            //                 if trimmed_line == "y" {
            //                     break;
            //                 }
            //             }
            //         };
            //     }
            //     0 => dices_available = 6,
            //     _ => {}
            // }
        }

        Some(score)
    }

    fn ai_pick_take<'a>(&self, hand: &'a Hand) -> Option<&'a TakeOption> {
        let takes: Vec<_> = hand.get_takes().collect();
        takes.get(0).cloned()
    }
}

#[derive(Debug)]
pub enum PlayerType {
    AI,
    Human,
}
