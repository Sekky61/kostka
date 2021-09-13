use std::vec;

use super::Player;
use crate::{dice_m::Hand, game_m::GameAction};

pub enum GameStatus {
    Winning(u16), // todo vec?
    Won(u16),
    NobodyWinning,
    Error(&'static str),
}

pub struct GameState {
    round: u16,
    players_count: u16,
    playing: u16,
    score_goal: u32,
    player_scores: Vec<u32>,
    bad_state: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            round: 0,
            players_count: 0,
            playing: 0,
            score_goal: 0,
            player_scores: vec![],
            bad_state: false,
        }
    }

    pub fn with_goal(score_goal: u32) -> Self {
        GameState {
            round: 0,
            players_count: 0,
            playing: 0,
            score_goal,
            player_scores: vec![],
            bad_state: false,
        }
    }

    pub fn set_goal(&mut self, score_goal: u32) {
        self.score_goal = score_goal;
    }

    pub fn add_player(&mut self) {
        self.players_count += 1;
        self.player_scores.push(0); // todo maybe create vector on game start
    }

    pub fn add_score_to_current(&mut self, score: u32) {
        let current_index = self.playing;
        let get = self.player_scores.get_mut(current_index as usize);

        match get {
            Some(player_score) => *player_score += score,
            None => unreachable!(),
        }
    }

    pub fn update_player_turn(&mut self, turn: TurnResult) {
        match turn {
            TurnResult::Error(_) => todo!(),
            TurnResult::Nothing => {}
            TurnResult::Value(v) => {
                self.add_score_to_current(v);
            }
        };

        // next players move
        self.playing = if self.playing == self.players_count - 1 {
            self.round += 1;
            0
        } else {
            self.playing + 1
        };
    }

    pub fn game_status(&self) -> GameStatus {
        let who_winning: Vec<_> = self
            .player_scores
            .iter()
            .enumerate()
            .filter(|(i, &score)| score > self.score_goal)
            .collect();

        if self.bad_state {
            return GameStatus::Error("In bad state"); // todo hold error message
        }

        match who_winning.as_slice() {
            [] => GameStatus::NobodyWinning,
            [el] => GameStatus::Winning(el.0 as u16),
            other => todo!(),
        }
    }
}

pub struct Game {
    players: Vec<Player>,
    state: GameState,
}

enum TurnResult {
    Error(&'static str),
    Nothing,
    Value(u32),
}

pub enum MatchResult {
    Won(u16),
    Error(&'static str),
}

impl Game {
    pub fn new() -> Game {
        let game_state = GameState::new();
        Game {
            players: vec![],
            state: game_state,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
        self.state.add_player();
    }

    pub fn set_limit(&mut self, limit: u32) {
        self.state.set_goal(limit);
    }

    pub fn play(&mut self) -> MatchResult {
        let res = loop {
            // play a turn
            let res = self.play_player();

            // update state
            self.state.update_player_turn(res);

            match self.state.game_status() {
                GameStatus::Winning(_) => continue,
                GameStatus::Won(i) => break MatchResult::Won(i),
                GameStatus::NobodyWinning => continue,
                GameStatus::Error(e) => break MatchResult::Error(e),
            }
        };

        res
    }

    // todo refactor
    fn play_player(&mut self) -> TurnResult {
        let player_index = self.state.playing;
        let player_option: Option<&mut Player> = self.players.get_mut(player_index as usize);
        let player = if let Some(pl) = player_option {
            pl
        } else {
            return TurnResult::Error("Wrong player index");
        };

        let mut score = 0;
        let mut dices_available = 6;

        player.new_round();

        // todo fix print
        // let name = player.get_name();
        // let current_score = ;
        // println!("Playing: {} - score {}", name, current_score);

        loop {
            let hand = Hand::with_dices(dices_available);

            print!("score: {} | dices: ", score);

            for dice in hand.get_dices() {
                print!(" {}", dice);
            }
            println!();

            let take = player.pick_take(&self.state, hand);

            // todo check if take is from hand, do not trust player

            match take {
                Some(take) => {
                    score += take.value;
                    dices_available -= take.dices_count();
                }
                None => return TurnResult::Nothing, // no move possible
            };

            println!("score: {}", score);

            match dices_available {
                0 => {
                    dices_available = 6;
                    player.new_dices()
                }
                1 | 2 => {
                    if player.continue_or_stop(&self.state) == GameAction::Stop {
                        break;
                    }
                }
                _ => {}
            };
        }

        TurnResult::Value(score)
    }
}
