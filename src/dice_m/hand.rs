use super::{Dice, Dices, ScoredCombination, TakeOption};
use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
pub struct Hand {
    dices: Dices,
    dice_counts: [i32; 6], //todo generic
    take_options: HashSet<TakeOption>,
}

impl Hand {
    // pub fn new() -> Self {
    //     let mut hand = Hand {
    //         dices: Dices::of_length(6),
    //         dice_counts: Default::default(),
    //         take_options: Default::default(),
    //     };
    //     hand.analyze_dices();
    //     hand
    // }

    pub fn with_dices(n: usize) -> Self {
        let mut hand = Hand {
            dices: Dices::of_length(n),
            dice_counts: Default::default(),
            take_options: Default::default(),
        };
        hand.analyze_dices();
        hand
    }

    pub fn get_takes(&self) -> std::collections::hash_set::Iter<'_, TakeOption> {
        self.take_options.iter()
    }

    pub fn get_dices(&self) -> &[Dice] {
        self.dices.as_slice()
    }

    pub fn dices_used(&self) -> usize {
        self.dices.len()
    }

    pub fn takes_use_all(&self) -> Vec<&TakeOption> {
        let hand_dices = self.dices_used();
        self.take_options
            .iter()
            .filter(|&take| take.dices_count() == hand_dices)
            .collect()
    }

    fn generate_counts(&mut self) {
        self.dice_counts = Default::default(); // zero out
        for dice in self.dices.iter() {
            match dice.value {
                1..=6 => self.dice_counts[(dice.value as usize) - 1] += 1,
                _ => panic!("Dice has value outside of 1..=6"),
            };
        }
    }

    fn analyze_dices(&mut self) {
        self.generate_counts();
        self.take_options = self.generate_basic_options();
        self.combine_options();
    }

    fn dices_with(&self, value: i32) -> i32 {
        let dice_range = 1..=6;
        assert!(dice_range.contains(&value));
        self.dice_counts[(value as usize) - 1]
    }

    fn generate_basic_options(&self) -> HashSet<TakeOption> {
        let mut options = HashSet::default();

        for face in 1..=6 {
            let n_of_dices = self.dices_with(face);
            options.extend(TakeOption::one_face_all_combinations(face, n_of_dices));
        }

        if self.dice_counts == [1, 1, 1, 1, 1, 1] {
            options.insert(TakeOption::from_combination(
                [1, 1, 1, 1, 1, 1],
                ScoredCombination::Straight,
            ));
        }

        options
    }

    fn takes_overlap(&self, first: &TakeOption, second: &TakeOption) -> bool {
        let mut available_dices = self.dice_counts.to_owned();

        for (i, dice) in first.dices_used.iter().enumerate() {
            available_dices[i] -= dice;
        }

        for (i, dice) in second.dices_used.iter().enumerate() {
            available_dices[i] -= dice;
        }

        available_dices.iter().any(|&n| n < 0)
    }

    fn includes_take(&self, take: &TakeOption) -> bool {
        self.take_options.iter().any(|x| x == take)
    }

    fn make_combinations(&self) -> HashSet<TakeOption> {
        let mut new_combinations = HashSet::new();

        for option in self.take_options.iter() {
            for option_ot in self.take_options.iter() {
                if !self.takes_overlap(option, option_ot) {
                    let new_option = option.combine(option_ot);
                    if !self.includes_take(&new_option) {
                        new_combinations.insert(new_option);
                    }
                }
            }
        }

        new_combinations
    }

    fn combine_options(&mut self) {
        loop {
            let new_options = self.make_combinations();
            match new_options.len() {
                0 => break,
                _ => {
                    self.take_options.extend(new_options);
                }
            }
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " Dices: {}\nCounts: {:?}\n Takes:\n",
            self.dices, self.dice_counts
        )?;
        for opt in self.take_options.iter() {
            writeln!(f, "{:?}", opt)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn hand_from_dices(dices: Dices) -> Hand {
        let mut hand = Hand {
            dices,
            dice_counts: Default::default(),
            take_options: Default::default(),
        };
        hand.analyze_dices();
        hand
    }

    #[test]
    fn detect_straight() {
        let dices = Dices::from([4, 3, 5, 6, 2, 1]); // straight
        let hand = hand_from_dices(dices);

        let expected = [
            TakeOption {
                dices_used: [1, 1, 1, 1, 1, 1],
                value: 2000,
            },
            TakeOption {
                dices_used: [1, 0, 0, 0, 0, 0],
                value: 100,
            },
            TakeOption {
                dices_used: [0, 0, 0, 0, 1, 0],
                value: 50,
            },
            TakeOption {
                dices_used: [1, 0, 0, 0, 1, 0],
                value: 150,
            },
        ];

        let expected_hash = expected.iter().cloned().collect();

        assert_eq!(hand.take_options, expected_hash);
    }

    mod take_all {
        use super::*;

        impl Hand {
            fn can_take_all(&self) -> bool {
                !self.takes_use_all().is_empty()
            }
        }

        #[test]
        fn can_take_all_straight() {
            let dices = Dices::from([4, 3, 5, 6, 2, 1]); // straight
            let hand = hand_from_dices(dices);

            assert!(hand.can_take_all());
        }

        #[test]
        fn two_triplets() {
            let dices = Dices::from([2, 2, 2, 3, 3, 3]);
            let hand = hand_from_dices(dices);

            assert!(hand.can_take_all())
        }

        #[test]
        fn combination() {
            let dices = Dices::from([1, 1, 5, 6, 6, 6]);
            let hand = hand_from_dices(dices);

            assert!(hand.can_take_all())
        }

        #[test]
        fn nothing() {
            let dices = Dices::from([2, 2, 3, 4, 6, 6]);
            let hand = hand_from_dices(dices);

            assert!(!hand.can_take_all())
        }

        #[test]
        fn five_out_of_six() {
            let dices = Dices::from([4, 4, 4, 5, 5, 6]);
            let hand = hand_from_dices(dices);

            assert!(!hand.can_take_all())
        }
    }

    mod combination {

        use super::*;

        #[test]
        fn takes_overlap_two_ones() {
            let dices = Dices::from([1, 1, 3, 4, 4, 6]);
            let hand = hand_from_dices(dices);

            let expected = [
                TakeOption {
                    dices_used: [1, 0, 0, 0, 0, 0],
                    value: 100,
                },
                TakeOption {
                    dices_used: [2, 0, 0, 0, 0, 0],
                    value: 200,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(hand.take_options, expected_hash);
        }

        #[test]
        fn combine_one_one_five() {
            let dices = Dices::from([1, 1, 3, 4, 4, 5]);
            let hand = hand_from_dices(dices);

            let expected = [
                TakeOption {
                    dices_used: [1, 0, 0, 0, 0, 0],
                    value: 100,
                },
                TakeOption {
                    dices_used: [2, 0, 0, 0, 0, 0],
                    value: 200,
                },
                TakeOption {
                    dices_used: [0, 0, 0, 0, 1, 0],
                    value: 50,
                },
                TakeOption {
                    dices_used: [1, 0, 0, 0, 1, 0],
                    value: 150,
                },
                TakeOption {
                    dices_used: [2, 0, 0, 0, 1, 0],
                    value: 250,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(hand.take_options, expected_hash);
        }

        #[test]
        fn combine_two_triples() {
            let dices = Dices::from([2, 2, 2, 6, 6, 6]);
            let hand = hand_from_dices(dices);

            let expected = vec![
                TakeOption {
                    dices_used: [0, 3, 0, 0, 0, 0],
                    value: 200,
                },
                TakeOption {
                    dices_used: [0, 0, 0, 0, 0, 3],
                    value: 600,
                },
                TakeOption {
                    dices_used: [0, 3, 0, 0, 0, 3],
                    value: 800,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(hand.take_options, expected_hash);
        }
    }
}
