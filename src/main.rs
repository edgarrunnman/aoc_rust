mod services;
mod solutions;

extern crate reqwest;

use dotenv::dotenv;
use services::InputDataService;
use solutions::{Solution, SolutionIdentity, SolutionPart};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let input_service = InputDataService::new();

    let id = SolutionIdentity::new(2022, 1);
    let execution = input_service
        .get_input(&id)
        .await
        .and_then(|input| Ok(solutions::solution_2022_1::Solution2022_1 { input }))
        .and_then(|solution| Ok(execute_solution(id.day, solution)));
    execution.unwrap().await;

    let id = SolutionIdentity::new(2022, 2);
    let execution = input_service
        .get_input(&id)
        .await
        .and_then(|input| Ok(solutions::solution_2022_2::Solution2022_2 { input }))
        .and_then(|solution| Ok(execute_solution(id.day, solution)));
    execution.unwrap().await;

    let id = SolutionIdentity::new(2022, 3);
    let execution = input_service
        .get_input(&id)
        .await
        .and_then(|input| Ok(solutions::solution_2022_3::Solution2022_3 { input }))
        .and_then(|solution| Ok(execute_solution(id.day, solution)));
    execution.unwrap().await;
}

async fn execute_solution<S>(day: u32, solution: S)
where
    S: Solution,
{
    let first_part = solution.get_result(SolutionPart::First);
    let second_part = solution.get_result(SolutionPart::Second);
    println!(
        "Day {}, First solution result: {}",
        day,
        first_part.unwrap()
    );
    println!(
        "Day {}, Second solution result: {}",
        day,
        second_part.unwrap()
    );
}
