use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let lines = self.input.lines();
        let lines = lines
            .map(|line| {
                line.split(":")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .split(' ')
                    .filter(|str| !str.is_empty())
                    .map(|number_str| number_str.trim().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        let times = lines.get(0).unwrap().clone();
        let distances = lines.get(1).unwrap().clone();
        let races = times
            .into_iter()
            .zip(distances.into_iter())
            .map(|pair| Race {
                time: pair.0,
                distance: pair.1,
            })
            .collect::<Vec<Race>>();
        let result = races
            .into_iter()
            .map(|race| get_winning_count(race))
            .fold(1, |acc, count| acc * count);
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let lines = self.input.lines();
        let lines = lines
            .map(|line| {
                line.split(":")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .replace(" ", "")
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<u64>>();
        let time = lines.get(0).unwrap().clone();
        let distance = lines.get(1).unwrap().clone();
        let race = Race { time, distance };
        let result = get_winning_count(race);
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn get_winning_count(race: Race) -> u64 {
    let mut min_time: u64 = 0;
    let mut max_time: u64 = 0;
    for ms in 1..race.time {
        if ms * (race.time - ms) > race.distance {
            min_time = ms;
            break;
        }
    }
    for ms in (1..race.time).rev() {
        if ms * (race.time - ms) > race.distance {
            max_time = ms;
            break;
        }
    }
    max_time - min_time + 1
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

#[test]
fn test_first() {
    let test_input = "Time:      7  15   30
Distance:  9  40  200";
    let test_result = "288";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "Time:      7  15   30
Distance:  9  40  200";
    let test_result = "71503";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
