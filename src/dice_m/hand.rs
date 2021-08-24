use super::{Dice, Dices, ScoredCombination, TakeOption};
use std::fmt::Display;

#[derive(Debug)]
pub struct Hand {
    dices: Dices,
    dice_counts: [i32; 6],
    take_options: Vec<TakeOption>,
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

impl Hand {
    pub fn new() -> Self {
        let mut dices = Vec::with_capacity(6);
        for _ in 0..6 {
            dices.push(Dice::with_sides(6));
        }
        Hand {
            dices: Dices::from(dices),
            dice_counts: Default::default(),
            take_options: vec![],
        }
    }

    fn generate_indexes(&mut self) {
        self.dice_counts = Default::default();
        for dice in self.dices.into_iter() {
            let dice_range = 1..=6; // todo assert on dice
            self.dice_counts[(dice.value as usize) - 1] += 1;
        }
    }

    pub fn roll(&mut self) {
        self.dices.roll();

        self.analyze_dices();
    }

    pub fn analyze_dices(&mut self) {
        self.generate_indexes();
        self.take_basic_options();
        self.combine_options();
    }

    fn dices_with(&self, value: i32) -> i32 {
        let dice_range = 1..=6;
        assert!(dice_range.contains(&value));
        self.dice_counts[(value as usize) - 1]
    }

    pub fn take_basic_options(&mut self) {
        let mut options: Vec<TakeOption> = vec![];

        for i in 1..=6 {
            let n_of_dices = self.dices_with(i);
            options.extend(TakeOption::one_dice_all_combinations(i, n_of_dices));
        }

        if self.dice_counts == [1, 1, 1, 1, 1, 1] {
            options.push(TakeOption::from_combination(
                Dices::from(&[1, 2, 3, 4, 5, 6][..]),
                ScoredCombination::Straight,
            ));
        }

        self.take_options = options
    }

    pub fn takes_overlap(&self, first: &TakeOption, second: &TakeOption) -> bool {
        let mut available_dices = self.dice_counts.to_owned();

        for dice in first
            .dices_used
            .into_iter()
            .chain(second.dices_used.into_iter())
        {
            available_dices[(dice.value as usize) - 1] -= 1;
        }

        available_dices.iter().any(|&n| n < 0)
    }

    pub fn includes_take(&self, take: &TakeOption) -> bool {
        self.take_options.iter().any(|x| x == take)
    }

    fn make_combinations(&self) -> Vec<TakeOption> {
        let mut new_combinations = vec![];

        for (i, option) in self.take_options.iter().enumerate() {
            for option_ot in self.take_options.iter().skip(i + 1) {
                if !self.takes_overlap(option, option_ot) {
                    let new_option = option.combine(option_ot);
                    if !self.includes_take(&new_option) {
                        new_combinations.push(new_option);
                    }
                }
            }
        }

        new_combinations
    }

    pub fn combine_options(&mut self) {
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

#[cfg(test)]
mod tests {

    use super::*;

    fn hand_from_dices(dices: &[Dice]) -> Hand {
        Hand {
            dices: Dices::from(dices.to_vec()),
            dice_counts: Default::default(),
            take_options: vec![],
        }
    }

    mod combination_tests {

        use super::*;

        #[test]
        fn combine_one_one_five() {
            let dices = Dices::from(&[1, 1, 3, 4, 4, 5][..]);
            let mut hand = hand_from_dices(dices.as_slice());
            hand.analyze_dices();

            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[1][..]),
                    value: 100,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1][..]),
                    value: 200,
                },
                TakeOption {
                    dices_used: Dices::from(&[5][..]),
                    value: 50,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 5][..]),
                    value: 150,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 5][..]),
                    value: 250,
                },
            ];

            assert!(hand.take_options.iter().eq(expected.iter()));
        }

        #[test]
        fn combine_two_triples() {
            let dices = Dices::from(&[2, 2, 2, 6, 6, 6][..]);
            let mut hand = hand_from_dices(dices.as_slice());
            hand.analyze_dices();

            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[2, 2, 2][..]),
                    value: 200,
                },
                TakeOption {
                    dices_used: Dices::from(&[6, 6, 6][..]),
                    value: 600,
                },
                TakeOption {
                    dices_used: Dices::from(&[2, 2, 2, 6, 6, 6][..]),
                    value: 800,
                },
            ];

            assert!(hand.take_options.iter().eq(expected.iter()));
        }
    }

    mod takeoption_tests {

        use super::*;

        #[test]
        fn one() {
            let options = TakeOption::one_dice_all_combinations(1, 1);
            let expected = vec![TakeOption {
                dices_used: Dices::from(&[1][..]),
                value: 100,
            }];

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn two_ones() {
            let options = TakeOption::one_dice_all_combinations(1, 2);
            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[1][..]),
                    value: 100,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1][..]),
                    value: 200,
                },
            ];

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn three_ones() {
            let options = TakeOption::one_dice_all_combinations(1, 3);
            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[1][..]),
                    value: 100,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1][..]),
                    value: 200,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1][..]),
                    value: 1000,
                },
            ];

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn four_ones() {
            let options = TakeOption::one_dice_all_combinations(1, 4);
            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[1][..]),
                    value: 100,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1][..]),
                    value: 200,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1][..]),
                    value: 1000,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1, 1][..]),
                    value: 2000,
                },
            ];

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn five_ones() {
            let options = TakeOption::one_dice_all_combinations(1, 5);
            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[1][..]),
                    value: 100,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1][..]),
                    value: 200,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1][..]),
                    value: 1000,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1, 1][..]),
                    value: 2000,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1, 1, 1][..]),
                    value: 4000,
                },
            ];

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn six_ones() {
            let options = TakeOption::one_dice_all_combinations(1, 6);
            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[1][..]),
                    value: 100,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1][..]),
                    value: 200,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1][..]),
                    value: 1000,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1, 1][..]),
                    value: 2000,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1, 1, 1][..]),
                    value: 4000,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1, 1, 1, 1][..]),
                    value: 8000,
                },
            ];

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn no_value_combinations() {
            let mut options = vec![];
            let expected = vec![];

            for &dice_val in &[2, 3, 4, 6] {
                options.extend(TakeOption::one_dice_all_combinations(dice_val, 1));
                options.extend(TakeOption::one_dice_all_combinations(dice_val, 2));
            }

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn triples() {
            let mut options = vec![];
            for dice_val in 1..=6 {
                options.extend(TakeOption::one_dice_all_combinations(dice_val, 3));
            }

            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[1][..]),
                    value: 100,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1][..]),
                    value: 200,
                },
                TakeOption {
                    dices_used: Dices::from(&[1, 1, 1][..]),
                    value: 1000,
                },
                TakeOption {
                    dices_used: Dices::from(&[2, 2, 2][..]),
                    value: 200,
                },
                TakeOption {
                    dices_used: Dices::from(&[3, 3, 3][..]),
                    value: 300,
                },
                TakeOption {
                    dices_used: Dices::from(&[4, 4, 4][..]),
                    value: 400,
                },
                TakeOption {
                    dices_used: Dices::from(&[5][..]),
                    value: 50,
                },
                TakeOption {
                    dices_used: Dices::from(&[5, 5][..]),
                    value: 100,
                },
                TakeOption {
                    dices_used: Dices::from(&[5, 5, 5][..]),
                    value: 500,
                },
                TakeOption {
                    dices_used: Dices::from(&[6, 6, 6][..]),
                    value: 600,
                },
            ];

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn quads() {
            let options = TakeOption::one_dice_all_combinations(4, 4);
            let expected = vec![
                TakeOption {
                    dices_used: Dices::from(&[4, 4, 4][..]),
                    value: 400,
                },
                TakeOption {
                    dices_used: Dices::from(&[4, 4, 4, 4][..]),
                    value: 800,
                },
            ];

            assert!(options.iter().eq(expected.iter()));
        }
    }
}
