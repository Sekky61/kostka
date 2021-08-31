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
            let score_gain = player.play();
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
