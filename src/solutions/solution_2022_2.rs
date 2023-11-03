use super::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let foo: u32 = self
            .input
            .split("\n")
            .filter(|c| c != &"")
            .map(|c| ((c.code(2) - c.code(0) + 2) % 3) * 3 + (c.code(2) - 87))
            .sum();
        Some(foo.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let foo: u32 = self
            .input
            .split("\n")
            .filter(|c| c != &"")
            .map(|c| (c.code(2) - 88) * 3 + ((c.code(2) + c.code(0) - 1) % 3) + 1)
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
