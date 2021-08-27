use std::collections::HashSet;

use super::{Hand, TakeOption};

#[derive(Debug)]
pub enum PlayerType {
    AI,
    Human,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    score: i32,
    player_type: PlayerType,
}

impl Player {
    pub fn new(name: &str, player_type: PlayerType) -> Self {
        Player {
            name: name.into(),
            score: 0,
            player_type,
        }
    }

    fn play(&self) -> i32 {
        let hand = Hand::with_dices(6);

        let take = match self.player_type {
            PlayerType::AI => self.ai_pick_take(hand.get_takes()),
            PlayerType::Human => self.interactive_pick_take(hand.get_takes()),
        };

        match take {
            Some(take) => take.value,
            None => 0, // no move possible
        }
    }

    fn ai_pick_take<'a, I>(&self, mut takes: I) -> Option<TakeOption>
    where
        I: Iterator<Item = &'a TakeOption>,
    {
        takes.next().copied()
    }

    fn interactive_pick_take<'a, I>(&self, mut takes: I) -> Option<TakeOption>
    where
        I: Iterator<Item = &'a TakeOption>,
    {
        takes.next().copied()
    }
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    round: i32,
    playing: usize,
    score_goal: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: vec![],
            round: 0,
            playing: 0,
            score_goal: 500,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn play(&mut self) {
        loop {
            self.play_round();

            if self.players.iter().any(|p| p.score > self.score_goal) {
                //todo more winners in one round
                break;
            }
        }

        println!("Game finished");
    }

    fn play_round(&mut self) {
        for player in self.players.iter_mut() {
            let score_gain = player.play();
            player.score += score_gain;
        }
    }
}
