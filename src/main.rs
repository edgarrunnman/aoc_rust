mod services;
mod solutions;

extern crate reqwest;

use std::time::SystemTime;

use dotenv::dotenv;
use services::InputDataService;
use solutions::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let input_service = InputDataService::new();

    // execute_day::<year_2022::day_1::SolutionImp>(&input_service, 2022, 1).await;
    // execute_day::<year_2022::day_2::SolutionImp>(&input_service, 2022, 2).await;
    // execute_day::<year_2022::day_3::SolutionImp>(&input_service, 2022, 3).await;
    // execute_day::<year_2022::day_4::SolutionImp>(&input_service, 2022, 4).await;
    // execute_day::<year_2022::day_5::SolutionImp>(&input_service, 2022, 5).await;
    // execute_day::<year_2022::day_6::SolutionImp>(&input_service, 2022, 6).await;
    //
    // execute_day::<year_2023::day_1::SolutionImp>(&input_service, 2023, 1).await;
    // execute_day::<year_2023::day_2::SolutionImp>(&input_service, 2023, 2).await;
    // execute_day::<year_2023::day_3::SolutionImp>(&input_service, 2023, 3).await;
    // execute_day::<year_2023::day_4::SolutionImp>(&input_service, 2023, 4).await;
    // execute_day::<year_2023::day_5::SolutionImp>(&input_service, 2023, 5).await;
    // execute_day::<year_2023::day_6::SolutionImp>(&input_service, 2023, 6).await;
    // execute_day::<year_2023::day_7::SolutionImp>(&input_service, 2023, 7).await;
    // execute_day::<year_2023::day_8::SolutionImp>(&input_service, 2023, 8).await;
    // execute_day::<year_2023::day_9::SolutionImp>(&input_service, 2023, 9).await;
    // execute_day::<year_2023::day_10::SolutionImp>(&input_service, 2023, 10).await;
    // execute_day::<year_2023::day_11::SolutionImp>(&input_service, 2023, 11).await;
    // execute_day::<year_2023::day_12::SolutionImp>(&input_service, 2023, 12).await;
    // execute_day::<year_2023::day_13::SolutionImp>(&input_service, 2023, 13).await;
    execute_day::<year_2023::day_14::SolutionImp>(&input_service, 2023, 14).await;
    execute_day::<year_2023::day_15::SolutionImp>(&input_service, 2023, 15).await;
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
            let start_time = SystemTime::now();

            let result_first = solution
                .get_result(SolutionPart::First)
                .unwrap_or(not_imp());
            let end_time_first = SystemTime::now();

            let result_second = solution
                .get_result(SolutionPart::Second)
                .unwrap_or(not_imp());
            let end_time_second = SystemTime::now();

            let duration_first = end_time_first.duration_since(start_time).unwrap();
            let duration_second = end_time_second.duration_since(end_time_first).unwrap();
            Ok((
                (result_first, duration_first),
                (result_second, duration_second),
            ))
        })
        .unwrap();

    println!(
        "{}-{}, First solution result: {} (duration: {} ms)",
        year,
        day,
        solutions.0 .0,
        solutions.0 .1.as_millis()
    );

    println!(
        "{}-{}, Second solution result: {} (duration: {} ms)",
        year,
        day,
        solutions.1 .0,
        solutions.1 .1.as_millis()
    );
}
fn not_imp() -> String {
    String::from("not implemented")
}
