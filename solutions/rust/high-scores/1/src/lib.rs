use std::{collections::BinaryHeap, vec};
#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut v_scores = Vec::new();
        for s in scores {
            v_scores.push(*s);
        }
        HighScores { scores: v_scores }
    }

    pub fn scores(&self) -> &[u32] {
        // unimplemented!("Return all the scores as a slice")
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let last = self.scores.last();
        match last {
            Some(v) => Some(*v),
            None => None,
        }
        // unimplemented!("Return the latest (last) score")
    }

    pub fn personal_best(&self) -> Option<u32> {
        // unimplemented!("Return the highest score")
        let max = self.scores.iter().max();
        match max {
            Some(v) => Some(*v),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // unimplemented!("Return 3 highest scores")
        let mut heap = BinaryHeap::from_iter(self.scores.iter());
        let mut ans = Vec::with_capacity(3);
        for _ in 0..3 {
            let v = heap.pop();
            match v {
                Some(i) => ans.push(*i),
                None => {}
            }
        }
        ans
    }
}