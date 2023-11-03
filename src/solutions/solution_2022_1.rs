use super::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        self.input
            .split("\n\n")
            .map(|g| g.sum_of_group())
            .max()
            .map(|n| n.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let mut result = self
            .input
            .split("\n\n")
            .map(|group| group.sum_of_group())
            .collect::<Vec<u32>>();
        result.sort();
        Some(result.iter().rev().take(3).sum::<u32>().to_string())
    }
}

trait Extensions {
    fn sum_of_group(&self) -> u32;
}

impl Extensions for &str {
    fn sum_of_group(&self) -> u32 {
        self.split("\n")
            .map(|c| c.parse::<u32>().unwrap_or_default())
            .sum()
    }
}
