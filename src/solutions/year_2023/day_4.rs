use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let result = self
            .input
            .lines()
            .into_iter()
            .map(|line| {
                let cards_with_numbers = line.split(": ").collect::<Vec<&str>>();
                let numbers_raw = cards_with_numbers
                    .get(1)
                    .unwrap()
                    .split(" | ")
                    .collect::<Vec<&str>>();
                let winning_numbers = normalize_numbers(numbers_raw.get(0).unwrap());
                let skrached_numbers = normalize_numbers(numbers_raw.get(1).unwrap());
                let point_numbers = skrached_numbers
                    .into_iter()
                    .filter(|game_card| winning_numbers.contains(&game_card))
                    .collect::<Vec<u32>>();
                if point_numbers.len() == 0 {
                    0
                } else {
                    (1..point_numbers.len()).fold(1, |c, _| c * 2 as u32)
                }
            })
            .sum::<u32>();
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let mut card_register = self
            .input
            .lines()
            .into_iter()
            .map(|line| {
                let cards_with_numbers = line.split(": ").collect::<Vec<&str>>();
                let numbers_raw = cards_with_numbers
                    .get(1)
                    .unwrap()
                    .split(" | ")
                    .collect::<Vec<&str>>();
                let winning_numbers = normalize_numbers(numbers_raw.get(0).unwrap());
                let skrached_numbers = normalize_numbers(numbers_raw.get(1).unwrap());
                let copies_to_make = skrached_numbers
                    .into_iter()
                    .filter(|skrached_number| winning_numbers.contains(&skrached_number))
                    .collect::<Vec<u32>>()
                    .len();
                let initial_amount: u32 = 1;
                (copies_to_make, initial_amount)
            })
            .collect::<Vec<(usize, u32)>>();

        let mut current_index: usize = 0;

        while current_index < card_register.len() {
            let card = card_register.get(current_index).unwrap().clone();
            (0..card.1).for_each(|_| {
                (0..card.0).for_each(|n| {
                    let target_index = current_index + 1 + n;
                    if target_index < card_register.len() {
                        card_register[target_index].1 += 1
                    }
                })
            });
            current_index += 1;
        }
        let result = card_register.into_iter().map(|item| item.1).sum::<u32>();
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

fn normalize_numbers(cards_string: &str) -> Vec<u32> {
    cards_string
        .replace("  ", " ")
        .trim()
        .split(" ")
        .into_iter()
        .map(|card| card.parse::<u32>().unwrap())
        .collect()
}

#[test]
fn test_first() {
    let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let test_result = "13";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}

#[test]
fn test_second() {
    let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let test_result = "30";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
