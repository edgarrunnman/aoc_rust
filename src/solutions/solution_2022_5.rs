use super::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let (data, commands) = to_data_and_commands(&self.input);
        let mut stacks = make_stacks(&data);
        for command in parse_commands(commands) {
            for _ in 0..command.amount {
                let char = stacks[command.from - 1].pop().unwrap();
                stacks[command.to - 1].push(char);
            }
        }
        Some(
            stacks
                .iter()
                .map(|stack| stack.chars().into_iter().last())
                .fold(String::new(), |mut acc, char| {
                    acc.push(char.unwrap());
                    acc
                }),
        )
    }

    fn solution_part_2(&self) -> Option<String> {
        let (data, commands) = to_data_and_commands(&self.input);
        let mut stacks = make_stacks(&data);
        for command in parse_commands(commands) {
            let cargo: String = stacks[command.from - 1]
                .chars()
                .rev()
                .take(command.amount as usize)
                .collect();

            stacks[command.from - 1] = stacks[command.from - 1]
                .chars()
                .into_iter()
                .take(stacks[command.from - 1].len() - cargo.chars().into_iter().count())
                .collect();

            stacks[command.to - 1].push_str(&cargo);
        }
        Some(
            stacks
                .iter()
                .map(|stack| stack.chars().into_iter().last())
                .fold(String::new(), |mut acc, char| {
                    acc.push(char.unwrap());
                    acc
                }),
        )
    }
}

fn to_data_and_commands(input: &String) -> (String, String) {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    (String::from(data[0]), String::from(data[1]))
}

fn make_stacks(input: &String) -> Vec<String> {
    let lines = input.lines().rev().skip(1).collect::<Vec<&str>>();
    (0..(lines[0].len() + 1) / 4)
        .map(|n| make_stack(n, &lines))
        .collect::<Vec<String>>()
}

fn make_stack(index: usize, lines: &Vec<&str>) -> String {
    lines.iter().fold(String::new(), |mut stack, line| {
        line.chars().nth((4 * index) + 1).and_then(|char| {
            if char != ' ' {
                stack.push(char)
            }
            Some(char)
        });
        stack
    })
}

fn parse_commands(input: String) -> Vec<Command> {
    input.lines().map(from_line).collect()
}

fn from_line(line: &str) -> Command {
    let parts = line.split(" ").collect::<Vec<&str>>();
    Command {
        amount: parts[1].parse::<u8>().unwrap(),
        from: parts[3].parse::<usize>().unwrap(),
        to: parts[5].parse::<usize>().unwrap(),
    }
}

#[derive(Debug)]
struct Command {
    amount: u8,
    from: usize,
    to: usize,
}
