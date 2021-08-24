use super::Dices;
use super::ScoredCombination;

//todo own partialeq
#[derive(Debug, PartialEq)]
pub struct TakeOption {
    pub dices_used: Dices,
    pub value: i32,
}

impl TakeOption {
    pub fn new() -> Self {
        TakeOption {
            dices_used: Dices::new(),
            value: 0,
        }
    }

    pub fn from_combination(dices_used: Dices, comb: ScoredCombination) -> Self {
        let value = comb.to_score();
        TakeOption { dices_used, value }
    }

    pub fn one_dice_all_combinations(dice: i32, n: i32) -> Vec<TakeOption> {
        let mut options = vec![];

        for i in 1..=n {
            match i {
                1 => match dice {
                    1 => options.push(TakeOption::from_combination(
                        Dices::from(&[dice][..]),
                        ScoredCombination::One,
                    )),
                    5 => options.push(TakeOption::from_combination(
                        Dices::from(&[dice][..]),
                        ScoredCombination::Five,
                    )),
                    _ => {}
                },
                2 => match dice {
                    1 => options.push(TakeOption::from_combination(
                        Dices::from(&[dice, dice][..]),
                        ScoredCombination::TwoOnes,
                    )),
                    5 => options.push(TakeOption::from_combination(
                        Dices::from(&[dice, dice][..]),
                        ScoredCombination::TwoFives,
                    )),
                    _ => {}
                },
                3 => options.push(TakeOption::from_combination(
                    Dices::from(&[dice; 3][..]),
                    ScoredCombination::Triple(dice),
                )),
                4 => options.push(TakeOption::from_combination(
                    Dices::from(&[dice; 4][..]),
                    ScoredCombination::Quad(dice),
                )),
                5 => options.push(TakeOption::from_combination(
                    Dices::from(&[dice; 5][..]),
                    ScoredCombination::Quint(dice),
                )),
                6 => options.push(TakeOption::from_combination(
                    Dices::from(&[dice; 6][..]),
                    ScoredCombination::Sext(dice),
                )),
                0 => {}
                _ => panic!("dices_with returned outside of <0;6>"),
            }
        }

        options
    }

    pub fn combine(&self, other: &TakeOption) -> TakeOption {
        let new_dices = self.dices_used.combine(&other.dices_used);
        TakeOption {
            dices_used: new_dices,
            value: (self.value + other.value),
        }
    }
}

impl Default for TakeOption {
    fn default() -> Self {
        Self::new()
    }
}
