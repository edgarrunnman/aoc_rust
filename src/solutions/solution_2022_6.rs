use super::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        for index in 4..self.input.len() {
            let slice = ['r', 'u', 's', 't'];
            let mut iter = slice.windows(2);
            let mut foo = self.input[(index - 4)..index]
                .chars()
                .into_iter()
                .collect::<Vec<char>>()
                .clone();
            print!("sliece [{} - {}] : {:?}", index - 4, index, foo);
            let foo = foo.drain(..);
            // if foo.len() == 4 {
            //     return Some(index.to_string());
            // };
            println!(" => {:?}", foo);
        }
        // let foo = self.input[0..4];
        // println!("{:?}", foo);
        None
    }

    fn solution_part_2(&self) -> Option<String> {
        None
    }
}
