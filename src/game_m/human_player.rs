use std::io;

use super::{game::GameState, Decision, GameAction};
use crate::dice_m::{Hand, TakeOption};

#[derive(Debug)]
pub struct HumanPlayer {
    round_score: u32,
    round_dices_used: usize,
}

impl HumanPlayer {
    pub fn new() -> Self {
        HumanPlayer {
            round_score: 0,
            round_dices_used: 0,
        }
    }

    pub fn reset(&mut self) {
        self.round_score = 0;
        self.round_dices_used = 0;
    }
}

impl Decision for HumanPlayer {
    fn pick_take(&mut self, game_state: &GameState, hand: Hand) -> Option<TakeOption> {
        let takes_to_list = {
            //let hand = self.hand.as_ref().expect("Cannot pick: no hand");

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

    fn continue_or_stop(&self, game_state: &GameState) -> GameAction {
        let dices_left = 6 - self.round_dices_used;
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

    fn new_round(&mut self) {
        self.reset();
    }

    fn new_dices(&mut self) {
        self.round_dices_used = 0; // todo zrusit
    }
}
