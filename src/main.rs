mod services;
mod solutions;

extern crate reqwest;

use dotenv::dotenv;
use services::InputDataService;
use solutions::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let input_service = InputDataService::new();

    year_2023::day_2::test_first();
    year_2023::day_2::test_second();
    //
    execute_day::<year_2023::day_2::SolutionImp>(&input_service, 2023, 2).await;
}

async fn execute_day<S>(input_service: &InputDataService, year: u16, day: u16)
where
    S: Solution<S>,
{
    let solutions = input_service
        .get_input(year, day)
        .await
        .and_then(|input| Ok(S::new(input)))
        .and_then(|solution| {
            Ok((
                solution
                    .get_result(SolutionPart::First)
                    .unwrap_or(not_imp()),
                solution
                    .get_result(SolutionPart::Second)
                    .unwrap_or(not_imp()),
            ))
        })
        .unwrap();
    println!("{}-{}, First solution result: {}", year, day, solutions.0);
    println!("{}-{}, Second solution result: {}", year, day, solutions.1);
}
fn not_imp() -> String {
    String::from("not implemented")
}
