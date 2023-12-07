use std::collections::HashMap;

use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let foo = self
            .input
            .split("\n\n")
            .map(|split| {
                let maping_raw = split
                    .split(':')
                    .map(|value| value.trim())
                    .collect::<Vec<&str>>();
                (*maping_raw.get(0).unwrap(), *maping_raw.get(1).unwrap())
            })
            .collect::<HashMap<&str, &str>>();
        let seeds = foo
            .get("seeds")
            .unwrap()
            .split(' ')
            .map(|seed| seed.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        let result = seeds
            .iter()
            .map(|seed| source_to_destination(*seed, foo.get("seed-to-soil map").unwrap()))
            .map(|seed| source_to_destination(seed, foo.get("soil-to-fertilizer map").unwrap()))
            .map(|seed| source_to_destination(seed, foo.get("fertilizer-to-water map").unwrap()))
            .map(|seed| source_to_destination(seed, foo.get("water-to-light map").unwrap()))
            .map(|seed| source_to_destination(seed, foo.get("light-to-temperature map").unwrap()))
            .map(|seed| {
                source_to_destination(seed, foo.get("temperature-to-humidity map").unwrap())
            })
            .map(|seed| source_to_destination(seed, foo.get("humidity-to-location map").unwrap()))
            .min()
            .unwrap();

        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let input = self
            .input
            .split("\n\n")
            .map(|split| {
                let maping_raw = split
                    .split(':')
                    .map(|value| value.trim())
                    .collect::<Vec<&str>>();
                (*maping_raw.get(0).unwrap(), *maping_raw.get(1).unwrap())
            })
            .collect::<HashMap<&str, &str>>();

        let seeds = input
            .get("seeds")
            .unwrap()
            .split(' ')
            .map(|seed| seed.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        let mappings = input
            .iter()
            .filter(|mapping| mapping.0 != &"seeds")
            .map(|mapping| (*mapping.0, make_mapping(mapping.1)))
            .collect::<HashMap<&str, Vec<MapRange>>>();

        let seed_ranges = (0..seeds.len())
            .step_by(2)
            .into_iter()
            .map(|index| {
                Range::from_range(*seeds.get(index).unwrap(), *seeds.get(index + 1).unwrap())
            })
            .collect::<Vec<Range>>();

        let result = seed_ranges
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(seed_range, mappings.get("seed-to-soil map").unwrap())
            })
            .reduce(append_ranges)?
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("soil-to-fertilizer map").unwrap(),
                )
            })
            .reduce(append_ranges)?
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("fertilizer-to-water map").unwrap(),
                )
            })
            .reduce(append_ranges)?
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("water-to-light map").unwrap(),
                )
            })
            .reduce(append_ranges)?
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("light-to-temperature map").unwrap(),
                )
            })
            .reduce(append_ranges)?
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("temperature-to-humidity map").unwrap(),
                )
            })
            .reduce(append_ranges)?
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("humidity-to-location map").unwrap(),
                )
            })
            .reduce(append_ranges)?
            .first()
            .unwrap()
            .min;
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
fn append_ranges(mut a: Vec<Range>, mut b: Vec<Range>) -> Vec<Range> {
    a.append(b.as_mut());
    a
}
fn source_rng_to_destination_rng(source_range: &Range, mapping: &Vec<MapRange>) -> Vec<Range> {
    let mut source_range = Range::from_max(source_range.min, source_range.max);
    let mut overlapping_map_ranges = mapping
        .into_iter()
        .filter(|mapping| {
            source_range.min < mapping.range.max && source_range.max > mapping.range.min
        })
        .map(|mapping| MapRange {
            range: Range::from_max(mapping.range.min, mapping.range.max),
            dif: mapping.dif,
        })
        .collect::<Vec<MapRange>>();
    overlapping_map_ranges.sort_by(|a, b| a.range.min.cmp(&b.range.min));

    let mut result_ranges: Vec<Range> = vec![];
    for map_range in &overlapping_map_ranges {
        let s_min = source_range.min;
        let s_max = source_range.max;
        let m_s_min = map_range.range.min;
        let m_s_max = map_range.range.max;
        let map_dif = map_range.dif;

        if s_min < m_s_max && s_min >= m_s_min && s_max <= m_s_max && s_max > m_s_min {
            //within
            let new_range = Range::from_max(s_min + map_dif, s_max + map_dif);
            result_ranges.push(new_range);
        } else if s_min < m_s_min && s_max > m_s_max {
            //contains
            let left_range = Range::from_max(s_min, m_s_min);
            let new_range = Range::from_max(m_s_min + map_dif, m_s_max + map_dif);
            let right_range = Range::from_max(m_s_max, s_max);
            result_ranges.push(left_range);
            result_ranges.push(new_range);
            source_range = right_range;
        } else if s_min < m_s_min && s_max > m_s_min && s_max <= m_s_max {
            //touching left
            let left_range = Range::from_max(s_min, m_s_min);
            let new_range = Range::from_max(m_s_min + map_dif, s_max + map_dif);
            result_ranges.push(left_range);
            result_ranges.push(new_range);
        } else if s_min >= m_s_min && s_min < m_s_max && s_max > m_s_max {
            //touching right
            let new_range = Range::from_max(s_min + map_dif, m_s_max + map_dif);
            let right_range = Range::from_max(m_s_max, s_max);
            result_ranges.push(new_range);
            source_range = right_range;
        } else if s_min == m_s_min && s_max == m_s_max {
            let new_range = Range::from_max(m_s_min + map_dif, m_s_max + map_dif);
            result_ranges.push(new_range);
        } else {
            //not touching
            let new_range = Range::from_max(source_range.min, source_range.max);
            result_ranges.push(new_range);
        }
    }
    if overlapping_map_ranges.len() == 0 {
        let s_min = source_range.min;
        let s_max = source_range.max;
        let new_range = Range::from_max(s_min, s_max);
        result_ranges.push(new_range);
    }
    result_ranges
}

fn make_mapping(map: &str) -> Vec<MapRange> {
    map.lines()
        .map(|line| {
            let line = line
                .split(" ")
                .map(|split| split.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            MapRange {
                range: Range::from_range(*line.get(1).unwrap(), *line.get(2).unwrap()),
                dif: *line.get(0).unwrap() - *line.get(1).unwrap(),
            }
        })
        .collect::<Vec<MapRange>>()
}

fn source_to_destination(source: i128, map: &str) -> i128 {
    let default_map = Mapping {
        source,
        destination: source,
        range: 1,
    };
    let mapping = map
        .lines()
        .map(|line| {
            let line = line
                .split(" ")
                .map(|split| split.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            Mapping {
                source: *line.get(1).unwrap(),
                destination: *line.get(0).unwrap(),
                range: *line.get(2).unwrap(),
            }
        })
        .filter(|mapping| (mapping.source..mapping.source + mapping.range).contains(&source))
        .collect::<Vec<Mapping>>();

    let mapping = mapping.get(0).unwrap_or(&default_map);
    source - mapping.source + mapping.destination
}

#[derive(Debug)]
struct Mapping {
    source: i128,
    destination: i128,
    range: i128,
}
#[derive(Debug)]
struct MapRange {
    range: Range,
    dif: i128,
}

#[derive(Debug)]
struct Range {
    min: i128,
    max: i128,
}
impl Range {
    fn from_range(min: i128, range: i128) -> Range {
        Range {
            min,
            max: min + range,
        }
    }
    fn from_max(min: i128, max: i128) -> Range {
        Range { min, max }
    }
}

#[test]
fn test_first() {
    let test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let test_result = "35";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let test_result = "46";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
