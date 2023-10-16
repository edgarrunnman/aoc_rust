use super::Solution;

pub struct Solution2022_1 {
    pub input: String,
}

impl Solution for Solution2022_1 {
    fn solution_part_1(&self) -> String {
        self.input
            .split("\n\n")
            .map(|g| {
                g.split("\n")
                    .map(|c| c.parse::<u32>().unwrap_or_default())
                    .sum::<u32>()
            })
            .max()
            .unwrap()
            .to_string()
    }
    fn solution_part_2(&self) -> String {
        String::from("second part")
    }
}
