use std::collections::HashMap;

use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let mut hands = self
            .input
            .lines()
            .map(|line| {
                let line = line.trim();
                let line = line.split(' ').collect::<Vec<&str>>();
                HandWithBid::new(
                    line.get(0).unwrap(),
                    line.get(1).unwrap().parse::<u16>().unwrap(),
                )
            })
            .collect::<Vec<HandWithBid>>();
        hands.sort();
        let mut result: u64 = 0;
        for (i, hand) in hands.iter().enumerate() {
            result = result + (i as u64 + 1) * hand.bid as u64;
        }
        Some(result.to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        let mut hands = self
            .input
            .lines()
            .map(|line| {
                let line = line.trim();
                let line = line.split(' ').collect::<Vec<&str>>();
                HandWithBid::new_with_jokers(
                    line.get(0).unwrap(),
                    line.get(1).unwrap().parse::<u16>().unwrap(),
                )
            })
            .collect::<Vec<HandWithBid>>();
        hands.sort();
        let mut result: u64 = 0;
        for (i, hand) in hands.iter().enumerate() {
            result = result + (i as u64 + 1) * hand.bid as u64;
        }
        Some(result.to_string())
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
#[derive(Debug)]
struct HandWithBid {
    hand: Vec<char>,
    bid: u16,
}

impl HandWithBid {
    fn new(hand: &str, bid: u16) -> HandWithBid {
        let hand = hand.chars().collect::<Vec<char>>();
        HandWithBid { hand, bid }
    }
    fn new_with_jokers(hand: &str, bid: u16) -> HandWithBid {
        let hand = hand.replace("J", "*").chars().collect::<Vec<char>>();
        HandWithBid { hand, bid }
    }
    fn hand_power(&self) -> u8 {
        let mut map: HashMap<char, i8> = HashMap::new();
        for char in &self.hand {
            if let Some(count) = map.get_mut(char) {
                *count += 1;
            } else {
                map.insert(*char, 1);
            }
        }
        let jokers_count = map.remove_entry(&'*').unwrap_or(('*', 0)).1;
        let mut map = map.iter().map(|pair| pair.1.clone()).collect::<Vec<i8>>();
        map.sort();
        if jokers_count > 0 {
            let mut last = map.pop().unwrap_or_default();
            last = last + jokers_count;
            map.push(last);
        }
        if map.get(4) == Some(&1) {
            0
        } else if map.get(3) == Some(&2) {
            1
        } else if map.get(1) == Some(&2) && map.get(2) == Some(&2) {
            2
        } else if map.get(2) == Some(&3) {
            3
        } else if map.get(0) == Some(&2) && map.get(1) == Some(&3) {
            4
        } else if map.get(1) == Some(&4) {
            5
        } else {
            6
        }
    }
}

fn card_power(card: &char) -> u8 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        '*' => 0,
        _ => panic!("invalid char"),
    }
}
impl Ord for HandWithBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            let a_hand = self.hand.clone();
            let b_hand = other.hand.clone();

            for i in 0..5 {
                let a = card_power(a_hand.get(i).unwrap());
                let b = card_power(b_hand.get(i).unwrap());
                if a != b {
                    return a.cmp(&b);
                }
            }
        }
        self.hand_power().cmp(&other.hand_power())
    }
}

impl Eq for HandWithBid {}
impl PartialOrd for HandWithBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for HandWithBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand_power() == other.hand_power()
    }
}

#[test]
fn test_first() {
    let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let test_result = "6440";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let test_result = "5905";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
