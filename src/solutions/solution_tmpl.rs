use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        None
    }

    fn solution_part_2(&self) -> Option<String> {
        None
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

#[test]
fn test_first() {
    let test_input = "";
    let test_result = "";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "";
    let test_result = "";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
