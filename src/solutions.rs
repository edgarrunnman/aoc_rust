pub mod year_2022;
pub mod year_2023;

pub trait Solution<T> {
    fn new(input: String) -> T;
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
