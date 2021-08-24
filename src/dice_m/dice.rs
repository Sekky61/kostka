use rand::prelude::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dice {
    sides: i32,
    pub value: i32,
}

impl Dice {
    pub fn new() -> Self {
        Dice { sides: 6, value: 0 }
    }

    pub fn with_sides(sides: i32) -> Self {
        Dice { sides, value: 0 }
    }

    pub fn from_value(value: i32) -> Self {
        Dice { sides: 6, value }
    }

    pub fn roll(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = rng.gen_range(1..=6);
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

#[derive(Debug, PartialEq)]
pub struct Dices(Vec<Dice>);

impl Dices {
    pub fn new() -> Self {
        Dices(vec![])
    }

    pub fn combine(&self, other: &Self) -> Self {
        let mut v = self.0.clone();
        v.extend(other.0.iter());
        Dices(v)
    }

    pub fn as_slice(&self) -> &[Dice] {
        self.0.as_slice()
    }

    pub fn roll(&mut self) {
        for dice in self.0.iter_mut() {
            dice.roll();
        }
    }
}

impl Default for Dices {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> std::iter::IntoIterator for &'a Dices {
    type Item = <std::slice::Iter<'a, Dice> as Iterator>::Item;
    type IntoIter = std::slice::Iter<'a, Dice>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_slice().iter()
    }
}

impl From<Vec<Dice>> for Dices {
    fn from(v: Vec<Dice>) -> Self {
        Dices(v)
    }
}

impl From<&[i32]> for Dices {
    fn from(s: &[i32]) -> Self {
        let v = s.iter().map(|&val| Dice::from_value(val)).collect();
        Dices(v)
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
                let dices = Dices::from(&[1, 2, 3][..]);

                for (i, dice) in dices.into_iter().enumerate() {
                    assert_eq!(i + 1, (dice.value as usize));
                }

                // dices not consumed
                println!("dices: {}", dices);
            }

            #[test]
            fn next() {
                let dices = Dices::from(&[1, 2, 3][..]);

                let mut it = dices.into_iter();
                assert_eq!(it.next(), Some(&Dice::from_value(1)));
                assert_eq!(it.next(), Some(&Dice::from_value(2)));
                assert_eq!(it.next(), Some(&Dice::from_value(3)));
                assert_eq!(it.next(), None);
            }
        }
    }
}
