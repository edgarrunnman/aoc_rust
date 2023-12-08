use crate::Solution;
use std::collections::HashMap;
pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let instructions = self.input.lines().next().unwrap();
        let nods = nodes(&self.input);

        let mut counter = 0;
        let mut not_last = true;
        let mut next_node = nods.get("AAA").unwrap();
        while not_last {
            for instruction in instructions.chars() {
                counter += 1;
                if instruction == 'L' {
                    let left = &next_node.0;
                    if is_last(left) {
                        not_last = false;
                        break;
                    }
                    next_node = nods.get(left).unwrap();
                } else {
                    let right = &next_node.1;
                    if is_last(right) {
                        not_last = false;
                        break;
                    }
                    next_node = nods.get(right).unwrap();
                }
            }
        }
        Some(counter.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let instructions = self.input.lines().next().unwrap();
        let nods = nodes(&self.input);

        let mut pathes = nods
            .clone()
            .into_iter()
            .filter(|(k, _)| k.ends_with("A"))
            .map(|(k, _)| k)
            .collect::<Vec<String>>();

        let mut steps: Vec<u32> = vec![];
        let mut counter = 0;
        while pathes.len() > 0 {
            for instruction in instructions.chars() {
                counter += 1;
                let mut new_pathes: Vec<String> = vec![];
                for index in 0..pathes.len() {
                    let path = pathes.get(index).unwrap();
                    let node = nods.get(path).unwrap();
                    if instruction == 'L' {
                        let left = &node.0;
                        if left.ends_with(&"z".to_string()) {
                            steps.push(counter);
                        } else {
                            new_pathes.push(left.to_string());
                        }
                    } else {
                        let right = &node.1;
                        if right.ends_with(&"Z".to_string()) {
                            steps.push(counter);
                        } else {
                            new_pathes.push(right.to_string());
                        }
                    }
                }
                pathes = new_pathes;
            }
        }
        let result = steps
            .iter()
            .map(|step| step.to_string().parse::<i128>().unwrap())
            .reduce(|a, b| num_integer::lcm(a, b))
            .unwrap();

        Some(result.to_string())
    }
    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn directions<'a>(input: &str) -> (String, String) {
    let directions = input.split(", ").collect::<Vec<&str>>();
    let left = directions.first().unwrap().replace("(", "");
    let right = directions.last().unwrap().replace(")", "");
    (left, right)
}

fn nodes(input: &str) -> HashMap<String, (String, String)> {
    input
        .lines()
        .into_iter()
        .filter(|line| line.contains("="))
        .map(|line| {
            let split = line.split(" = ").collect::<Vec<&str>>();
            let node = split.first().unwrap().to_string();
            let directions = directions(split.last().unwrap());
            (node, directions)
        })
        .collect()
}

fn is_last(input: &String) -> bool {
    input == &"ZZZ".to_string()
}

#[test]
fn test_first_1() {
    let test_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let test_result = "2";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}

#[test]
fn test_first_2() {
    let test_input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let test_result = "6";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let test_result = "6";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
