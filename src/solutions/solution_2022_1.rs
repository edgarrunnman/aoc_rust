use super::Solution;

pub struct Solution2022_1 {
    pub input: String,
}

impl Solution for Solution2022_1 {
    fn solution_part_1(&self) -> Option<String> {
        self.input
            .split("\n\n")
            .map(|g| {
                g.split("\n")
                    .map(|c| c.parse::<u32>().unwrap_or_default())
                    .sum::<u32>()
            })
            .max()
            .map(|n| n.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let mut result = self
            .input
            .split("\n\n")
            .map(|g| {
                g.split("\n")
                    .map(|c| c.parse::<u32>().unwrap_or_default())
                    .sum::<u32>()
            })
            .collect::<Vec<u32>>();
        result.sort();
        Some(result.iter().rev().take(3).sum::<u32>().to_string())
    }
}
