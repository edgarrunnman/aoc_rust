use std::collections::hash_map::DefaultHasher;
use std::fs::create_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::solutions::SolutionIdentity;

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
    pub async fn get_input(&self, id: &SolutionIdentity) -> Result<String, String> {
        create_input_folder_if_missing();
        let h = calculate_hash(&self.session_token);
        let file_name = format!("inputs/{}_{}_{}.txt", h, id.year, id.day);
        let mut _input = String::new();
        let mut file = OpenOptions::new().read(true).open(&file_name);

        let input_status = match file.as_mut() {
            Ok(f) => {
                println!("reading from file");
                f.read_to_string(&mut _input)
            }
            Err(_) => {
                println!("reading from api");
                let url = format!("{}/{}/day/{}/input", &self.root_url, id.year, id.day);
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
                    .unwrap();

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

fn create_input_folder_if_missing() {
    let root_path = project_root::get_project_root().unwrap_or_default();
    let path_string = format!("{}/inputs", root_path.to_str().unwrap());
    if !Path::new(&path_string).exists() {
        create_dir(path_string).unwrap();
    }
}
