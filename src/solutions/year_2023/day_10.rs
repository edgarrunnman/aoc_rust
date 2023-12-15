use crate::Solution;
use colored::Colorize;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let mut map = self
            .input
            .lines()
            .map(|line| {
                let mut lines = line.to_string();
                lines.insert(0, '.');
                lines.push('.');
                let lines = lines.chars().map(Cell::new).collect::<Vec<Cell>>();
                lines
            })
            .collect::<Map>();
        let extra_line = (0..map.first().unwrap().len())
            .map(|_| Cell::new('.'))
            .collect::<Vec<Cell>>();
        map.insert(0, extra_line.clone());
        map.push(extra_line);

        let start_p = start_position(&map).unwrap();
        let (map, _) = path_finder(start_p, start_p, map);
        let result = max_count(&map).div_ceil(2);
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let mut map = self
            .input
            .lines()
            .map(|line| {
                let mut lines = line.to_string();
                lines.insert(0, '.');
                lines.push('.');
                let lines = lines.chars().map(Cell::new).collect::<Vec<Cell>>();
                lines
            })
            .collect::<Map>();
        let extra_line = (0..map.first().unwrap().len())
            .map(|_| Cell::new('.'))
            .collect::<Vec<Cell>>();
        map.insert(0, extra_line.clone());
        map.push(extra_line);

        let start_p = start_position(&map).unwrap();
        let (mut map, mut path) = path_finder(start_p, start_p, map);
        // print_map(&map, start_p);
        path.push(start_p);
        path.reverse();
        map = mark_inner_loop(path, map);
        // print_map(&map, start_p);
        let map = fill_inner(map);
        // print_map(&map, start_p);
        let count_inner = map
            .into_iter()
            .map(|line| line.iter().filter(|cell| cell.symb == '*').count())
            .sum::<usize>();
        Some(count_inner.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
fn fill_inner(mut map: Map) -> Map {
    let x_max = map.len() - 1;
    let y_max = map.first().unwrap().len() - 1;
    for x in 1..x_max {
        for y in 1..y_max {
            let cell = map.get(x).unwrap().get(y).unwrap();
            if cell.symb == '*' {
                if map.get(x - 1).unwrap().get(y - 1).unwrap().count == 0 {
                    map[x - 1][y - 1].symb = '*';
                }
                if map.get(x - 1).unwrap().get(y).unwrap().count == 0 {
                    map[x - 1][y].symb = '*';
                }
                if map.get(x - 1).unwrap().get(y + 1).unwrap().count == 0 {
                    map[x - 1][y + 1].symb = '*';
                }
                if map.get(x + 1).unwrap().get(y - 1).unwrap().count == 0 {
                    map[x + 1][y - 1].symb = '*';
                }
                if map.get(x + 1).unwrap().get(y).unwrap().count == 0 {
                    map[x + 1][y].symb = '*';
                }
                if map.get(x + 1).unwrap().get(y + 1).unwrap().count == 0 {
                    map[x + 1][y + 1].symb = '*';
                }
                if map.get(x).unwrap().get(y - 1).unwrap().count == 0 {
                    map[x][y - 1].symb = '*';
                }
                if map.get(x).unwrap().get(y + 1).unwrap().count == 0 {
                    map[x][y + 1].symb = '*';
                }
            }
        }
    }
    map
}
fn mark_inner_loop(path: Vec<Position>, mut map: Map) -> Map {
    for (prev_p, p) in path.windows(2).map(|p| (p[0], p[1])) {
        let nav_up = prev_p.x == p.x + 1;
        let nav_down = prev_p.x == p.x - 1;
        let nav_right = prev_p.y == p.y - 1;
        let nav_left = prev_p.y == p.y + 1;
        let cell = map.get(p.x).unwrap().get(p.y).unwrap().clone();

        if nav_up {
            let inner_c = map.get(p.x).unwrap().get(p.y + 1).unwrap();
            if !inner_c.inner && inner_c.count == 0 {
                let new_cell = inner_c.clone_inner();
                map[p.x][p.y + 1] = new_cell;
            }
            if cell.symb == '7' || cell.symb == 'S' {
                let inner_c = map.get(p.x - 1).unwrap().get(p.y + 1).unwrap();
                if !inner_c.inner && inner_c.count == 0 {
                    let new_cell = inner_c.clone_inner();
                    map[p.x - 1][p.y + 1] = new_cell;
                }
                let inner_c = map.get(p.x - 1).unwrap().get(p.y).unwrap();
                if !inner_c.inner && inner_c.count == 0 {
                    let new_cell = inner_c.clone_inner();
                    map[p.x - 1][p.y] = new_cell;
                }
            }
        } else if nav_down {
            let inner_c = map.get(p.x).unwrap().get(p.y - 1).unwrap();
            if !inner_c.inner && inner_c.count == 0 {
                let new_cell = inner_c.clone_inner();
                map[p.x][p.y - 1] = new_cell;
            }
            if cell.symb == 'L' || cell.symb == 'S' {
                let inner_c = map.get(p.x + 1).unwrap().get(p.y - 1).unwrap();
                if !inner_c.inner && inner_c.count == 0 {
                    let new_cell = inner_c.clone_inner();
                    map[p.x + 1][p.y - 1] = new_cell;
                }
                let inner_c = map.get(p.x + 1).unwrap().get(p.y).unwrap();
                if !inner_c.inner && inner_c.count == 0 {
                    let new_cell = inner_c.clone_inner();
                    map[p.x + 1][p.y] = new_cell;
                }
            }
        } else if nav_right {
            let inner_c = map.get(p.x + 1).unwrap().get(p.y).unwrap();
            if !inner_c.inner && inner_c.count == 0 {
                let new_cell = inner_c.clone_inner();
                map[p.x + 1][p.y] = new_cell;

                if cell.symb == 'J' || cell.symb == 'S' {
                    let inner_c = map.get(p.x + 1).unwrap().get(p.y + 1).unwrap();
                    if !inner_c.inner && inner_c.count == 0 {
                        let new_cell = inner_c.clone_inner();
                        map[p.x + 1][p.y + 1] = new_cell;
                    }
                    let inner_c = map.get(p.x).unwrap().get(p.y + 1).unwrap();
                    if !inner_c.inner && inner_c.count == 0 {
                        let new_cell = inner_c.clone_inner();
                        map[p.x][p.y + 1] = new_cell;
                    }
                }
            }
        } else if nav_left {
            let inner_c = map.get(p.x - 1).unwrap().get(p.y).unwrap();
            if !inner_c.inner && inner_c.count == 0 {
                let new_cell = inner_c.clone_inner();
                map[p.x - 1][p.y] = new_cell;
                if cell.symb == 'F' || cell.symb == 'S' {
                    let inner_c = map.get(p.x - 1).unwrap().get(p.y - 1).unwrap();
                    if !inner_c.inner && inner_c.count == 0 {
                        let new_cell = inner_c.clone_inner();
                        map[p.x - 1][p.y - 1] = new_cell;
                    }
                    let inner_c = map.get(p.x).unwrap().get(p.y - 1).unwrap();
                    if !inner_c.inner && inner_c.count == 0 {
                        let new_cell = inner_c.clone_inner();
                        map[p.x][p.y - 1] = new_cell;
                    }
                }
            }
        }
    }
    map
}

fn path_finder(mut prev_p: Position, mut p: Position, mut map: Map) -> (Map, Vec<Position>) {
    let mut path: Vec<Position> = vec![];
    let mut tracking = true;
    while tracking {
        let cur_count = map.get(p.x).and_then(|line| line.get(p.y)).unwrap().count;
        let start_cell = map.get(p.x).and_then(|line| line.get(p.y)).unwrap();
        let up = map.get(p.x - 1).unwrap().get(p.y).unwrap();
        let down = map.get(p.x + 1).unwrap().get(p.y).unwrap();
        let left = map.get(p.x).unwrap().get(p.y - 1).unwrap();
        let right = map.get(p.x).unwrap().get(p.y + 1).unwrap();
        if right_valid(start_cell, right) && prev_p.y != p.y + 1 {
            prev_p = p;
            p = Position { x: p.x, y: p.y + 1 };
            path.push(p);
            let new_right = right.clone_with_count(cur_count + 1);
            map[p.x][p.y] = new_right;
        } else if left_valid(start_cell, left) && prev_p.y != p.y - 1 {
            prev_p = p;
            p = Position { x: p.x, y: p.y - 1 };
            path.push(p);
            let new_left = left.clone_with_count(cur_count + 1);
            map[p.x][p.y] = new_left;
        } else if down_valid(start_cell, down) && prev_p.x != p.x + 1 {
            prev_p = p;
            p = Position { x: p.x + 1, y: p.y };
            path.push(p);
            let new_down = down.clone_with_count(cur_count + 1);
            map[p.x][p.y] = new_down;
        } else if up_valid(start_cell, up) && prev_p.x != p.x - 1 {
            prev_p = p;
            p = Position { x: p.x - 1, y: p.y };
            path.push(p);
            let new_up = up.clone_with_count(cur_count + 1);
            map[p.x][p.y] = new_up;
        } else {
            tracking = false;
        }
    }
    (map, path)
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    symb: char,
    count: u32,
    inner: bool,
}
#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}
impl Cell {
    fn new(symb: char) -> Cell {
        Cell {
            symb,
            count: 0,
            inner: false,
        }
    }
    fn clone_with_count(&self, count: u32) -> Cell {
        Cell {
            symb: self.symb.clone(),
            count,
            inner: false,
        }
    }
    fn clone_inner(&self) -> Cell {
        Cell {
            symb: '*',
            count: self.count.clone(),
            inner: true,
        }
    }
}
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.symb == other.symb
    }
}

type Map = Vec<Vec<Cell>>;
#[allow(dead_code)]
fn print_map(map: &Map, p: Position) {
    for (x, line) in map.into_iter().enumerate() {
        for (y, cell) in line.into_iter().enumerate() {
            if p.x == x && p.y == y {
                print!("{}", cell.symb.to_string().purple());
            } else if cell.count != 0 {
                print!("{}", cell.symb.to_string().blue());
            } else if cell.symb == '*' {
                print!("{}", cell.symb.to_string().red());
            } else {
                print!("{}", cell.symb);
            }
        }
        println!("");
    }
}

fn start_position(map: &Map) -> Option<Position> {
    for (x, line) in map.into_iter().enumerate() {
        for (y, cell) in line.into_iter().enumerate() {
            if cell == &Cell::new('S') {
                return Some(Position { x, y });
            }
        }
    }
    None
}
fn max_count(map: &Map) -> u32 {
    let mut max_count: u32 = 0;
    for line in map {
        for cell in line {
            if cell.count > max_count {
                max_count = cell.count;
            }
        }
    }
    max_count
}

fn right_valid(source_c: &Cell, target_c: &Cell) -> bool {
    if source_c.count < target_c.count || target_c.count == 0 {
        match source_c.symb {
            '-' => ['-', 'J', '7'].contains(&target_c.symb),
            'F' => ['-', 'J', '7'].contains(&target_c.symb),
            'L' => ['-', 'J', '7'].contains(&target_c.symb),
            'S' => true,
            _ => false,
        }
    } else {
        false
    }
}
fn left_valid(source_c: &Cell, target_c: &Cell) -> bool {
    if source_c.count < target_c.count || target_c.count == 0 {
        match source_c.symb {
            '-' => ['-', 'L', 'F'].contains(&target_c.symb),
            '7' => ['-', 'L', 'F'].contains(&target_c.symb),
            'J' => ['-', 'L', 'F'].contains(&target_c.symb),
            'S' => true,
            _ => false,
        }
    } else {
        false
    }
}
fn up_valid(source_c: &Cell, target_c: &Cell) -> bool {
    if source_c.count < target_c.count || target_c.count == 0 {
        match source_c.symb {
            '|' => ['|', '7', 'F'].contains(&target_c.symb),
            'L' => ['|', '7', 'F'].contains(&target_c.symb),
            'J' => ['|', '7', 'F'].contains(&target_c.symb),
            'S' => true,
            _ => false,
        }
    } else {
        false
    }
}
fn down_valid(source_c: &Cell, target_c: &Cell) -> bool {
    if source_c.count < target_c.count || target_c.count == 0 {
        match source_c.symb {
            '|' => ['|', 'L', 'J'].contains(&target_c.symb),
            'F' => ['|', 'L', 'J'].contains(&target_c.symb),
            '7' => ['|', 'L', 'J'].contains(&target_c.symb),
            'S' => true,
            _ => false,
        }
    } else {
        false
    }
}

#[test]
fn test_first_1() {
    let test_input = ".....
.S-7.
.|.|.
.L-J.
.....";
    let test_result = "4";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_first_2() {
    let test_input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let test_result = "8";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second_1() {
    let test_input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    let test_result = "4";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}

#[test]
fn test_second_2() {
    let test_input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    let test_result = "8";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}

#[test]
fn test_second_3() {
    let test_input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    let test_result = "10";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}

#[test]
fn test_second_4() {
    let test_input = "F-----7
|.....|
|.....|
|.....|
|.....|
|.....|
|.....|
S-----J";
    let test_result = "10";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
