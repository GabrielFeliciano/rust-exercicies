#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let last_score = self.scores.last()?;
        Some(last_score.clone())
    }

    pub fn personal_best(&self) -> Option<u32> {
        let best_score = self.scores
            .iter()
            .reduce(|a, b| if a > b { a } else { b })?;
        Some(best_score.clone())
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores: Vec<_> = self.scores.iter().cloned().collect();
        sorted_scores.sort_unstable();
        sorted_scores.reverse();
        sorted_scores.truncate(3);

        sorted_scores
    }
}
