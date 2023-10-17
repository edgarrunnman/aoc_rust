mod services;
mod solutions;

extern crate reqwest;

use dotenv::dotenv;
use services::InputDataService;
use solutions::{Solution, SolutionPart};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let input_service = InputDataService::new();
    let year = 2022;
    let day = 1;
    let input = input_service
        .get_input(year.to_string(), day.to_string())
        .await
        .unwrap();
    let solution2 = solutions::solution_2022_1::Solution2022_1 { input };
    let first_part = solution2.get_result(SolutionPart::First);
    let second_part = solution2.get_result(SolutionPart::Second);
    println!("First solution result: {}", first_part.unwrap());
    println!("First solution result: {}", second_part.unwrap());
}
