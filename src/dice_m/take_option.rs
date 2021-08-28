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

    pub fn dices_count(&self) -> usize {
        self.dices_used.iter().sum::<i32>() as usize
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

#[cfg(test)]
mod tests {

    use super::TakeOption;
    use std::collections::HashSet;

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

        assert_eq!(options, expected_hash);
    }

    #[test]
    fn three_ones() {
        let options = TakeOption::one_face_all_combinations(1, 3);
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
        ];

        let expected_hash = expected.iter().cloned().collect();

        assert_eq!(options, expected_hash);
    }

    #[test]
    fn four_ones() {
        let options = TakeOption::one_face_all_combinations(1, 4);
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
        let mut options = HashSet::new();
        let expected = HashSet::new();

        for &dice_val in &[2, 3, 4, 6] {
            options.extend(TakeOption::one_face_all_combinations(dice_val, 1));
            options.extend(TakeOption::one_face_all_combinations(dice_val, 2));
        }

        assert_eq!(options, expected);
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
