use std::fmt::Display;

use rand::prelude::*;

#[derive(Debug)]
pub enum ScoredCombination {
    Straight,
    One,
    Five,
    TwoOnes,
    TwoFives,
    Triple(i32),
    Quad(i32),
    Quint(i32),
    Sext(i32),
}

impl ScoredCombination {
    pub fn to_score(self) -> i32 {
        match self {
            ScoredCombination::Straight => 2000, //todo dohozená
            ScoredCombination::One => 100,
            ScoredCombination::Five => 50,
            ScoredCombination::TwoOnes => 200,
            ScoredCombination::TwoFives => 100,
            // ones
            ScoredCombination::Triple(1) => 1000,
            ScoredCombination::Quad(1) => 1000 * 2,
            ScoredCombination::Quint(1) => 1000 * 4,
            ScoredCombination::Sext(1) => 1000 * 8,
            // groups
            ScoredCombination::Triple(v) => v * 100,
            ScoredCombination::Quad(v) => v * 100 * 2,
            ScoredCombination::Quint(v) => v * 100 * 4,
            ScoredCombination::Sext(v) => v * 100 * 8,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dice {
    sides: i32,
    value: i32,
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Dice {
    pub fn new() -> Self {
        Dice { sides: 0, value: 0 }
    }

    pub fn with_sides(sides: i32) -> Self {
        Dice { sides, value: 0 }
    }

    pub fn from_value(value: i32) -> Self {
        Dice { sides: 6, value }
    }

    pub fn from_values(values: &[i32]) -> Vec<Self> {
        values.iter().map(|&val| Dice::from_value(val)).collect()
    }

    pub fn roll(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = rng.gen_range(1..=6);
    }
}

//todo own partialeq
#[derive(Debug, PartialEq)]
pub struct TakeOption {
    dices_used: Vec<i32>,
    value: i32,
}

impl TakeOption {
    pub fn new() -> Self {
        TakeOption {
            dices_used: vec![],
            value: 0,
        }
    }

    pub fn from_combination(dices: &[i32], comb: ScoredCombination) -> Self {
        let dices_used = dices.to_vec();
        let value = comb.to_score();
        TakeOption { dices_used, value }
    }

    pub fn one_dice_all_combinations(dice: i32, n: i32) -> Vec<TakeOption> {
        let mut options = vec![];

        for i in 1..=n {
            match i {
                1 => match dice {
                    1 => options.push(TakeOption::from_combination(
                        &[dice],
                        ScoredCombination::One,
                    )),
                    5 => options.push(TakeOption::from_combination(
                        &[dice],
                        ScoredCombination::Five,
                    )),
                    _ => {}
                },
                2 => match dice {
                    1 => options.push(TakeOption::from_combination(
                        &[dice, dice],
                        ScoredCombination::TwoOnes,
                    )),
                    5 => options.push(TakeOption::from_combination(
                        &[dice, dice],
                        ScoredCombination::TwoFives,
                    )),
                    _ => {}
                },
                3 => options.push(TakeOption::from_combination(
                    &[dice; 3],
                    ScoredCombination::Triple(dice),
                )),
                4 => options.push(TakeOption::from_combination(
                    &[dice; 4],
                    ScoredCombination::Quad(dice),
                )),
                5 => options.push(TakeOption::from_combination(
                    &[dice; 5],
                    ScoredCombination::Quint(dice),
                )),
                6 => options.push(TakeOption::from_combination(
                    &[dice; 6],
                    ScoredCombination::Sext(dice),
                )),
                0 => {}
                _ => panic!("dices_with returned outside of <0;6>"),
            }
        }

        options
    }

    pub fn combine(&self, other: &TakeOption) -> TakeOption {
        let mut new_dices_used = self.dices_used.to_vec();
        new_dices_used.extend(other.dices_used.iter());
        TakeOption {
            dices_used: new_dices_used,
            value: (self.value + other.value),
        }
    }
}

#[derive(Debug)]
pub struct Dices(Vec<Dice>);

impl Display for Dices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut it = self.0.iter();

        match it.next() {
            Some(v) => write!(f, "{}", v)?,
            None => write!(f, "]")?,
        };

        loop {
            match it.next() {
                Some(v) => write!(f, ", {}", v)?,
                None => {write!(f, "]")?; return Ok(())},
            };
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    dices: Dices,
    dice_counts: [i32; 6],
    take_options: Vec<TakeOption>,
}

impl Display for Hand {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dices: {}\nCounts: {:?}\nTakes: {:?}", self.dices, self.dice_counts, self.take_options)?;
        Ok(())
    }
}

impl Hand {
    pub fn new() -> Self {
        let mut dices: Vec<Dice> = Vec::with_capacity(6);
        for _ in 1..=6 {
            dices.push(Dice::with_sides(6));
        }
        Hand {
            dices: Dices(dices),
            dice_counts: Default::default(),
            take_options: vec![],
        }
    }

    pub fn from_dices(dices: &[Dice]) -> Self {
        Hand {
            dices: Dices(dices.to_vec()),
            dice_counts: Default::default(),
            take_options: vec![],
        }
    }

    fn generate_indexes(&mut self) {
        self.dice_counts = Default::default();
        for dice in self.dices.0.iter() {
            let dice_range = 1..=6;
            assert!(dice_range.contains(&dice.value));
            self.dice_counts[(dice.value as usize) - 1] += 1;
        }
    }

    pub fn roll(&mut self) {
        for dice in self.dices.0.iter_mut() {
            dice.roll();
        }

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

        if &self.dice_counts[..] == &[1, 1, 1, 1, 1, 1] {
            options.push(TakeOption::from_combination(
                &[1, 2, 3, 4, 5, 6],
                ScoredCombination::Straight,
            ));
        }

        self.take_options = options
    }

    pub fn takes_intersect(&self, first: &TakeOption, second: &TakeOption) -> bool {
        let mut available_dices = self.dice_counts.to_owned();

        for &dice in first.dices_used.iter() {
            available_dices[(dice as usize) - 1] -= 1;
        }

        for &dice in second.dices_used.iter() {
            available_dices[(dice as usize) - 1] -= 1;
        }

        for &dice_n in &available_dices {
            if dice_n < 0 {
                return true;
            }
        }

        false
    }

    pub fn includes_take(&self, take: &TakeOption) -> bool {
        match self.take_options.iter().find(|&x| x == take) {
            Some(_) => true,
            None => false,
        }
    }

    fn make_combinations(&self) -> Vec<TakeOption> {
        let mut new_combinations = vec![];

        for (i, option) in self.take_options.iter().enumerate() {
            for option_ot in self.take_options.iter().skip(i + 1) {
                if !self.takes_intersect(option, option_ot) {
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

    mod combination_tests {

        use super::*;

        #[test]
        fn combine_one_one_five() {
            let dices = Dice::from_values(&[1, 1, 3, 4, 4, 5]);
            let mut hand = Hand::from_dices(&dices);
            hand.analyze_dices();

            let expected = vec![
                TakeOption {
                    dices_used: vec![1],
                    value: 100,
                },
                TakeOption {
                    dices_used: vec![1, 1],
                    value: 200,
                },
                TakeOption {
                    dices_used: vec![5],
                    value: 50,
                },
                TakeOption {
                    dices_used: vec![1, 5],
                    value: 150,
                },
                TakeOption {
                    dices_used: vec![1, 1, 5],
                    value: 250,
                },
            ];

            println!("hand: {:?}", hand.take_options);

            assert!(hand.take_options.iter().eq(expected.iter()));
        }

        #[test]
        fn combine_two_triples() {
            let dices = Dice::from_values(&[2, 2, 2, 6, 6, 6]);
            let mut hand = Hand::from_dices(&dices);
            hand.analyze_dices();

            let expected = vec![
                TakeOption {
                    dices_used: vec![2, 2, 2],
                    value: 200,
                },
                TakeOption {
                    dices_used: vec![6, 6, 6],
                    value: 600,
                },
                TakeOption {
                    dices_used: vec![2, 2, 2, 6, 6, 6],
                    value: 800,
                },
            ];

            println!("hand: {:?}", hand.take_options);

            assert!(hand.take_options.iter().eq(expected.iter()));
        }
    }

    mod takeoption_tests {

        use super::*;

        #[test]
        fn one() {
            let options = TakeOption::one_dice_all_combinations(1, 1);
            let expected = vec![TakeOption {
                dices_used: vec![1],
                value: 100,
            }];

            assert!(options.iter().eq(expected.iter()));
        }

        #[test]
        fn two_ones() {
            let options = TakeOption::one_dice_all_combinations(1, 2);
            let expected = vec![
                TakeOption {
                    dices_used: vec![1],
                    value: 100,
                },
                TakeOption {
                    dices_used: vec![1, 1],
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
                    dices_used: vec![1],
                    value: 100,
                },
                TakeOption {
                    dices_used: vec![1, 1],
                    value: 200,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1],
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
                    dices_used: vec![1],
                    value: 100,
                },
                TakeOption {
                    dices_used: vec![1, 1],
                    value: 200,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1],
                    value: 1000,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1, 1],
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
                    dices_used: vec![1],
                    value: 100,
                },
                TakeOption {
                    dices_used: vec![1, 1],
                    value: 200,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1],
                    value: 1000,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1, 1],
                    value: 2000,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1, 1, 1],
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
                    dices_used: vec![1],
                    value: 100,
                },
                TakeOption {
                    dices_used: vec![1, 1],
                    value: 200,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1],
                    value: 1000,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1, 1],
                    value: 2000,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1, 1, 1],
                    value: 4000,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1, 1, 1, 1],
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
                    dices_used: vec![1],
                    value: 100,
                },
                TakeOption {
                    dices_used: vec![1, 1],
                    value: 200,
                },
                TakeOption {
                    dices_used: vec![1, 1, 1],
                    value: 1000,
                },
                TakeOption {
                    dices_used: vec![2, 2, 2],
                    value: 200,
                },
                TakeOption {
                    dices_used: vec![3, 3, 3],
                    value: 300,
                },
                TakeOption {
                    dices_used: vec![4, 4, 4],
                    value: 400,
                },
                TakeOption {
                    dices_used: vec![5],
                    value: 50,
                },
                TakeOption {
                    dices_used: vec![5, 5],
                    value: 100,
                },
                TakeOption {
                    dices_used: vec![5, 5, 5],
                    value: 500,
                },
                TakeOption {
                    dices_used: vec![6, 6, 6],
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
                    dices_used: vec![4, 4, 4],
                    value: 400,
                },
                TakeOption {
                    dices_used: vec![4, 4, 4, 4],
                    value: 800,
                },
            ];

            assert!(options.iter().eq(expected.iter()));
        }
    }
}
