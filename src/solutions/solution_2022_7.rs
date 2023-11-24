use super::Solution;

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
