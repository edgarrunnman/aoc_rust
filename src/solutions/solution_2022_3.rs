use super::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let result: u32 = self
            .input
            .lines()
            .map(split_half)
            .map(|split| {
                get_common_chars_2(split)
                    .and_then(|comon| comon.chars().nth(0))
                    .and_then(get_priority_value)
                    .unwrap_or_default()
            })
            .sum();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let result: u32 = line_group(self.input.lines().collect::<Vec<&str>>())
            .iter()
            .map(|group| {
                let first_common = get_common_chars(String::from(group.0), String::from(group.1));
                get_common_chars(first_common.unwrap_or_default(), String::from(group.2))
                    .and_then(|comon| comon.chars().nth(0))
                    .and_then(get_priority_value)
                    .unwrap_or_default()
            })
            .sum();
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn get_common_chars(first: String, second: String) -> Option<String> {
    Some(
        first
            .chars()
            .filter(|&char| second.contains(char))
            .collect::<String>(),
    )
}
fn get_common_chars_2(pair: (&str, &str)) -> Option<String> {
    Some(
        pair.0
            .chars()
            .filter(|&char| pair.1.contains(char))
            .collect::<String>(),
    )
}

fn get_priority_value(char: char) -> Option<u32> {
    let n = char as u32;
    if n > 96 {
        Some(n - 96)
    } else {
        Some(n - 38)
    }
}

fn split_half(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn line_group(lines: Vec<&str>) -> Vec<(&str, &str, &str)> {
    lines
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect()
}
