#[allow(unused_imports)]
use colored::Colorize;

use crate::Solution;
pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let patterns = self
            .input
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .into_iter()
                    .map(|str| str.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        let foo = patterns.into_iter().map(get_symetry_points).sum::<usize>();
        Some(foo.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let patterns = self
            .input
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .into_iter()
                    .map(|str| str.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        let foo = patterns
            .into_iter()
            .map(get_symetry_points_smuge)
            .sum::<usize>();
        Some(foo.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
fn get_symetry_points_smuge(mut pattern: Vec<String>) -> usize {
    let mut biggest_index = 0;
    let mut vinner = 0;
    if let Some(index) = get_symetry_index_with_smuge(&pattern, false) {
        if index >= biggest_index {
            biggest_index = index;
            vinner = 1;
        }
    }
    pattern = swap_axis(pattern);
    if let Some(index) = get_symetry_index_with_smuge(&pattern, true) {
        if index > biggest_index || index == 0 {
            biggest_index = index;
            vinner = 2;
        }
    }
    if vinner == 1 {
        return (biggest_index + 1) * 100;
    }
    if vinner == 2 {
        return biggest_index + 1;
    }
    0
}
fn get_symetry_points(mut pattern: Vec<String>) -> usize {
    let mut biggest_index = 0;
    let mut vinner = 0;
    if let Some(index) = get_symetry_index(&pattern) {
        if index >= biggest_index {
            biggest_index = index;
            vinner = 1;
        }
    }
    pattern = swap_axis(pattern);
    if let Some(index) = get_symetry_index(&pattern) {
        if index > biggest_index || index == 0 {
            biggest_index = index;
            vinner = 2;
        }
    }
    if vinner == 1 {
        return (biggest_index + 1) * 100;
    }
    if vinner == 2 {
        return biggest_index + 1;
    }

    0
}
fn get_symetry_index(pattern: &Vec<String>) -> Option<usize> {
    let lenght = pattern.len();
    pattern
        .windows(2)
        .into_iter()
        .enumerate()
        .filter(|(_, lines)| lines.first().unwrap() == lines.last().unwrap())
        .map(|(index, _)| index)
        .filter(|index| symetric_index(*index, pattern))
        .reduce(|a, b| {
            let a_to_center = (lenght as i32 / 2 - a as i32).abs() as usize;
            let b_to_center = (lenght as i32 / 2 - b as i32).abs() as usize;
            if b_to_center < a_to_center {
                return b;
            } else {
                return a;
            }
        })
}

fn symetric_index(i: usize, pattern: &Vec<String>) -> bool {
    let lenght = pattern.len();
    if i >= lenght / 2 {
        for n in 0..(lenght - i - 2) {
            if pattern.get(i - n - 1).unwrap() != pattern.get(i + n + 2).unwrap() {
                return false;
            }
        }
    } else {
        for n in 0..i {
            if pattern.get(i - n - 1).unwrap() != pattern.get(i + n + 2).unwrap() {
                return false;
            }
        }
    }
    true
}
#[allow(unused_variables)]
fn get_symetry_index_with_smuge(pattern: &Vec<String>, rot: bool) -> Option<usize> {
    let lenght = pattern.len();
    let anti_indexes = pattern
        .windows(2)
        .into_iter()
        .enumerate()
        .filter(|(_, lines)| lines.first().unwrap() == lines.last().unwrap())
        .map(|(index, _)| index)
        .filter(|index| symetric_index(*index, &pattern))
        .collect::<Vec<usize>>();

    let mut indexes = vec![];
    for (line_i, line) in pattern.into_iter().enumerate() {
        for (char_i, char) in line.chars().into_iter().enumerate() {
            let mut test_pattern = pattern
                .clone()
                .into_iter()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            if char == '.' {
                test_pattern[line_i][char_i] = '#'
            } else {
                test_pattern[line_i][char_i] = '.'
            }

            let test_pattern = test_pattern
                .into_iter()
                .map(|line| line.into_iter().collect::<String>())
                .collect::<Vec<String>>();

            test_pattern
                .windows(2)
                .into_iter()
                .enumerate()
                .filter(|(_, lines)| lines.first().unwrap() == lines.last().unwrap())
                .map(|(index, _)| index)
                .filter(|index| !anti_indexes.contains(index))
                .filter(|index| symetric_index(*index, &test_pattern))
                .for_each(|index| {
                    indexes.push(index);
                })
        }
    }
    let index = indexes.into_iter().reduce(|a, b| {
        let a_to_center = (lenght as i32 / 2 - a as i32).abs() as usize;
        let b_to_center = (lenght as i32 / 2 - b as i32).abs() as usize;
        if b_to_center < a_to_center {
            return b;
        } else {
            return a;
        }
    });
    index
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
    let test_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let test_result = "405";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_first_2() {
    let test_input = "....#.#..#
.###.#####
..#.#.....
.###.##..#
..##.#.##.
..#.#.#..#
..#...#..#
.###..####
.###.#....
.###.#....
.###..####
..#...#..#
..#.#.#..#
..##.#.##.
.###.##..#";
    let test_result = "8";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_first_3() {
    let test_input = ".#.#.#..##.##
.#.####.....#
..##.#....#..
###.#..##..##
###.#..##..##
..##.#....#..
.#.####.....#
.#.#.#..##.##
.#.#.#..##.##";
    let test_result = "400";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_first_4() {
    let test_input = ".#.#.#..##.##
.#.####.....#
..##......#..
###.#..##..##
###.#..##..##
..##.#....#..
.#.####.....#
.#.#.#..##.##
.#.#.#..##.##";
    let test_result = "800";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let test_result = "400";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
