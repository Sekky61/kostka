use std::io;

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

    fn play(&self) -> Option<i32> {
        let mut score = 0;
        let mut dices_available = 6;

        println!(
            "---------------------\nPlayer: {} | Score: {}",
            self.name, self.score
        );

        loop {
            let hand = Hand::with_dices(dices_available);

            print!("score: {} | dices: ", score);

            for dice in hand.get_dices() {
                print!(" {}", dice);
            }

            println!();

            let take = match self.player_type {
                PlayerType::AI => self.ai_pick_take(&hand),
                PlayerType::Human => self.interactive_pick_take(&hand),
            };

            match take {
                Some(take) => {
                    score += take.value;
                    dices_available -= take.dices_count();
                }
                None => return None, // no move possible
            };

            match dices_available {
                1 | 2 => {
                    match self.player_type {
                        PlayerType::AI => break,
                        PlayerType::Human => {
                            println!("score: {}", score);
                            println!("Do you want to end your turn? (y/n)");

                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("Stdin error");
                            let trimmed_line = input.trim();
                            if trimmed_line == "y" {
                                break;
                            }
                        }
                    };
                }
                0 => dices_available = 6,
                _ => {}
            }
        }

        Some(score)
    }

    fn ai_pick_take<'a>(&self, hand: &'a Hand) -> Option<&'a TakeOption> {
        let takes: Vec<_> = hand.get_takes().collect();
        takes.get(0).cloned()
    }

    fn interactive_pick_take<'a>(&self, hand: &'a Hand) -> Option<&'a TakeOption> {
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

        takes_to_list.get(pick as usize - 1).cloned()
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
            match score_gain {
                Some(score) => {
                    println!("Player {} played {} points", player.name, score);
                    player.score += score;
                }
                None => println!("Player {} had no picks", player.name),
            }
        }
    }
}
