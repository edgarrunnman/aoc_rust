use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let mut map = self
            .input
            .lines()
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Map>();
        map = extra_space(&map, 'H');
        map = swap_axis(&map);
        map = extra_space(&map, 'V');
        map = swap_axis(&map);
        let points = get_points(&map);
        let mut distances: Vec<usize> = vec![];
        let galaxies = points.iter().count().to_string().parse::<i32>().unwrap();
        let mut counter = 0;
        while (galaxies - counter) > 0 {
            (counter..galaxies - 1).into_iter().for_each(|i| {
                let dist = points.get(counter as usize).unwrap().distance(
                    points.get((i + 1) as usize).unwrap(),
                    &map,
                    2 as usize,
                );
                distances.push(dist)
            });
            counter += 1;
        }
        let result = distances.into_iter().sum::<usize>();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let mut map = self
            .input
            .lines()
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Map>();
        map = extra_space(&map, 'H');
        map = swap_axis(&map);
        map = extra_space(&map, 'V');
        map = swap_axis(&map);
        let points = get_points(&map);
        let mut distances: Vec<usize> = vec![];
        let galaxies = points.iter().count().to_string().parse::<i32>().unwrap();
        let mut counter = 0;
        while (galaxies - counter) > 0 {
            (counter..galaxies - 1).into_iter().for_each(|i| {
                let dist = points.get(counter as usize).unwrap().distance(
                    points.get((i + 1) as usize).unwrap(),
                    &map,
                    1000000 as usize,
                );
                distances.push(dist)
            });
            counter += 1;
        }
        let result = distances.into_iter().sum::<usize>();
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
struct Point {
    x: usize,
    y: usize,
}
type Map = Vec<Vec<char>>;

fn get_points(map: &Map) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];

    for (x, line) in map.into_iter().enumerate() {
        for (y, char) in line.into_iter().enumerate() {
            if char == &'#' {
                points.push(Point { x, y })
            }
        }
    }
    points
}
fn swap_axis(map: &Map) -> Map {
    let width = map.len();
    let height = map.first().unwrap().len();

    let mut swap_lines: Map = vec![vec!['?'; width]; height];
    for (x, line) in map.clone().into_iter().enumerate() {
        for (y, char) in line.into_iter().enumerate() {
            swap_lines[y][x] = char;
        }
    }
    swap_lines
}

fn extra_space(map: &Map, direction: char) -> Map {
    let mut new_map: Vec<Vec<char>> = vec![];
    for line in map {
        if line.into_iter().all(|char| ['.', 'V', 'H'].contains(char)) {
            new_map.push(line.into_iter().map(|_| direction).collect::<Vec<char>>());
        } else {
            new_map.push(line.clone());
        }
    }
    new_map
}

impl Point {
    fn distance(&self, other: &Point, map: &Map, space_multiplayer: usize) -> usize {
        let x_dif: usize;
        let y_dif: usize;
        if self.x > other.x {
            let extra_space = (other.x..self.x)
                .into_iter()
                .map(|x| map.get(x).unwrap().get(self.y).unwrap())
                .filter(|c| *c == &'H')
                .count();
            x_dif = self.x - other.x + extra_space * space_multiplayer - extra_space;
        } else {
            let extra_space = (self.x..other.x)
                .into_iter()
                .map(|x| map.get(x).unwrap().get(self.y).unwrap())
                .filter(|c| *c == &'H')
                .count();
            x_dif = other.x - self.x + extra_space * space_multiplayer - extra_space;
        }

        if self.y > other.y {
            let extra_space = (other.y..self.y)
                .into_iter()
                .map(|y| map.get(self.x).unwrap().get(y).unwrap())
                .filter(|c| *c == &'V')
                .count();
            y_dif = self.y - other.y + extra_space * space_multiplayer - extra_space;
        } else {
            let extra_space = (self.y..other.y)
                .into_iter()
                .map(|y| map.get(self.x).unwrap().get(y).unwrap())
                .filter(|c| *c == &'V')
                .count();
            y_dif = other.y - self.y + extra_space * space_multiplayer - extra_space;
        }
        x_dif + y_dif
    }
}

#[test]
fn test_first() {
    let test_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let test_result = "374";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let test_result = "8410";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
