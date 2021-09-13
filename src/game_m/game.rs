use super::Player;
use crate::{dice_m::Hand, game_m::GameAction};

pub struct GameState {
    round: u16,
    players_count: u16,
    playing: u16,
    score_goal: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            round: 0,
            players_count: 0,
            playing: 0,
            score_goal: 500,
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
        self.state.players_count += 1;
    }

    pub fn play(&mut self) -> MatchResult {
        loop {
            let res = self.play_player();

            self.state.playing = match self.state.playing + 1 {
                c if c == self.state.players_count => {
                    self.state.round += 1;
                    0
                }
                el => el,
            };

            match res {
                TurnResult::Error(e) => return MatchResult::Error(e),
                TurnResult::Nothing => {}
                TurnResult::Value(v) => {
                    let player_index = self.state.playing;
                    let player_option: Option<&mut Player> =
                        self.players.get_mut(player_index as usize);
                    let player = if let Some(pl) = player_option {
                        pl
                    } else {
                        return MatchResult::Error("Wrong player index");
                    };

                    player.add_score(v);

                    if let Some((i, _)) = self
                        .players
                        .iter()
                        .enumerate()
                        .find(|(_, p)| p.get_score() > self.state.score_goal)
                    {
                        //todo more winners in one round
                        println!("Game finished");
                        return MatchResult::Won(i as u16);
                    }
                }
            }
        }
    }

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

        let name = player.get_name();
        let current_score = player.get_score();
        println!("Playing: {} - score {}", name, current_score);

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
