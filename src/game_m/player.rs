use super::{game::GameState, HumanPlayer};
use crate::dice_m::{Hand, TakeOption};

#[derive(Debug, PartialEq, Eq)]
pub enum GameAction {
    Continue,
    Stop,
}

pub struct Player {
    name: String,
    score: u32,
    brain: Box<dyn Decision>,
}

impl Player {
    pub fn human(name: &str) -> Self {
        Player {
            name: name.into(),
            score: 0,
            brain: Box::new(HumanPlayer::new()),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn add_score(&mut self, score: u32) {
        self.score += score;
    }

    pub fn pick_take(&mut self, game_state: &GameState, hand: Hand) -> Option<TakeOption> {
        self.brain.pick_take(game_state, hand)
    }

    pub fn continue_or_stop(&self, game_state: &GameState) -> GameAction {
        self.brain.continue_or_stop(game_state)
    }

    pub fn new_round(&mut self) {
        self.brain.new_round();
    }

    pub fn new_dices(&mut self) {
        self.brain.new_dices();
    }
}

pub trait Decision {
    fn new_round(&mut self);

    fn new_dices(&mut self);

    fn pick_take(&mut self, game_state: &GameState, hand: Hand) -> Option<TakeOption>;

    fn continue_or_stop(&self, game_state: &GameState) -> GameAction;
}
