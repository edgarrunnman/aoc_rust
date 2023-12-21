use colored::Colorize;

use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let mut map = self
            .input
            .lines()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|char| (char, None))
                    .collect::<Vec<(char, Option<Direction>)>>()
            })
            .collect::<Map>();

        map = mark_map(map, &Point { x: 0, y: 0 }, Direction::E);
        let result = map
            .into_iter()
            .map(|line| line.into_iter().filter(|cell| cell.1 != None).count())
            .sum::<usize>();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let map = self
            .input
            .lines()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|char| (char, None))
                    .collect::<Vec<(char, Option<Direction>)>>()
            })
            .collect::<Map>();

        print_map(&map);

        let mut to_w_points = map
            .clone()
            .into_iter()
            .enumerate()
            .map(|(n, _)| {
                (
                    Point {
                        x: n as isize,
                        y: map.len() as isize,
                    },
                    Direction::W,
                )
            })
            .collect::<Vec<(Point, Direction)>>();

        let mut to_e_points = to_w_points
            .clone()
            .into_iter()
            .map(|p| {
                (
                    Point {
                        x: p.0.x,
                        y: 0 as isize,
                    },
                    Direction::E,
                )
            })
            .collect::<Vec<(Point, Direction)>>();

        let mut to_n_points = map
            .clone()
            .first()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(n, _)| {
                (
                    Point {
                        x: map.first().unwrap().len() as isize,
                        y: n as isize,
                    },
                    Direction::N,
                )
            })
            .collect::<Vec<(Point, Direction)>>();

        let mut to_s_points = to_n_points
            .clone()
            .into_iter()
            .map(|p| {
                (
                    Point {
                        x: 0 as isize,
                        y: p.0.y,
                    },
                    Direction::S,
                )
            })
            .collect::<Vec<(Point, Direction)>>();

        to_w_points.append(to_e_points.as_mut());
        to_w_points.append(to_n_points.as_mut());
        to_w_points.append(to_s_points.as_mut());
        let all_points = to_w_points;

        let result = all_points
            .into_iter()
            .map(|p| {
                let mut map = map.clone();
                map = mark_map(map, &p.0, p.1);
                map.into_iter()
                    .map(|line| line.into_iter().filter(|cell| cell.1 != None).count())
                    .sum::<usize>()
            })
            .max()
            .and_then(|n| Some(n.to_string()));
        result
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn print_map(map: &Map) {
    for line in map {
        for char in line {
            match char.1 {
                None => print!("{}", char.0),
                _ => print!("{}", char.0.to_string().red()),
            }
        }
        println!();
    }
    println!();
}
fn mark_map(mut map: Map, current_p: &Point, direction: Direction) -> Map {
    if let Some(char) = map
        .clone()
        .get(current_p.x as usize)
        .and_then(|line| line.get(current_p.y as usize))
    {
        if char.1 == Some(direction.clone()) {
            return map;
        }
        map[current_p.x as usize][current_p.y as usize].1 = Some(direction.clone());
        // print_map(&map);
        if (char.0, &direction) == ('|', &Direction::N) {
            map = mark_map(map, &get_next_point(current_p, Direction::N), Direction::N);
        } else if (char.0, &direction) == ('|', &Direction::S) {
            map = mark_map(map, &get_next_point(current_p, Direction::S), Direction::S);
        } else if (char.0, &direction) == ('-', &Direction::E) {
            map = mark_map(map, &get_next_point(current_p, Direction::E), Direction::E);
        } else if (char.0, &direction) == ('-', &Direction::W) {
            map = mark_map(map, &get_next_point(current_p, Direction::W), Direction::W);

        // "\"
        } else if (char.0, &direction) == ('\\', &Direction::N) {
            map = mark_map(map, &get_next_point(current_p, Direction::W), Direction::W);
        } else if (char.0, &direction) == ('\\', &Direction::E) {
            map = mark_map(map, &get_next_point(current_p, Direction::S), Direction::S);
        } else if (char.0, &direction) == ('\\', &Direction::S) {
            map = mark_map(map, &get_next_point(current_p, Direction::E), Direction::E);
        } else if (char.0, &direction) == ('\\', &Direction::W) {
            map = mark_map(map, &get_next_point(current_p, Direction::N), Direction::N);

        // "/"
        } else if (char.0, &direction) == ('/', &Direction::N) {
            map = mark_map(map, &get_next_point(current_p, Direction::E), Direction::E);
        } else if (char.0, &direction) == ('/', &Direction::E) {
            map = mark_map(map, &get_next_point(current_p, Direction::N), Direction::N);
        } else if (char.0, &direction) == ('/', &Direction::S) {
            map = mark_map(map, &get_next_point(current_p, Direction::W), Direction::W);
        } else if (char.0, &direction) == ('/', &Direction::W) {
            map = mark_map(map, &get_next_point(current_p, Direction::S), Direction::S);

        // "."
        } else if char.0 == '.' {
            map = mark_map(
                map,
                &get_next_point(current_p, direction.clone()),
                direction.clone(),
            );
        // beam split
        } else if (char.0, &direction) == ('-', &Direction::S) {
            map = mark_map(map, &get_next_point(current_p, Direction::W), Direction::W);
            map = mark_map(map, &get_next_point(current_p, Direction::E), Direction::E);
        } else if (char.0, &direction) == ('-', &Direction::N) {
            map = mark_map(map, &get_next_point(current_p, Direction::W), Direction::W);
            map = mark_map(map, &get_next_point(current_p, Direction::E), Direction::E);
        } else if (char.0, &direction) == ('|', &Direction::E) {
            map = mark_map(map, &get_next_point(current_p, Direction::N), Direction::N);
            map = mark_map(map, &get_next_point(current_p, Direction::S), Direction::S);
        } else if (char.0, &direction) == ('|', &Direction::W) {
            map = mark_map(map, &get_next_point(current_p, Direction::N), Direction::N);
            map = mark_map(map, &get_next_point(current_p, Direction::S), Direction::S);
        }
    }
    map
}

fn get_next_point(current_p: &Point, direction: Direction) -> Point {
    match direction {
        Direction::N => Point {
            x: current_p.x - 1,
            y: current_p.y,
        },
        Direction::S => Point {
            x: current_p.x + 1,
            y: current_p.y,
        },
        Direction::W => Point {
            x: current_p.x,
            y: current_p.y - 1,
        },
        Direction::E => Point {
            x: current_p.x,
            y: current_p.y + 1,
        },
    }
}
type Map = Vec<Vec<(char, Option<Direction>)>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[test]
fn test_first() {
    let test_input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    let test_result = "46";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    let test_result = "51";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
