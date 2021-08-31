use std::io;

use super::{GameAction, Player};
use crate::dice_m::{Hand, TakeOption};

#[derive(Debug)]
pub struct HumanPlayer {
    name: String,
    score: i32,
}

impl HumanPlayer {
    pub fn new(name: &str) -> Self {
        HumanPlayer {
            name: name.into(),
            score: 0,
        }
    }
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

    fn pick_take(&self, hand: Hand) -> Option<TakeOption> {
        let mut takes: Vec<_> = hand.get_takes().collect();
        takes.sort_by(|take, other| other.value.cmp(&take.value));

        if takes.is_empty() {
            return None;
        }

        let must_takes = hand.takes_use_all();

        let takes_to_list = match must_takes.len() {
            0 => takes,
            _ => {
                println!("All dices used - must pick:");
                must_takes
            }
        };

        for (i, take) in takes_to_list.iter().enumerate() {
            println!("{}) {} - {:?}", i + 1, take.value, take.dices_used);
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Stdin error");
        let trimmed_line = input.trim();
        let pick: i32 = trimmed_line.parse().expect("Not a number");

        takes_to_list.get(pick as usize - 1).cloned().cloned()
    }

    fn continue_or_stop(&self, dices_left: usize) -> GameAction {
        match dices_left {
            1 | 2 => GameAction::Stop,
            _ => GameAction::Continue,
        }
    }
}
