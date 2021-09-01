use std::io;

use super::{GameAction, Player};
use crate::dice_m::{Hand, TakeOption};

#[derive(Debug)]
pub struct HumanPlayer {
    name: String,
    score: i32,
    hand: Option<Hand>,
    round_score: i32,
    round_dices_used: usize,
}

impl HumanPlayer {
    pub fn new(name: &str) -> Self {
        HumanPlayer {
            name: name.into(),
            score: 0,
            hand: None,
            round_score: 0,
            round_dices_used: 0,
        }
    }

    // fn play(&self) -> Option<i32> {
    //     let mut score = 0;
    //     let mut dices_available = 6;

    //     println!(
    //         "---------------------\nPlayer: {} | Score: {}",
    //         self.get_name(),
    //         self.get_score()
    //     );

    //     loop {
    //         let hand = Hand::with_dices(dices_available);

    //         print!("score: {} | dices: ", score);

    //         for dice in hand.get_dices() {
    //             print!(" {}", dice);
    //         }

    //         println!();

    //         let take = self.pick_take(hand);

    //         match take {
    //             Some(take) => {
    //                 score += take.value;
    //                 dices_available -= take.dices_count();
    //             }
    //             None => return None, // no move possible
    //         };

    //         println!("score: {}", score);
    //         let action = self.continue_or_stop(dices_available);

    //         if action == GameAction::Stop {
    //             break;
    //         }
    //     }

    //     Some(score)
    // }
}

impl Player for HumanPlayer {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_score(&self) -> i32 {
        self.score
    }

    fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    fn give_hand(&mut self, hand: Hand) {
        self.hand = Some(hand);
    }

    fn pick_take(&mut self) -> Option<TakeOption> {
        let takes_to_list = {
            let hand = self.hand.as_ref().expect("Cannot pick: no hand");

            let mut takes: Vec<&TakeOption> = hand.get_takes().collect();
            takes.sort_by(|take, other| other.value.cmp(&take.value));

            if takes.is_empty() {
                return None;
            }

            let must_takes = hand.takes_use_all();

            match must_takes.len() {
                0 => takes,
                _ => {
                    println!("All dices used - must pick:");
                    must_takes
                }
            }
        };

        for (i, take) in takes_to_list.iter().enumerate() {
            println!("{}) {} - {:?}", i + 1, take.value, take.dices_used);
        }

        // user's pick
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Stdin error");
        let trimmed_line = input.trim();
        let pick: i32 = trimmed_line.parse().expect("Not a number");

        let take = takes_to_list.get(pick as usize - 1).map(|&&take| take);

        if let Some(t) = take {
            self.round_score += t.value();
            self.round_dices_used += t.dices_count();
        };

        take
    }

    fn continue_or_stop(&self) -> GameAction {
        let dices_left = self.hand.as_ref().expect("No hand error").dices_used()
            - (self.round_dices_used as usize);
        match dices_left {
            1 | 2 => {
                println!("Do you want to end your turn? (y/n)");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Stdin error");
                let trimmed_line = input.trim();
                if trimmed_line == "y" {
                    GameAction::Stop
                } else {
                    GameAction::Continue
                }
            }
            0 => GameAction::Continue, // todo move to game logic
            _ => GameAction::Continue,
        }
    }
}
