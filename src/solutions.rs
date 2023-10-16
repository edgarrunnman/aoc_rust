pub mod solution_2022_1;
mod solution_2022_2;

pub trait Solution {
    fn solution_part_1(&self) -> String;
    fn solution_part_2(&self) -> String;

    fn get_result(&self, part: SolutionPart) -> String {
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
