use crate::Solution;
use regex::Regex;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let foo = self
            .input
            .lines()
            .map(|line| line_to_arrangement(line))
            .collect::<Vec<Arrangement>>();
        // for fo in foo {
        //     println!("{:?}", fo);
        // }
        // foo.reverse();
        let result = foo
            .into_iter()
            // .take(1)
            .map(|arr| {
                arr.posible_combinations
                    .into_iter()
                    .filter(|cmb| combination_match(cmb, &arr.regex, &arr.regex_exp))
                    .count()
            })
            .sum::<usize>();

        // let foo = foo.get(1).unwrap().cl;
        // let foo = foo
        //     .posible_combinations
        //     .into_iter()
        //     .filter(|cmb| combination_match(cmb, &foo.regex, &foo.regex_exp))
        //     .count();
        println!("{:?}", result);

        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let foo = self
            .input
            .lines()
            .map(|line| line_to_arrangement(line))
            .collect::<Vec<Arrangement>>();
        // for fo in foo {
        //     println!("{:?}", fo);
        // }
        // foo.reverse();
        let result = foo
            .into_iter()
            // .take(1)
            .map(|arr| {
                arr.posible_combinations
                    .into_iter()
                    .filter(|cmb| combination_match(cmb, &arr.regex, &arr.regex_exp))
                    .count()
            })
            .sum::<usize>();

        // let foo = foo.get(1).unwrap().cl;
        // let foo = foo
        //     .posible_combinations
        //     .into_iter()
        //     .filter(|cmb| combination_match(cmb, &foo.regex, &foo.regex_exp))
        //     .count();
        println!("{:?}", result);

        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

#[allow(unused_variables)]
fn combination_match(combination: &String, regex: &Regex, regex_exp: &String) -> bool {
    match regex.find(combination.as_str()) {
        Some(_) => {
            // println!("{:?}   - {:?}", combination, regex_exp);
            true
        }
        None => {
            // println!("{:?}   - {:?}", combination, regex_exp);
            false
        }
    }
}

fn posible_combinations(data: &Vec<char>) -> Vec<String> {
    let unknwond_count = data.into_iter().filter(|char| char == &&'?').count();
    let mutations_count = (2 as u32).pow(unknwond_count as u32);
    let indexes_of_unknowns = data
        .into_iter()
        .enumerate()
        .map(|el| el)
        .filter(|(_, el)| el == &&'?')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    // println!("indexes_of_unknowns: {:?}", indexes_of_unknowns);
    //
    let mutations_in_bits = (0..mutations_count)
        .map(|i| number_to_bits(i))
        .collect::<DataSets>();
    // for fo in &mutations_in_bits {
    //     println!("{:?}", fo);
    // }
    let mut mutations = vec![];
    for bit_mutation in mutations_in_bits {
        let mut new_data = data
            .into_iter()
            .map(|char| match char {
                '?' => '.',
                _ => *char,
            })
            .collect::<Vec<char>>();

        for (index, bit) in bit_mutation.into_iter().enumerate() {
            let index_of_unknown = indexes_of_unknowns.get(index).unwrap();
            if bit == '0' {
                new_data[*index_of_unknown] = '.';
            } else {
                new_data[*index_of_unknown] = '#';
            }
        }

        mutations.push(new_data.into_iter().collect::<String>());
    }
    mutations
}

fn number_to_bits(mut mutations_count: u32) -> Vec<char> {
    let mut set_of_bits = vec![];
    loop {
        let m = mutations_count % 2;
        mutations_count = mutations_count / 2;

        set_of_bits.push(std::char::from_digit(m, 2).unwrap());
        if mutations_count == 0 {
            break;
        }
    }
    set_of_bits
}
fn reg_ex_expression(groups: &Vec<u32>) -> Regex {
    let mut regex_exp = groups
        .clone()
        .into_iter()
        .map(|i| format!("#{{{}}}", i))
        .collect::<Vec<String>>()
        .join("[.]+");
    regex_exp = "^[.]*".to_string() + regex_exp.as_str() + "[.]*$";
    Regex::new(regex_exp.as_str()).unwrap()
}

struct Arrangement {
    posible_combinations: Vec<String>,
    regex: Regex,
    regex_exp: String,
}
type Data = Vec<char>;
type DataSets = Vec<Data>;

fn line_to_arrangement(line: &str) -> Arrangement {
    let splits = line.split(' ').collect::<Vec<&str>>();
    let groups = splits
        .last()
        .unwrap()
        .repeat(5)
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let data = splits.first().unwrap().repeat(5).chars().collect();
    let mut regex_exp = groups
        .clone()
        .into_iter()
        .map(|i| format!("#{{{}}}", i))
        .collect::<Vec<String>>()
        .join("[.]+");
    regex_exp = "^[.]*".to_string() + regex_exp.as_str() + "[.]*$";
    Arrangement {
        posible_combinations: posible_combinations(&data),
        regex: reg_ex_expression(&groups),
        regex_exp,
    }
}
// ???.### 1,1,3 - 1 arrangement
// .??..??...?##. 1,1,3 - 4 arrangements
// ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
// ????.#...#... 4,1,1 - 1 arrangement
// ????.######..#####. 1,6,5 - 4 arrangements
// ?###???????? 3,2,1 - 10 arrangements
#[test]
fn test_first() {
    let test_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let test_result = "21";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let test_result = "525152";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
