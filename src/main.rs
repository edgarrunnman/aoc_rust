extern crate reqwest;

use dotenv::dotenv;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let input_service = InputDataService::new();
    let input = input_service.get_input(String::from("2022"), String::from("1"));
    // let mut file = File::create("inputs/foo.txt").unwrap();
    //
    // file.write(input.await.as_bytes());
    //
    // let foo = Local::now();
    println!("{}", input.await);
}

struct InputDataService {
    root_url: String,
    client: reqwest::Client,
    session_token: String,
}

impl InputDataService {
    fn new() -> InputDataService {
        let root_url = String::from("https://adventofcode.com");
        let client = reqwest::Client::new();
        let session_token = std::env::var("SESSION_TOKEN").expect("");
        InputDataService {
            root_url,
            client,
            session_token,
        }
    }
    async fn get_input(&self, year: String, day: String) -> String {
        let h = calculate_hash(&self.session_token);
        let file_name = format!("{}_{}_{}.txt", h, year, day);
        let mut _input = String::new();
        let file = OpenOptions::new().read(true).open(&file_name).as_mut();

        match file {
            Ok(x) => {
                x.read_to_string(&mut _input);
            }
            Err(_) => {
                let url = format!("{}/{}/day/{}/input", &self.root_url, year, day);
                let session_cookie = format!("session={}", &self.session_token);
                let res = &self
                    .client
                    .get(url)
                    .header("cookie", session_cookie)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let mut file = File::create(file_name).unwrap();
                _input = res.to_string();
                file.write(res.to_string().as_bytes());
            }
        };
        // let url = format!("{}/{}/day/{}/input", &self.root_url, year, day);
        // let session_cookie = format!("session={}", &self.session_token);
        // let res = &self
        //     .client
        //     .get(url)
        //     .header("cookie", session_cookie)
        //     .send()
        //     .await
        //     .unwrap()
        //     .text()
        //     .await
        //     .unwrap();
        // res.to_string()
        _input
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
