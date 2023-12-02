use std::collections::HashMap;

use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let result = self
            .input
            .lines()
            .into_iter()
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| {
                let line = line.split(':').collect::<Vec<&str>>().clone().to_owned();
                let game_and_sets = (line[0], line[1]);
                let game_id = String::from(game_and_sets.0.split(' ').collect::<Vec<&str>>()[1])
                    .parse::<u16>();
                let sets_hash = normalize_sets(game_and_sets.1);
                let valid_sets_hash = valid_sets(sets_hash);
                (game_id.unwrap(), valid_sets_hash)
            })
            .filter(|game| game.1)
            .map(|game| game.0)
            .sum::<u16>();

        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let result = self
            .input
            .lines()
            .into_iter()
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| {
                let line = line.split(':').collect::<Vec<&str>>().clone().to_owned();
                let game_and_sets = (line[0], line[1]);
                let sets_hash = normalize_sets(game_and_sets.1);
                set_power(sets_hash)
            })
            .sum::<u64>();

        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn normalize_sets(sets: &str) -> Vec<HashMap<String, u16>> {
    sets.split(';')
        .map(|it| String::from(it.trim()))
        .map(|set| {
            let set = set
                .split(',')
                .map(|it| String::from(it.trim()))
                .collect::<Vec<String>>();
            normalize_set(&set)
        })
        .collect()
}
fn normalize_set(gems: &Vec<String>) -> HashMap<String, u16> {
    gems.iter()
        .map(|gem| {
            let gems_amount = gem
                .split(' ')
                .map(|it| String::from(it.trim()))
                .collect::<Vec<String>>();
            let n = gems_amount[0].parse::<u16>().unwrap();
            let color = gems_amount[1].clone();
            (color, n)
        })
        .collect()
}

fn set_power(sets: Vec<HashMap<String, u16>>) -> u64 {
    let mut red: u16 = 0;
    let mut green: u16 = 0;
    let mut blue: u16 = 0;
    let zero: u16 = 0;
    for set in &sets {
        let set_red = set.get("red").unwrap_or(&zero).clone();
        let set_green = set.get("green").unwrap_or(&zero).clone();
        let set_blue = set.get("blue").unwrap_or(&zero).clone();
        if set_red > red {
            red = set_red
        }
        if set_green > green {
            green = set_green
        }
        if set_blue > blue {
            blue = set_blue
        }
    }
    red as u64 * green as u64 * blue as u64
}

fn valid_sets(sets: Vec<HashMap<String, u16>>) -> bool {
    sets.iter().all(|set| valid_set(set))
}

fn valid_set(set: &HashMap<String, u16>) -> bool {
    let max_blue: u16 = 14;
    let max_red: u16 = 12;
    let max_green: u16 = 13;
    let zero: u16 = 0;
    if set.get("blue").unwrap_or(&zero) > &max_blue {
        return false;
    }
    if set.get("red").unwrap_or(&zero) > &max_red {
        return false;
    }
    if set.get("green").unwrap_or(&zero) > &max_green {
        return false;
    }
    return true;
}

pub(crate) fn test_first() {
    let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let test_result = "8";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
pub(crate) fn test_second() {
    let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let test_result = "2286";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
