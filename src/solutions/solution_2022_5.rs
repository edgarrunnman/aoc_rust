use super::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let (data, commands) = to_data_and_commands(&self.input);
        make_stacks(&data);
        None
    }

    fn solution_part_2(&self) -> Option<String> {
        None
    }
}

fn to_data_and_commands(input: &String) -> (String, String) {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    (String::from(data[0]), String::from(data[1]))
}

fn make_stacks(input: &String) -> Vec<String> {
    let lines = input.lines().rev().skip(1);
    for line in &lines.clone().collect::<Vec<&str>>() {
        println!("{:?}", line);
    }
    let stacks = (0..(lines.collect::<Vec<&str>>()[0].len() + 1) / 4)
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", stacks);
    vec![]
}
