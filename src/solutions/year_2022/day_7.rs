use std::collections::{hash_map, HashMap};

use crate::Solution;

pub struct SolutionImp {
    pub input: String,
}

impl Solution<SolutionImp> for SolutionImp {
    fn solution_part_1(&self) -> Option<String> {
        let mut directory_container: HashMap<u8, Directory> = HashMap::new();
        let mut current_directory_id: u8 = 0;
        directory_container.insert(
            current_directory_id,
            Directory::new(current_directory_id, "/".to_string(), None),
        );
        println!("initial container {:?}", directory_container);
        for line in self.input.lines().into_iter() {
            if line.to_string().starts_with("dir ") {
                let dir_name = *line.split(" ").collect::<Vec<&str>>().get(1).unwrap();
                let dir_id = get_new_id(&directory_container);
                let new_dir =
                    Directory::new(dir_id, dir_name.to_string(), Some(current_directory_id));
                let mut dir_owned = directory_container
                    .get(&current_directory_id)
                    .unwrap()
                    .to_owned();
                dir_owned.add_directory(dir_name.to_string(), dir_id);
                directory_container.insert(dir_owned.id, dir_owned);
                directory_container.insert(new_dir.id, new_dir);
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
                let mut dir_owned = directory_container
                    .get(&current_directory_id)
                    .unwrap()
                    .to_owned();
                dir_owned.add_file(name.to_string(), size);
                directory_container.insert(dir_owned.id, dir_owned);
            }
            if line.to_string().starts_with("$ cd ")
                && !line.to_string().eq("$ cd ..")
                && !line.to_string().eq("$ cd /")
            {
                let dir_name = *line.split(" ").collect::<Vec<&str>>().get(2).unwrap();

                current_directory_id = directory_container
                    .get(&current_directory_id)
                    .unwrap()
                    .get_sub_directory_ref(dir_name);
            }
            if line.to_string().eq("$ cd ..") {
                current_directory_id = directory_container
                    .get(&current_directory_id)
                    .unwrap()
                    .get_parent_dir_ref();
            }
        }
        print!("{:?}", directory_container);
        Some("".to_string())
    }

    fn solution_part_2(&self) -> Option<String> {
        None
    }

    fn new(input: String) -> SolutionImp {
        SolutionImp { input }
    }
}
fn get_new_id(register: &HashMap<u8, Directory>) -> u8 {
    register.len() as u8
}

#[derive(Debug)]
struct Directory {
    id: u8,
    name: String,
    parent_ref: Option<u8>,
    directorie_refs: HashMap<String, u8>,
    files: Vec<File>,
}

impl Clone for Directory {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            parent_ref: self.parent_ref.clone(),
            directorie_refs: self.directorie_refs.clone(),
            files: self.files.clone(),
        }
    }
}

impl Directory {
    fn new(id: u8, name: String, parent_ref: Option<u8>) -> Directory {
        Directory {
            id,
            name,
            parent_ref,
            directorie_refs: HashMap::new(),
            files: vec![],
        }
    }
    fn get_sub_directory_ref(&self, name: &str) -> u8 {
        self.directorie_refs.get(name).unwrap().clone()
    }
    fn get_parent_dir_ref(&self) -> u8 {
        self.parent_ref.unwrap().clone()
    }
    fn add_file(&mut self, name: String, size: u32) {
        self.files.push(File { name, size });
    }
    fn add_directory(&mut self, key: String, id: u8) {
        self.directorie_refs.insert(key, id);
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl Clone for File {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            size: self.size.clone(),
        }
    }
}

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
