use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let mut cmd_state = "w8";
        let mut current_directory = Directory::new("root", None);

        for line in self.input.lines().into_iter() {
            if line == "$ ls" {
                cmd_state = "reading";
                continue;
            };
            if line.to_string().starts_with("dir ") {
                let dir_name = *line.split(" ").collect::<Vec<&str>>().get(1).unwrap();
                let new_dir = Directory::new(dir_name, None);
                current_directory.directories.push(new_dir);
                continue;
            }
            if line.to_string().chars().next().unwrap().is_digit(10) {
                let size = line
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let name = *line.split(" ").collect::<Vec<&str>>().get(1).unwrap();
                let new_file = File { name, size };
                current_directory.files.push(new_file);
                continue;
            }
            if line.to_string().starts_with("$ cd ") {
                let dir_name = *line.split(" ").collect::<Vec<&str>>().get(2).unwrap();
                current_directory = current_directory
                    .directories
                    .into_iter()
                    .find(|dir| dir.name == dir_name)
                    .unwrap();
                continue;
            }
        }
        None
    }

    fn solution_part_2(&self) -> Option<String> {
        None
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}

#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    parent_directory: Option<&'a Directory<'a>>,
    directories: Vec<Directory<'a>>,
    files: Vec<File<'a>>,
}

// impl<'a> Clone for Directory<'a>
//     fn clone(&self) -> Self {
//         Self {
//             name: self.name.clone(),
//             directories: self.directories.clone(),
//             files: self.files.clone(),
//         }
//     }
// }

impl<'a, 'b: 'a> Directory<'a> {
    fn new(name: &'a str, parent_dir: Option<&'b Directory>) -> Directory<'a> {
        Directory {
            name,
            parent_directory: parent_dir,
            directories: vec![],
            files: vec![],
        }
    }
    // fn add_directory(&mut self, directory: &'a Directory) {
    //     self.directories.push(directory);
    // }
}

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: u32,
}

// impl<'a> Clone for File<'a> {
//     fn clone(&self) -> Self {
//         Self {
//             name: self.name.clone(),
//             size: self.size.clone(),
//         }
//     }
// }

#[test]
fn test_first() {
    let test_input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let test_result = "95437";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_1().unwrap())
}
#[test]
fn test_second() {
    let test_input = "";
    let test_result = "";
    let test_input = String::from(test_input);
    let test_result = String::from(test_result);
    let solution = SolutionImp::new(test_input);
    assert_eq!(test_result, solution.solution_part_2().unwrap())
}
