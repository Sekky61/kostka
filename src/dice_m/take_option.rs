use std::collections::HashSet;

use super::ScoredCombination;

//todo own partialeq
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TakeOption {
    pub dices_used: [i32; 6],
    pub value: i32,
}

impl TakeOption {
    pub fn new() -> Self {
        TakeOption {
            dices_used: Default::default(),
            value: Default::default(),
        }
    }

    pub fn from_combination(dices_used: [i32; 6], comb: ScoredCombination) -> Self {
        let value = comb.to_score();
        TakeOption { dices_used, value }
    }

    pub fn one_face_all_combinations(face: i32, n: i32) -> HashSet<TakeOption> {
        let mut options = HashSet::new();

        for i in 1..=n {
            match i {
                1 => match face {
                    1 => options.insert(TakeOption::from_combination(
                        [1, 0, 0, 0, 0, 0],
                        ScoredCombination::One,
                    )),
                    5 => options.insert(TakeOption::from_combination(
                        [0, 0, 0, 0, 1, 0],
                        ScoredCombination::Five,
                    )),
                    _ => false,
                },
                2 => match face {
                    1 => options.insert(TakeOption::from_combination(
                        [2, 0, 0, 0, 0, 0],
                        ScoredCombination::TwoOnes,
                    )),
                    5 => options.insert(TakeOption::from_combination(
                        [0, 0, 0, 0, 2, 0],
                        ScoredCombination::TwoFives,
                    )),
                    _ => false,
                },
                3 => {
                    let mut dices_used = [0; 6];
                    dices_used[(face as usize) - 1] = 3;
                    options.insert(TakeOption::from_combination(
                        dices_used,
                        ScoredCombination::Triple(face),
                    ))
                }
                4 => {
                    let mut dices_used = [0; 6];
                    dices_used[(face as usize) - 1] = 4;
                    options.insert(TakeOption::from_combination(
                        dices_used,
                        ScoredCombination::Quad(face),
                    ))
                }
                5 => {
                    let mut dices_used = [0; 6];
                    dices_used[(face as usize) - 1] = 5;
                    options.insert(TakeOption::from_combination(
                        dices_used,
                        ScoredCombination::Quint(face),
                    ))
                }
                6 => {
                    let mut dices_used = [0; 6];
                    dices_used[(face as usize) - 1] = 6;
                    options.insert(TakeOption::from_combination(
                        dices_used,
                        ScoredCombination::Sext(face),
                    ))
                }
                0 => false,
                _ => panic!("dices_with returned outside of <0;6>"),
            };
        }

        options
    }

    pub fn combine(&self, other: &TakeOption) -> TakeOption {
        let mut dices_used: [i32; 6] = Default::default();
        for (i, (aval, bval)) in self.dices_used.iter().zip(&other.dices_used).enumerate() {
            dices_used[i] = aval + bval;
        }
        TakeOption {
            dices_used,
            value: (self.value + other.value),
        }
    }
}

impl Default for TakeOption {
    fn default() -> Self {
        Self::new()
    }
}
