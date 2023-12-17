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
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .split(',')
            .into_iter()
            .map(hasher)
            .map(|n| n as u32)
            .sum::<u32>();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let hash_map = self
            .input
            .lines()
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .split(',')
            .into_iter()
            .fold(HashMap::new(), |hash_map, step| {
                uppdate_hash_map(hash_map, step)
            });
        let result = hash_map
            .into_iter()
            .map(|(key, lenses)| {
                lenses
                    .into_iter()
                    .enumerate()
                    .map(|(index, lens)| (key as u32 + 1) * (index as u32 + 1) * lens.1 as u32)
                    .sum::<u32>()
            })
            .sum::<u32>();

        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
fn uppdate_hash_map<'a>(
    mut hash_map: HashMap<u8, Vec<(&'a str, u8)>>,
    step: &'a str,
) -> HashMap<u8, Vec<(&'a str, u8)>> {
    if step.contains('=') {
        let split = step.split('=').collect::<Vec<&str>>();
        let lens_label = split.first().unwrap();
        let label_hash = hasher(lens_label);
        let lens_pwr = split.last().unwrap().parse::<u8>().unwrap();
        if let Some(b) = hash_map.get_mut(&label_hash) {
            if let Some(index) = &b.into_iter().position(|lens| &lens.0 == lens_label) {
                let mut new_b: Vec<(&str, u8)> = b.clone();
                new_b[*index] = (lens_label, lens_pwr);
                *b = new_b;
            } else {
                let mut new_b: Vec<(&str, u8)> = b.clone();
                new_b.push((lens_label, lens_pwr));
                *b = new_b;
            }
        } else {
            hash_map.insert(label_hash, vec![(lens_label, lens_pwr)]);
        }
    }
    if step.contains('-') {
        let label_string = step.replace("-", "");
        let label = label_string.as_str();
        let label_hash = hasher(label);
        if let Some(b) = hash_map.get_mut(&label_hash) {
            let new_b = b
                .clone()
                .into_iter()
                .filter(|lens| lens.0 != label)
                .collect();
            *b = new_b;
        }
    }

    hash_map
}
fn hasher(string: &str) -> u8 {
    string
        .chars()
        .into_iter()
        .fold(0, |acc, char| ((acc as u32 + char as u32) * 17) % 256) as u8
}
#[test]
fn test_first() {
    let test_input = "HASH";
    let test_result = "52";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}

#[test]
fn test_first_2() {
    let test_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let test_result = "1320";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}

#[test]
fn test_second() {
    let test_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let test_result = "145";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
