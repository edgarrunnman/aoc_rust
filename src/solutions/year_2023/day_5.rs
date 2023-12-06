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
        let log = true;
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

        // println!("seed ranges {:?}", seed_ranges);
        // println!("");
        // println!("");
        // for range in &seed_ranges {
        //     println!("seed ranges {:?}", range);
        // }

        let mappings = foo
            .iter()
            .filter(|mapping| mapping.0 != &"seeds")
            .map(|mapping| (*mapping.0, make_mapping_v2(mapping.1)))
            .collect::<HashMap<&str, Vec<MapRange>>>();

        // let seed_ranges = (0..seeds.len())
        //     .step_by(2)
        //     .into_iter()
        //     .map(|index| {
        //         Range::from_range(*seeds.get(index).unwrap(), *seeds.get(index + 1).unwrap())
        //     })
        //     .take(3)
        //     .collect::<Vec<Range>>()
        //     .drain(2..)
        //     .collect::<Vec<Range>>();
        let seed_ranges = (0..seeds.len())
            .step_by(2)
            .into_iter()
            .map(|index| {
                Range::from_range(*seeds.get(index).unwrap(), *seeds.get(index + 1).unwrap())
            })
            .collect::<Vec<Range>>();

        println!("seed_ranges");
        for range in &seed_ranges {
            println!("{:?}", range);
        }
        println!("");
        let seed_to_soil = seed_ranges
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(seed_range, mappings.get("seed-to-soil map").unwrap())
                    .into_iter()
                    .collect::<Vec<Range>>()
            })
            .into_iter()
            .fold(vec![], |mut acc, mut ranges| {
                acc.append(ranges.as_mut());
                acc as Vec<Range>
            });
        if log {
            println!("after seed_to_soil");
            for range in &seed_to_soil {
                println!("after seed to soil {:?}", range);
            }
            println!("");
        }

        let soil_to_fertilizer = sort_and_dedup_ranges(seed_to_soil)
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("soil-to-fertilizer map").unwrap(),
                )
                .into_iter()
                .collect::<Vec<Range>>()
            })
            .into_iter()
            .fold(vec![], |mut acc, mut ranges| {
                acc.append(ranges.as_mut());
                acc as Vec<Range>
            });

        if log {
            println!("after soil to fertilizer");
            for range in &soil_to_fertilizer {
                println!("{:?}", range);
            }
            println!("");
            println!("");
        }
        let fertilizer_to_water = sort_and_dedup_ranges(soil_to_fertilizer)
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("fertilizer-to-water map").unwrap(),
                )
                .into_iter()
                .collect::<Vec<Range>>()
            })
            .into_iter()
            .fold(vec![], |mut acc, mut ranges| {
                acc.append(ranges.as_mut());
                acc as Vec<Range>
            });

        if log {
            println!("after fertilizer_to_water");
            for range in &fertilizer_to_water {
                println!("{:?}", range);
            }
            println!("");
            println!("");
        }
        let water_to_light = sort_and_dedup_ranges(fertilizer_to_water)
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("water-to-light map").unwrap(),
                )
                .into_iter()
                .collect::<Vec<Range>>()
            })
            .into_iter()
            .fold(vec![], |mut acc, mut ranges| {
                acc.append(ranges.as_mut());
                acc as Vec<Range>
            });

        if log {
            println!("after water_to_ligt ");
            for range in &water_to_light {
                println!("{:?}", range);
            }
            println!("");
            println!("");
        }
        let light_to_temperature = sort_and_dedup_ranges(water_to_light)
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("light-to-temperature map").unwrap(),
                )
                .into_iter()
                .collect::<Vec<Range>>()
            })
            .into_iter()
            .fold(vec![], |mut acc, mut ranges| {
                acc.append(ranges.as_mut());
                acc as Vec<Range>
            });

        if log {
            println!("after ligcht_to_temperature ");

            for range in &light_to_temperature {
                println!("{:?}", range);
            }
            println!("");
            println!("");
        }
        let temperature_to_humidity = sort_and_dedup_ranges(light_to_temperature)
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("temperature-to-humidity map").unwrap(),
                )
                .into_iter()
                .collect::<Vec<Range>>()
            })
            .into_iter()
            .fold(vec![], |mut acc, mut ranges| {
                acc.append(ranges.as_mut());
                acc as Vec<Range>
            });

        if log {
            println!("after temperature_to_humidity",);
            for range in &temperature_to_humidity {
                println!("{:?}", range);
            }
            println!("");
            println!("");
        }
        let humidity_to_location = sort_and_dedup_ranges(temperature_to_humidity)
            .iter()
            .map(|seed_range| {
                source_rng_to_destination_rng(
                    seed_range,
                    mappings.get("humidity-to-location map").unwrap(),
                )
                .into_iter()
                .collect::<Vec<Range>>()
            })
            .into_iter()
            .fold(vec![], |mut acc, mut ranges| {
                acc.append(ranges.as_mut());
                acc as Vec<Range>
            });
        let result = sort_and_dedup_ranges(humidity_to_location);
        if log {
            println!("");
            println!("restult ranges");
            println!("");
            for range in &result {
                println!("{:?}", range);
            }
        }
        let resut = result.first().unwrap().min;
        Some(resut.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn sort_and_dedup_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    let log = true;
    ranges.sort_by(|a, b| a.min.cmp(&b.min));
    ranges.dedup_by(|a, b| (a.min, b.min) == (a.max, b.max));
    if log {
        println!("merging ranges");
        for range in &ranges {
            println!("{:?}", range);
        }
    }
    let ranges = merge_ranges(ranges);
    if log {
        println!("");
        println!("done merg");
        for range in &ranges {
            println!("{:?}", range);
        }
        println!("");
    }
    ranges
}

fn source_rng_to_destination_rng(source_rng: &Range, mapping: &Vec<MapRange>) -> Vec<Range> {
    let log = false;
    let mut result_ranges: Vec<Range> = vec![];
    let s_min = &source_rng.min;
    let s_max = &source_rng.max;

    for map_range in mapping {
        let mut add_result_ranges = execute_mapping(source_rng, map_range);
        result_ranges.append(add_result_ranges.as_mut());
    }

    if result_ranges.is_empty() {
        // println!(" - NO MAPPING");
        let new_range = Range::from_max(*s_min, *s_max);
        result_ranges.push(new_range);
    }
    if log {
        println!("");
    }
    sort_and_dedup_ranges(result_ranges)
    // result_ranges
}
fn execute_mapping(source_range: &Range, map_range: &MapRange) -> Vec<Range> {
    let log = true;
    let s_min = &source_range.min;
    let s_max = &source_range.max;
    let m_s_min = &map_range.range.min;
    let m_s_max = &map_range.range.max;
    let map_dif = &map_range.dif;

    if log {}
    let mut result_ranges: Vec<Range> = vec![];
    // println!(
    //     "source: {:?} and map_ranges {:?}: diff = {:?}",
    //     source_rng, map_ranges.range, map_dif
    // );
    if s_min > m_s_max || s_max < m_s_min {
        // println!(" - not touching");
        //not touching
    } else if s_min <= m_s_max && s_min >= m_s_min && s_max <= m_s_max && s_max >= m_s_min {
        //within
        let new_range = Range::from_max(s_min + map_dif, s_max + map_dif);
        // if new_range.min == 0 {
        //     print!(
        //         "{:?} and {:?}: diff = {:?}",
        //         source_rng, map_ranges.range, map_dif
        //     );
        //     println!(" - within: resut {:?}", new_range);
        // }
        if log {
            print!(
                "{:?} and {:?}: diff = {:?}",
                source_range, map_range.range, map_dif
            );
            println!(" - within: resut {:?}", new_range);
            println!("");
        }
        result_ranges.push(new_range);
    } else if s_min <= m_s_min && s_max >= m_s_max {
        //contains
        let left_range = Range::from_max(*s_min, m_s_min - 1);
        let new_range = Range::from_max(m_s_min + map_dif, m_s_max + map_dif);
        let right_range = Range::from_max(m_s_max + 1, *s_max);
        // if left_range.min == 0 || new_range.min == 0 || right_range.min == 0 {
        //     print!(
        //         "{:?} and {:?}: diff = {:?}",
        //         source_rng, map_ranges.range, map_dif
        //     );
        //     println!(
        //         " -contains: resut {:?}:{:?}:{:?}",
        //         left_range, new_range, right_range
        //     );
        // }
        if log {
            print!(
                "{:?} and {:?}: diff = {:?}",
                source_range, map_range.range, map_dif
            );
            println!(
                " -contains: result {:?}:{:?}:{:?}",
                left_range, new_range, right_range
            );
            println!("");
        }
        result_ranges.push(left_range);
        result_ranges.push(new_range);
        result_ranges.push(right_range);
    } else if s_min <= m_s_min && s_max >= m_s_min && s_max <= m_s_max {
        //touching left
        let left_range = Range::from_max(*s_min, m_s_min - 1);
        let new_range = Range::from_max(m_s_min + map_dif, s_max + map_dif);
        // if left_range.min == 0 || new_range.min == 0 {
        //     print!(
        //         "{:?} and {:?}: diff = {:?}",
        //         source_rng, map_ranges.range, map_dif
        //     );
        //     println!(" -touching left: resut {:?}:{:?}", left_range, new_range);
        // }
        if log {
            print!(
                "{:?} and {:?}: diff = {:?}",
                source_range, map_range.range, map_dif
            );
            println!(" -touching left: result {:?}:{:?}", left_range, new_range);
            println!("");
        }
        result_ranges.push(new_range);
        result_ranges.push(left_range);
    } else if s_min >= m_s_min && s_min <= m_s_max && s_max >= m_s_max {
        //touching right
        let new_range = Range::from_max(s_min + map_dif, m_s_max + map_dif);
        let right_range = Range::from_max(m_s_max + 1, *s_max);
        // if right_range.min == 0 || new_range.min == 0 {
        //     print!(
        //         "{:?} and {:?}: diff = {:?}",
        //         source_rng, map_ranges.range, map_dif
        //     );
        //     println!(" -touching right: resut {:?}:{:?}", new_range, right_range);
        // }
        if log {
            print!(
                "{:?} and {:?}: diff = {:?}",
                source_range, map_range.range, map_dif
            );
            println!(" -touching right: result {:?}:{:?}", new_range, right_range);
            println!("");
            println!("");
        }
        result_ranges.push(new_range);
        result_ranges.push(right_range);
    } else if s_min == m_s_min && s_max == m_s_max {
        let new_range = Range::from_max(m_s_min + map_dif, m_s_max + map_dif);
        result_ranges.push(new_range);
    }
    result_ranges
}
fn make_mapping_v2(map: &str) -> Vec<MapRange> {
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

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by(|a, b| a.min.cmp(&b.min));
    ranges.into_iter().fold(vec![], |mut acc, b| {
        if acc.len() > 0 {
            let a = acc.pop().unwrap();
            if a.max + 1 >= b.min && a.max >= b.max {
                let new_range = Range {
                    min: a.min,
                    max: a.max,
                };
                acc.push(new_range);
            } else if a.max + 1 >= b.min && a.max < b.max {
                let new_range = Range {
                    min: a.min,
                    max: b.max,
                };
                acc.push(new_range);
            } else {
                acc.push(a);
                acc.push(b);
            }
        } else {
            acc.push(b);
        }
        acc as Vec<Range>
    })
}

fn source_to_destination(source: i128, map: &str) -> i128 {
    let default_map = Mapping {
        source,
        destination: source,
        range: 1,
    };
    let foo = map
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

    let foo = foo.get(0).unwrap_or(&default_map);
    source - foo.source + foo.destination
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
            max: min + range - 1,
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

#[test]
fn range_mapping_contains() {
    let source_range = Range::from_max(100, 200);
    let map_range = MapRange {
        range: Range::from_max(150, 160),
        dif: 10,
    };
    let result_ranges = execute_mapping(&source_range, &map_range);
    println!("");
    println!("");
    for resut in result_ranges {
        println!("{:?}", resut)
    }
}
#[test]
fn range_mapping_within_borders_not_touching() {
    let source_range = Range::from_max(151, 159);
    let map_range = MapRange {
        range: Range::from_max(150, 160),
        dif: 10,
    };
    let result_ranges = execute_mapping(&source_range, &map_range);
    println!("");
    println!("");
    for resut in result_ranges {
        println!("{:?}", resut)
    }
}
#[test]
fn range_mapping_within_both_borders_touching() {
    let source_range = Range::from_max(151, 159);
    let map_range = MapRange {
        range: Range::from_max(150, 160),
        dif: 10,
    };
    let result_ranges = execute_mapping(&source_range, &map_range);
    println!("");
    println!("");
    for resut in result_ranges {
        println!("{:?}", resut)
    }
}
