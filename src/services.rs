use std::collections::hash_map::DefaultHasher;
use std::fs::create_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::io::Write;

pub struct InputDataService {
    root_url: String,
    client: reqwest::Client,
    session_token: String,
}

impl InputDataService {
    pub fn new() -> InputDataService {
        let root_url = String::from("https://adventofcode.com");
        let client = reqwest::Client::new();
        let session_token = std::env::var("SESSION_TOKEN").expect("");
        InputDataService {
            root_url,
            client,
            session_token,
        }
    }
    pub async fn get_input(&self, year: String, day: String) -> Result<String, String> {
        let h = calculate_hash(&self.session_token);
        let file_name = format!("inputs/{}_{}_{}.txt", h, year, day);
        let mut _input = String::new();
        let mut file = OpenOptions::new().read(true).open(&file_name);
        let file = file.as_mut();

        let input_status = match file {
            Ok(x) => {
                println!("reading from file");
                x.read_to_string(&mut _input)
            }
            Err(_) => {
                println!("reading from api");
                let url = format!("{}/{}/day/{}/input", &self.root_url, year, day);
                let session_cookie = format!("session={}", &self.session_token);
                let input_from_api = &self
                    .client
                    .get(url)
                    .header("cookie", session_cookie)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .to_string();
                create_dir("inputs");
                let mut file = File::create(file_name).unwrap();
                _input = input_from_api.clone();
                file.write(input_from_api.as_bytes())
            }
        };
        match input_status {
            Ok(_) => Ok(_input),
            Err(_) => Err(String::from("Error")),
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
