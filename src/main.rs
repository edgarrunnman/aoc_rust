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
        .and_then(|input| Ok(solutions::solution_2022_1::SolutionImp { input }))
        .and_then(|solution| Ok(execute_solution(id.day, solution)));
    execution.unwrap().await;

    let id = SolutionIdentity::new(2022, 2);
    let execution = input_service
        .get_input(&id)
        .await
        .and_then(|input| Ok(solutions::solution_2022_2::SolutionImp { input }))
        .and_then(|solution| Ok(execute_solution(id.day, solution)));
    execution.unwrap().await;

    let id = SolutionIdentity::new(2022, 3);
    let execution = input_service
        .get_input(&id)
        .await
        .and_then(|input| Ok(solutions::solution_2022_3::SolutionImp { input }))
        .and_then(|solution| Ok(execute_solution(id.day, solution)));
    execution.unwrap().await;

    let id = SolutionIdentity::new(2022, 4);
    let execution = input_service
        .get_input(&id)
        .await
        .and_then(|input| Ok(solutions::solution_2022_4::SolutionImp { input }))
        .and_then(|solution| Ok(execute_solution(id.day, solution)));
    execution.unwrap().await;

    let id = SolutionIdentity::new(2022, 5);
    let execution = input_service
        .get_input(&id)
        .await
        .and_then(|input| Ok(solutions::solution_2022_5::SolutionImp { input }))
        .and_then(|solution| Ok(execute_solution(id.day, solution)));
    execution.unwrap().await;

    let id = SolutionIdentity::new(2022, 6);
    let execution = input_service
        .get_input(&id)
        .await
        .and_then(|input| Ok(solutions::solution_2022_6::SolutionImp { input }))
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
        first_part.unwrap_or_else(return_not_imp)
    );
    println!(
        "Day {}, Second solution result: {}",
        day,
        second_part.unwrap_or_else(return_not_imp)
    );
}
fn return_not_imp() -> String {
    String::from("not implemented")
}
