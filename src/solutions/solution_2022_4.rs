use super::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let result = self
            .input
            .lines()
            .map(make_range_pair)
            .filter(filter_overlapped)
            .count();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let result = self
            .input
            .lines()
            .map(make_range_pair)
            .filter(filter_touching)
            .count();
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn make_range_pair(line: &str) -> ((u32, u32), (u32, u32)) {
    let pair = line
        .split(',')
        .map(range_string_to_u32)
        .collect::<Vec<(u32, u32)>>();
    (pair[0], pair[1])
}

fn range_string_to_u32(string: &str) -> (u32, u32) {
    let range = string
        .split('-')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (range[0], range[1])
}

fn filter_overlapped(pair: &((u32, u32), (u32, u32))) -> bool {
    let left_overlapp = pair.0 .0 >= pair.1 .0 && pair.0 .1 <= pair.1 .1;
    let right_overlapp = pair.1 .0 >= pair.0 .0 && pair.1 .1 <= pair.0 .1;
    left_overlapp | right_overlapp
}
fn filter_touching(pair: &((u32, u32), (u32, u32))) -> bool {
    let touching = pair.1 .1 >= pair.0 .0 && pair.1 .0 <= pair.0 .1;
    touching
}
