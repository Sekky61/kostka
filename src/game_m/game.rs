use crate::{dice_m::Hand, game_m::GameAction};

use super::Player;

pub struct Game {
    players: Vec<Box<dyn Player>>,
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

    pub fn add_player<T>(&mut self, player: T)
    where
        T: Player + 'static,
    {
        self.players.push(Box::from(player));
    }

    pub fn play(&mut self) {
        loop {
            self.play_round();

            if self.players.iter().any(|p| p.get_score() > self.score_goal) {
                //todo more winners in one round
                break;
            }
        }

        println!("Game finished");
    }

    fn play_round(&mut self) {
        for player in self.players.iter_mut() {
            let score_gain = play_player(player);

            match score_gain {
                Some(score) => {
                    println!("Player {} played {} points", player.get_name(), score);
                    player.add_score(score);
                }
                None => println!("Player {} had no picks", player.get_name()),
            }
        }
    }
}

fn play_player(player: &mut Box<dyn Player>) -> Option<i32> {
    let mut score = 0;
    let mut dices_available = 6;

    loop {
        let hand = Hand::with_dices(dices_available);

        print!("score: {} | dices: ", score);

        for dice in hand.get_dices() {
            print!(" {}", dice);
        }
        println!();

        player.give_hand(hand);
        let take = player.pick_take();

        // todo check if take is from hand

        match take {
            Some(take) => {
                score += take.value;
                dices_available -= take.dices_count();
            }
            None => return None, // no move possible
        };

        println!("score: {}", score);

        match dices_available {
            0 => dices_available = 6,
            1 | 2 => match player.continue_or_stop() {
                GameAction::Stop => break,
                _ => {}
            },
            _ => {}
        };
    }

    Some(score)
}
