use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let mut part_numbers: Vec<PartNumber> = vec![];
        let mut symbol_points: Vec<SymbolPoint> = vec![];
        for (line_index, line) in self.input.lines().enumerate() {
            let chars = line.chars().collect::<Vec<char>>();
            let mut number: String = String::new();
            let mut x_min: usize = 0;
            let mut x_max: usize;
            for (x, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    if number.is_empty() {
                        x_min = x;
                    }
                    number.push(char)
                } else if char != '.' {
                    symbol_points.push(SymbolPoint { x, line_index });
                }
                if !chars.get(x + 1).unwrap_or(&'.').is_numeric() && !number.is_empty() {
                    x_max = x;
                    part_numbers.push(PartNumber {
                        number: number.parse::<u32>().unwrap(),
                        buffer: Buffer {
                            x_min,
                            x_max,
                            line_index,
                        },
                    });
                    number.clear();
                }
            }
        }
        let result = part_numbers
            .iter()
            .filter(|part_number| {
                symbol_points
                    .clone()
                    .into_iter()
                    .any(|symbol_point| symbol_touching_number(&symbol_point, &part_number.buffer))
            })
            .map(|part_number| part_number.number)
            .sum::<u32>();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let mut part_numbers: Vec<PartNumber> = vec![];
        let mut symbol_points: Vec<SymbolPoint> = vec![];
        for (line_index, line) in self.input.lines().enumerate() {
            let chars = line.chars().collect::<Vec<char>>();
            let mut number: String = String::new();
            let mut x_min: usize = 0;
            let mut x_max: usize;

            for (x, char) in line.chars().enumerate() {
                if char == '*' {
                    symbol_points.push(SymbolPoint { x, line_index });
                } else if char.is_numeric() {
                    if number.is_empty() {
                        x_min = x;
                    }
                    number.push(char)
                }
                if !chars.get(x + 1).unwrap_or(&'.').is_numeric() && !number.is_empty() {
                    x_max = x;
                    part_numbers.push(PartNumber {
                        number: number.parse::<u32>().unwrap(),
                        buffer: Buffer {
                            x_min,
                            x_max,
                            line_index,
                        },
                    });
                    number.clear();
                }
            }
        }
        let result = symbol_points
            .into_iter()
            .map(|gear_point| {
                let gear_numbers = part_numbers
                    .clone()
                    .into_iter()
                    .filter(|part_number| symbol_touching_number(&gear_point, &part_number.buffer))
                    .map(|part_number| part_number.number)
                    .collect::<Vec<u32>>();
                match gear_numbers.len() {
                    2 => gear_numbers[0] * gear_numbers[1],
                    _ => 0,
                }
            })
            .sum::<u32>();
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
fn symbol_touching_number(p: &SymbolPoint, b: &Buffer) -> bool {
    p.x + 1 >= b.x_min
        && p.x - 1 <= b.x_max
        && p.line_index + 1 >= b.line_index
        && p.line_index - 1 <= b.line_index
}

#[derive(Debug)]
struct PartNumber {
    number: u32,
    buffer: Buffer,
}

impl Clone for PartNumber {
    fn clone(&self) -> Self {
        Self {
            number: self.number.clone(),
            buffer: self.buffer.clone(),
        }
    }
}
#[derive(Debug)]
struct Buffer {
    x_min: usize,
    x_max: usize,
    line_index: usize,
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self {
            x_min: self.x_min.clone(),
            x_max: self.x_max.clone(),
            line_index: self.line_index.clone(),
        }
    }
}
#[derive(Debug)]
struct SymbolPoint {
    x: usize,
    line_index: usize,
}

impl Clone for SymbolPoint {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            line_index: self.line_index.clone(),
        }
    }
}

#[test]
fn test_first() {
    let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let test_result = "4361";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}

#[test]
fn test_second() {
    let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let test_result = "467835";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
