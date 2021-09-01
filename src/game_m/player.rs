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

    fn give_hand(&mut self, hand: Hand);

    fn pick_take(&mut self) -> Option<TakeOption>;

    fn continue_or_stop(&self) -> GameAction;
}
