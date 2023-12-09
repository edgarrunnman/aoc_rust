use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let result = self
            .input
            .lines()
            .map(|line| {
                line.split(" ")
                    .into_iter()
                    .map(|value| value.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .map(|s| prediction(&s))
            .sum::<i64>();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let result = self
            .input
            .lines()
            .map(|line| {
                line.split(" ")
                    .into_iter()
                    .map(|value| value.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .map(|s| prediction(&s.into_iter().rev().collect()))
            .sum::<i64>();
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn prediction(seq: &Vec<i64>) -> i64 {
    let mut seq = seq.clone();
    let mut sum = seq.last().unwrap().clone();
    while !seq.clone().into_iter().all(|value| value == 0) {
        seq = sub_seq(&seq);
        sum = seq.last().unwrap().clone() + sum;
    }
    sum
}

fn sub_seq(seq: &Vec<i64>) -> Vec<i64> {
    seq.windows(2)
        .map(|values| values[1] - values[0])
        .collect::<Vec<i64>>()
}

#[test]
fn test_first() {
    let test_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let test_result = "114";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let test_result = "2";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
