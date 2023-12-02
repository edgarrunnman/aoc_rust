use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let result: u32 = self
            .input
            .split("\n")
            .map(|line| {
                line.chars()
                    .filter(|char| char.is_numeric())
                    .collect::<Vec<char>>()
            })
            .filter(|line| line.len() > 0)
            .map(|line| {
                let first = line.first().unwrap();
                let last = line.last().unwrap();
                format!("{}{}", first, last).parse::<u32>().unwrap()
            })
            .sum();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let result: u32 = self
            .input
            .split("\n")
            .map(|line| {
                line.replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "tree3tree")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
                    .chars()
                    .filter(|char| char.is_numeric())
                    .collect::<Vec<char>>()
            })
            .filter(|line| line.len() > 0)
            .map(|line| {
                let first = line.first().unwrap();
                let last = line.last().unwrap();
                format!("{}{}", first, last).parse::<u32>().unwrap()
            })
            .sum();
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

#[test]
fn test_first() {
    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let test_result = "142";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}

#[test]
fn test_second() {
    let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let test_result = "281";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
