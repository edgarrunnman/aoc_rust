use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let foo = self
            .input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        let foo = swap_axis(&foo);

        let mut result = 0;
        let mut new_foo = vec![];
        for line in foo {
            // println!("{}", line.blue());
            let mut line = line.chars().rev().collect::<Vec<_>>();
            for i in 1..line.len() {
                if line[i] == 'O' {
                    let mut n = 0;
                    while line[i - 1 - n as usize] == '.' {
                        line[i - 1 - n] = 'O';
                        line[i - n] = '.';
                        n = n + 1;
                        if (i as i16 - 1 - n as i16) < 0 {
                            break;
                        }
                    }
                }
            }
            new_foo.push(line.clone());

            // println!("{:?}", line);
            let mut count = 0;
            for (i, char) in line.clone().into_iter().enumerate() {
                if char == 'O' {
                    count = count + line.len() - i;
                }
            }
            // println!("{}", count);
            result = result + count;
        }
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let foo = self
            .input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        let cyckles: u64 = 1000000000;

        let (_, platform) = (0..1000)
            .into_iter()
            .fold((vec![], foo), |acc, _| one_cykle(acc.0, acc.1));

        let (mut pattern, _) = (0..100)
            .into_iter()
            .fold((vec![], platform), |acc, _| one_cykle(acc.0, acc.1));

        let mut step_pattern: Vec<(u32, String)> = vec![];
        let mut step_size: usize = 0;
        while step_size == 0 {
            let current_value = pattern.remove(0);
            step_pattern.push(current_value);
            if step_pattern[0].1 == pattern[0].1 {
                step_size = step_pattern.len();
            }
        }

        // println!("{:?}", pattern);
        println!("{:?}", step_pattern);
        let step_pattern = step_pattern
            .into_iter()
            .map(|it| it.0)
            .collect::<Vec<u32>>();

        println!("{:?}", step_pattern);
        let foo = (cyckles - 1001) % step_size as u64;

        println!("mod {:?}", foo);
        let result = step_pattern[foo as usize];
        println!("resutl {:?}", result);
        Some(result.to_string())
    }
    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn one_cykle(
    mut scores: Vec<(u32, String)>,
    mut foo: Vec<String>,
) -> (Vec<(u32, String)>, Vec<String>) {
    let mut count = 0;

    while count < 4 {
        foo = swap_axis(&foo);
        let mut new_foo: Vec<String> = vec![];
        for line in foo {
            let mut line = line.chars().rev().collect::<Vec<_>>();
            for i in 1..line.len() {
                if line[i] == 'O' {
                    let mut n = 0;
                    while line[i - 1 - n as usize] == '.' {
                        line[i - 1 - n] = 'O';
                        line[i - n] = '.';
                        n = n + 1;
                        if (i as i16 - 1 - n as i16) < 0 {
                            break;
                        }
                    }
                }
            }
            line.reverse();
            let line = line.clone().into_iter().collect::<String>();
            // println!("{}", line.blue());
            new_foo.push(line);
        }
        foo = new_foo;
        count += 1;
    }

    let mut result: u32 = 0;

    for line in swap_axis(&foo) {
        // println!("{}", line.blue());
        for (i, char) in line.chars().into_iter().enumerate() {
            if char == 'O' {
                result = result + i as u32 + 1;
            }
        }
    }

    // println!("");
    // println!("load: {}", result);
    // println!("");
    // println!("");
    let mut hasher = DefaultHasher::new();
    foo.hash(&mut hasher);
    scores.push((result, hasher.finish().to_string()));
    (scores, foo)
}
fn swap_axis(pattern: &Vec<String>) -> Vec<String> {
    let width = pattern.len();
    let height = pattern.first().unwrap().len();

    let mut swap_lines: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    for (x, line) in pattern.clone().into_iter().enumerate() {
        for (y, char) in line.chars().into_iter().enumerate() {
            swap_lines[y][width - x - 1] = char;
        }
    }
    let new_pattern = swap_lines
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .collect::<Vec<String>>();
    new_pattern
}

#[test]
fn test_first() {
    let test_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let test_result = "136";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let test_result = "64";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
