use super::Solution;
use std::iter::Iterator;

pub struct SolutionImp {
    pub input: String,
}

impl Solution for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        for index in 4..self.input.len() {
            let mut foo = self.input[(index - 4)..index]
                .chars()
                .into_iter()
                .collect::<Vec<char>>()
                .clone();
            foo.sort();
            foo.dedup();
            if foo.len() == 4 {
                return Some(index.to_string());
            };
        }
        None
    }

    fn solution_part_2(&self) -> Option<String> {
        None
    }
}
