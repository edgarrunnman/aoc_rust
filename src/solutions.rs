pub mod solution_2022_1;
pub mod solution_2022_2;
pub mod solution_2022_3;
pub mod solution_2022_4;
pub mod solution_2022_5;
pub mod solution_2022_6;
pub mod solution_2022_7;

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
