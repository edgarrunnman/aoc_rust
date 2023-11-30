use crate::Solution;
use std::iter::Iterator;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        find_packet_marker(4, &self.input)
    }

    fn solution_part_2(&self) -> Option<String> {
        find_packet_marker(14, &self.input)
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn find_packet_marker(size: usize, input: &String) -> Option<String> {
    for index in size..input.len() {
        let mut window = input[(index - size)..index]
            .chars()
            .into_iter()
            .collect::<Vec<char>>()
            .clone();
        window.sort();
        window.dedup();
        if window.len() == size {
            return Some(index.to_string());
        };
    }
    None
}
