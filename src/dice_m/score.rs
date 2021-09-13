#[derive(Debug)]
pub enum ScoredCombination {
    Straight,
    One,
    Five,
    TwoOnes,
    TwoFives,
    Triple(u32),
    Quad(u32),
    Quint(u32),
    Sext(u32),
}

impl ScoredCombination {
    pub fn to_score(&self) -> u32 {
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
