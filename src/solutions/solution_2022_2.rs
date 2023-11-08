use super::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let foo: u32 = self
            .input
            .split("\n")
            .filter(str_not_empty)
            .map(session_score)
            .sum();
        Some(foo.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let foo: u32 = self
            .input
            .split("\n")
            .filter(str_not_empty)
            .map(session_score_second)
            .sum();
        Some(foo.to_string())
    }
}

trait Extensions {
    fn code(&self, index: usize) -> u32;
}

impl Extensions for &str {
    fn code(&self, index: usize) -> u32 {
        self.chars().nth(index).unwrap_or_default() as u32
    }
}

fn str_not_empty(string: &&str) -> bool {
    string != &""
}

fn session_score(str: &str) -> u32 {
    ((str.code(2) - str.code(0) + 2) % 3) * 3 + (str.code(2) - 87)
}

fn session_score_second(str: &str) -> u32 {
    (str.code(2) - 88) * 3 + ((str.code(2) + str.code(0) - 1) % 3) + 1
}
