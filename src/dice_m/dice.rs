use rand::prelude::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Dice {
    sides: i32,
    pub value: i32,
}

impl Dice {
    // Dice with 6 sides and random value
    pub fn new() -> Self {
        let mut dice = Dice { sides: 6, value: 0 };
        dice.roll();
        dice
    }

    pub fn from_value(value: i32) -> Self {
        Dice { sides: 6, value }
    }

    fn roll_internal(&self) -> i32 {
        rand::thread_rng().gen_range(1..=self.sides)
    }

    pub fn roll(&mut self) {
        self.value = self.roll_internal();
    }
}

impl Default for Dice {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dices(Vec<Dice>); // must be vec, 2 ones cant be represented

impl Dices {
    pub fn new() -> Self {
        Dices(Default::default())
    }

    pub fn of_length(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(Dice::default());
        }
        Dices(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Dice> {
        //todo ref?
        self.0.iter()
    }
}

impl Default for Dices {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> From<[i32; N]> for Dices {
    fn from(arr: [i32; N]) -> Self {
        let s = arr.iter().map(|&i| Dice::from_value(i)).collect();
        Dices(s)
    }
}

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
                None => {
                    write!(f, "]")?;
                    return Ok(());
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod dices {
        use super::*;

        mod iter_tests {
            use super::*;

            #[test]
            fn for_loop_not_consumed() {
                let dices = Dices::from([1, 2, 3]);
                let dice_range = 1..=6;

                for dice in dices.iter() {
                    assert!(dice_range.contains(&dice.value));
                }

                // dices not consumed
                println!("dices: {}", dices);
            }

            #[test]
            fn next() {
                let dices = Dices::from([1]);

                let mut it = dices.iter();
                assert_eq!(it.next(), Some(&Dice::from_value(1)));
                assert_eq!(it.next(), None);
            }
        }
    }
}
