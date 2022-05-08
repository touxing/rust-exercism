#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self(scores.to_vec())
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        // self.0.last().map(|x| *x)
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut result = self.0.to_vec();
        result.sort_unstable_by(|a, b| b.cmp(a));
        result.truncate(3);
        result
    }
}
