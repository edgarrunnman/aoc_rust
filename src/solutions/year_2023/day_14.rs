use colored::Colorize;

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
        let foo = swap_axis(foo);

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
        let mut foo = self
            .input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        let cyckles: u64 = 1000;

        let (list, platform) = (0..cyckles)
            .into_iter()
            .fold((vec![], foo), |acc, _| one_cykle(acc.0, acc.1));

        println!("{:?}", list);
        Some(list.last().unwrap().to_string())
    }
    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn one_cykle(mut scores: Vec<u32>, mut foo: Vec<String>) -> (Vec<u32>, Vec<String>) {
    let mut count = 0;

    while count < 4 {
        foo = swap_axis(foo.clone());
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
    for line in foo.clone() {
        for (i, char) in line.chars().into_iter().enumerate() {
            if char == 'O' {
                result = result + line.len() as u32 - i as u32;
            }
        }
    }
    scores.push(result);
    (scores, foo)
}
fn swap_axis(pattern: Vec<String>) -> Vec<String> {
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
