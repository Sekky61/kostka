use super::{Dice, Dices, ScoredCombination, TakeOption};
use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
pub struct Hand {
    dices: Dices,
    dice_counts: [i32; 6], //todo generic
    take_options: HashSet<TakeOption>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            dices: Dices::of_length(6),
            dice_counts: Default::default(),
            take_options: Default::default(),
        }
    }

    fn generate_indexes(&mut self) {
        self.dice_counts = Default::default(); // zero out
        for dice in self.dices.iter() {
            match dice.value {
                1..=6 => self.dice_counts[(dice.value as usize) - 1] += 1,
                _ => panic!("Dice has value outside of 1..=6"),
            };
        }
    }

    pub fn roll(&mut self) {
        self.dices = Dices::of_length(self.dices.len());

        self.analyze_dices();
    }

    pub fn analyze_dices(&mut self) {
        self.generate_indexes();
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
                [1, 2, 3, 4, 5, 6],
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

    pub fn includes_take(&self, take: &TakeOption) -> bool {
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

    pub fn combine_options(&mut self) {
        loop {
            let new_options = self.make_combinations();
            println!("From current {:?}", self.take_options);
            println!("New combinations {:?}", new_options);
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
            "Dices: {}\nCounts: {:?}\nTakes: {:?}",
            self.dices, self.dice_counts, self.take_options
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::collections::HashSet;

    fn hand_from_dices(dices: Dices) -> Hand {
        Hand {
            dices,
            dice_counts: Default::default(),
            take_options: Default::default(),
        }
    }

    mod combination_tests {

        use super::*;

        #[test]
        fn takes_overlap_two_ones() {
            let dices = Dices::from([1, 1, 3, 4, 4, 6]);
            let mut hand = hand_from_dices(dices);
            hand.analyze_dices();

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
            let mut hand = hand_from_dices(dices);
            hand.analyze_dices();

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
            let mut hand = hand_from_dices(dices);
            hand.analyze_dices();

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

    mod takeoption_tests {

        use super::*;

        #[test]
        fn one() {
            let options = TakeOption::one_face_all_combinations(1, 1);
            let expected = [TakeOption {
                dices_used: [1, 0, 0, 0, 0, 0],
                value: 100,
            }];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(options, expected_hash);
        }

        #[test]
        fn two_ones() {
            let options = TakeOption::one_face_all_combinations(1, 2);
            let expected = vec![
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

            assert_eq!(options, expected_hash);
        }

        #[test]
        fn three_ones() {
            let options = TakeOption::one_face_all_combinations(1, 3);
            let expected = vec![
                TakeOption {
                    dices_used: [1, 0, 0, 0, 0, 0],
                    value: 100,
                },
                TakeOption {
                    dices_used: [2, 0, 0, 0, 0, 0],
                    value: 200,
                },
                TakeOption {
                    dices_used: [3, 0, 0, 0, 0, 0],
                    value: 1000,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(options, expected_hash);
        }

        #[test]
        fn four_ones() {
            let options = TakeOption::one_face_all_combinations(1, 4);
            let expected = vec![
                TakeOption {
                    dices_used: [1, 0, 0, 0, 0, 0],
                    value: 100,
                },
                TakeOption {
                    dices_used: [2, 0, 0, 0, 0, 0],
                    value: 200,
                },
                TakeOption {
                    dices_used: [3, 0, 0, 0, 0, 0],
                    value: 1000,
                },
                TakeOption {
                    dices_used: [4, 0, 0, 0, 0, 0],
                    value: 2000,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(options, expected_hash);
        }

        #[test]
        fn five_ones() {
            let options = TakeOption::one_face_all_combinations(1, 5);
            let expected = vec![
                TakeOption {
                    dices_used: [1, 0, 0, 0, 0, 0],
                    value: 100,
                },
                TakeOption {
                    dices_used: [2, 0, 0, 0, 0, 0],
                    value: 200,
                },
                TakeOption {
                    dices_used: [3, 0, 0, 0, 0, 0],
                    value: 1000,
                },
                TakeOption {
                    dices_used: [4, 0, 0, 0, 0, 0],
                    value: 2000,
                },
                TakeOption {
                    dices_used: [5, 0, 0, 0, 0, 0],
                    value: 4000,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(options, expected_hash);
        }

        #[test]
        fn six_ones() {
            let options = TakeOption::one_face_all_combinations(1, 6);
            let expected = vec![
                TakeOption {
                    dices_used: [1, 0, 0, 0, 0, 0],
                    value: 100,
                },
                TakeOption {
                    dices_used: [2, 0, 0, 0, 0, 0],
                    value: 200,
                },
                TakeOption {
                    dices_used: [3, 0, 0, 0, 0, 0],
                    value: 1000,
                },
                TakeOption {
                    dices_used: [4, 0, 0, 0, 0, 0],
                    value: 2000,
                },
                TakeOption {
                    dices_used: [5, 0, 0, 0, 0, 0],
                    value: 4000,
                },
                TakeOption {
                    dices_used: [6, 0, 0, 0, 0, 0],
                    value: 8000,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(options, expected_hash);
        }

        #[test]
        fn no_value_combinations() {
            let mut options = vec![];
            let expected = vec![];

            for &dice_val in &[2, 3, 4, 6] {
                options.extend(TakeOption::one_face_all_combinations(dice_val, 1));
                options.extend(TakeOption::one_face_all_combinations(dice_val, 2));
            }

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn triples() {
            let mut options = HashSet::new();
            for dice_val in 1..=6 {
                options.extend(TakeOption::one_face_all_combinations(dice_val, 3));
            }

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
                    dices_used: [3, 0, 0, 0, 0, 0],
                    value: 1000,
                },
                TakeOption {
                    dices_used: [0, 3, 0, 0, 0, 0],
                    value: 200,
                },
                TakeOption {
                    dices_used: [0, 0, 3, 0, 0, 0],
                    value: 300,
                },
                TakeOption {
                    dices_used: [0, 0, 0, 3, 0, 0],
                    value: 400,
                },
                TakeOption {
                    dices_used: [0, 0, 0, 0, 1, 0],
                    value: 50,
                },
                TakeOption {
                    dices_used: [0, 0, 0, 0, 2, 0],
                    value: 100,
                },
                TakeOption {
                    dices_used: [0, 0, 0, 0, 3, 0],
                    value: 500,
                },
                TakeOption {
                    dices_used: [0, 0, 0, 0, 0, 3],
                    value: 600,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(options, expected_hash);
        }

        #[test]
        fn quads() {
            let options = TakeOption::one_face_all_combinations(4, 4);
            let expected = [
                TakeOption {
                    dices_used: [0, 0, 0, 3, 0, 0],
                    value: 400,
                },
                TakeOption {
                    dices_used: [0, 0, 0, 4, 0, 0],
                    value: 800,
                },
            ];

            let expected_hash = expected.iter().cloned().collect();

            assert_eq!(options, expected_hash);
        }
    }
}
