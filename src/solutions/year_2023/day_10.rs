use crate::Solution;
use colored::Colorize;
use tailcall::tailcall;

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

        // print_count(&map);
        let start_p = start_position(&map).unwrap();
        map = path_finder(start_p, start_p, map);
        print_map_2(&map);
        // print_count(&map);
        let result = max_count(&map);
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        None
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
fn path_finder(mut prev_p: Position, mut p: Position, mut map: Map) -> Map {
    let initial_p = prev_p;
    // println!("prev p: {:?}, next p {:?}", prev_p, p);
    let mut front = true;
    let mut back = true;
    let mut counter = 0;
    // while counter < 10 {
    //     counter = counter + 1;
    while back {
        // println!("prev {:?}", prev_p);
        // println!("next {:?}", p);
        let cur_count = map.get(p.x).and_then(|line| line.get(p.y)).unwrap().count;
        let up = map.get(p.x - 1).unwrap().get(p.y).unwrap();
        let down = map.get(p.x + 1).unwrap().get(p.y).unwrap();
        let left = map.get(p.x).unwrap().get(p.y - 1).unwrap();
        let right = map.get(p.x).unwrap().get(p.y + 1).unwrap();
        // println!("up {:?}", up);
        // println!("down {:?}", down);
        // println!("left {:?}", left);
        // println!("right {:?}", right);
        if (up.symb == 'F' || up.symb == '|' || up.symb == '7')
            && (cur_count <= up.count || up.count == 0)
            && prev_p.x != p.x - 1
        {
            // println!("");
            // println!("up");

            prev_p = p;
            p = Position { x: p.x - 1, y: p.y };
            let new_cell = Cell {
                symb: up.symb,
                count: cur_count + 1,
            };
            map[p.x][p.y] = new_cell;
            // print_count(&map);
        } else if (down.symb == 'L' || down.symb == '|' || down.symb == 'J')
            && (cur_count <= down.count || down.count == 0)
            && prev_p.x != p.x + 1
        {
            // println!("");
            // println!("down");
            prev_p = p;
            p = Position { x: p.x + 1, y: p.y };
            let new_cell = Cell {
                symb: down.symb,
                count: cur_count + 1,
            };
            map[p.x][p.y] = new_cell;
            // print_count(&map);
        } else if (left.symb == 'L' || left.symb == '-' || left.symb == 'F')
            && (cur_count <= left.count || left.count == 0)
            && prev_p.y != p.y - 1
        {
            // println!("");
            // println!("left");
            prev_p = p;
            p = Position { x: p.x, y: p.y - 1 };
            let new_cell = Cell {
                symb: left.symb,
                count: cur_count + 1,
            };
            map[p.x][p.y] = new_cell;
            // print_count(&map);
        } else if (right.symb == 'J' || right.symb == '-' || right.symb == '7')
            && (cur_count <= right.count || right.count == 0)
            && prev_p.y != p.y + 1
        {
            // println!("");
            // println!("right");
            // println!("prev {:?}", prev_p);
            // println!("next {:?}", p);
            prev_p = p;
            p = Position { x: p.x, y: p.y + 1 };
            let new_cell = Cell {
                symb: right.symb,
                count: cur_count + 1,
            };
            map[p.x][p.y] = new_cell;
            // print_count(&map);
        } else {
            back = false;
        }
    }

    p = initial_p;
    prev_p = initial_p;

    // while counter < 20 {
    //     counter = counter + 1;
    while front {
        let cur_count = map.get(p.x).and_then(|line| line.get(p.y)).unwrap().count;
        let start_cell = map.get(p.x).and_then(|line| line.get(p.y)).unwrap();
        let up = map.get(p.x - 1).unwrap().get(p.y).unwrap();
        let down = map.get(p.x + 1).unwrap().get(p.y).unwrap();
        let left = map.get(p.x).unwrap().get(p.y - 1).unwrap();
        let right = map.get(p.x).unwrap().get(p.y + 1).unwrap();
        // println!("up {:?}", up);
        // println!("down {:?}", down);
        // println!("left {:?}", left);
        // println!("right {:?}", right);
        if right_valid(start_cell, right) {
            // println!("");
            prev_p = p;
            p = Position { x: p.x, y: p.y + 1 };
            let new_right = Cell {
                symb: right.symb,
                count: cur_count + 1,
            };
            map[p.x][p.y] = new_right;

            // print_count(&map);
        } else if (left.symb == 'L' || left.symb == '-' || left.symb == 'F')
            && (cur_count <= left.count || left.count == 0)
            && prev_p.y != p.y - 1
        {
            prev_p = p;
            p = Position { x: p.x, y: p.y - 1 };
            let new_left = Cell {
                symb: left.symb,
                count: cur_count + 1,
            };
            map[p.x][p.y] = new_left;
            // print_count(&map);
        } else if (down.symb == 'L' || down.symb == '|' || down.symb == 'J')
            && (cur_count <= down.count || down.count == 0)
            && prev_p.x != p.x + 1
        {
            // println!("");
            prev_p = p;
            p = Position { x: p.x + 1, y: p.y };
            let new_down = Cell {
                symb: down.symb,
                count: cur_count + 1,
            };
            map[p.x][p.y] = new_down;
            // print_count(&map);
        } else if (up.symb == 'F' || up.symb == '|' || up.symb == '7')
            && (cur_count <= up.count || up.count == 0)
            && prev_p.x != p.x - 1
        {
            // println!("");

            prev_p = p;
            p = Position { x: p.x - 1, y: p.y };
            let new_up = Cell {
                symb: up.symb,
                count: cur_count + 1,
            };
            map[p.x][p.y] = new_up;
            // print_count(&map);
        } else {
            front = false;
        }
    }

    map
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    symb: char,
    count: u32,
}
#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}
impl Cell {
    fn new(symb: char) -> Cell {
        Cell { symb, count: 0 }
    }
}
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.symb == other.symb
    }
}

type Map = Vec<Vec<Cell>>;
fn print_map(map: &Map) {
    for line in map {
        for cell in line {
            print!(" {} ", cell.symb);
        }
        println!("");
    }
}
fn print_map_2(map: &Map) {
    for line in map {
        for cell in line {
            if cell.count == 0 {
                print!("{}", cell.symb);
            } else {
                print!("{}", cell.symb.to_string().blue());
            }
        }
        println!("");
    }
}
fn print_count(map: &Map) {
    for line in map {
        for cell in line {
            print!(" {} ", cell.count);
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
    for (x, line) in map.into_iter().enumerate() {
        for (y, cell) in line.into_iter().enumerate() {
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
            _ => false,
        }
    } else {
        false
    }
}
fn left_valid(source_c: Cell, target_c: Cell) -> bool {
    if source_c.count < target_c.count || target_c.count == 0 {
        match source_c.symb {
            '-' => ['-', 'L', 'F'].contains(&target_c.symb),
            '7' => ['-', 'L', 'F'].contains(&target_c.symb),
            'J' => ['-', 'L', 'F'].contains(&target_c.symb),
            _ => false,
        }
    } else {
        false
    }
}
fn up_valid(source_c: Cell, target_c: Cell) -> bool {
    if source_c.count < target_c.count || target_c.count == 0 {
        match source_c.symb {
            '|' => ['|', '7', 'F'].contains(&target_c.symb),
            'L' => ['-', '7', 'F'].contains(&target_c.symb),
            'J' => ['-', '7', 'F'].contains(&target_c.symb),
            _ => false,
        }
    } else {
        false
    }
}
fn down_valid(source_c: Cell, target_c: Cell) -> bool {
    if source_c.count < target_c.count || target_c.count == 0 {
        match source_c.symb {
            '|' => ['|', 'L', 'J'].contains(&target_c.symb),
            'F' => ['-', 'L', 'J'].contains(&target_c.symb),
            '7' => ['-', 'L', 'J'].contains(&target_c.symb),
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
fn test_second() {
    let test_input = "";
    let test_result = "";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
