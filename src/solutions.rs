pub mod solution_2022_1;
pub mod solution_2022_2;
pub mod solution_2022_3;

pub struct SolutionIdentity {
    pub year: u32,
    pub day: u32,
}

impl SolutionIdentity {
    pub fn new(year: u32, day: u32) -> SolutionIdentity {
        SolutionIdentity { year, day }
    }
}

pub trait Solution {
    fn solution_part_1(&self) -> Option<String>;
    fn solution_part_2(&self) -> Option<String>;

    fn get_result(&self, part: SolutionPart) -> Option<String> {
        match part {
            SolutionPart::First => self.solution_part_1(),
            SolutionPart::Second => self.solution_part_2(),
        }
    }
}

#[derive(Debug)]
pub enum SolutionPart {
    First,
    Second,
}
