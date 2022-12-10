use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    subdirs: HashMap<String, Rc<RefCell<Directory>>>,
    files: BTreeSet<(u64, String)>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            name: "/".to_string(),
            parent: None,
            subdirs: HashMap::new(),
            files: BTreeSet::new(),
        }
    }

    fn with_name(self, name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            ..self
        }
    }

    fn with_parent(self, parent: Rc<RefCell<Directory>>) -> Directory {
        Directory {
            parent: Some(parent),
            ..self
        }
    }
}

pub fn no_space_left() {
    println!("Running day 7");

    let contents = fs::read_to_string("sample_files/07/sample.txt").unwrap();

    let root_dir = Rc::new(RefCell::new(Directory::new()));
    let mut current_dir: Rc<RefCell<Directory>> = root_dir.clone();
    let mut file_size_count = 0;

    for line in contents.lines() {
        // Command
        if let Some((_, command)) = line.split_once("$") {
            // Change directory
            if let Some((_, directory)) = command.trim().split_once("cd") {
                let directory = directory.trim();
                println!("Change dir: {}", directory);
                current_dir = match directory {
                    ".." => current_dir.borrow().parent.clone().unwrap(),
                    _ => {
                        if !current_dir.borrow().subdirs.contains_key(directory) {
                            let new_dir = Rc::new(RefCell::new(
                                Directory::new()
                                    .with_name(directory)
                                    .with_parent(current_dir.clone()),
                            ));
                            current_dir
                                .borrow_mut()
                                .subdirs
                                .insert(directory.to_string(), new_dir.clone());
                            new_dir
                        } else {
                            current_dir.borrow().subdirs.get(directory).unwrap().clone()
                        }
                    }
                }
            } else if let Some((_, _)) = command.trim().split_once("ls") {
                println!("Listdir: {}", line)
            }
        } else {
            // Only option left is to read directory contents
            if let Some((left, dir_or_file)) = line.split_once(" ") {
                // Read directory and files
                if left == "dir" && !current_dir.borrow().subdirs.contains_key(dir_or_file) {
                    let new_dir = Rc::new(RefCell::new(Directory::new().with_name(dir_or_file)));
                    current_dir
                        .borrow_mut()
                        .subdirs
                        .insert(dir_or_file.to_string(), new_dir.clone());
                } else if let Some(file_size) = left.parse::<u64>().ok() {
                    if !current_dir
                        .borrow()
                        .files
                        .contains(&(file_size, dir_or_file.to_string()))
                    {
                        current_dir
                            .borrow_mut()
                            .files
                            .insert((file_size, dir_or_file.to_string()));
                        file_size_count += file_size;
                    }
                }
            }
        }
    }

    let mut list_of_dirs: Vec<(String, u64)> = Vec::new();
    let total_fs_size = eval_dir_sizes("".to_string(), root_dir, &mut list_of_dirs);
    println!("Total fs: {:?}", list_of_dirs);
    println!("Total fs size: {}", total_fs_size);
    println!("\tPart 1: {}", file_size_count);
}

fn eval_dir_sizes(
    path_to_current_root: String,
    current_root: Rc<RefCell<Directory>>,
    list_of_dirs: &mut Vec<(String, u64)>,
) -> u64 {
    let new_root = vec![path_to_current_root, current_root.borrow().name.clone()].join("/");

    let subdir_sizes = current_root
        .borrow()
        .subdirs
        .iter()
        .map(|(_, v)| eval_dir_sizes(new_root.clone(), v.clone(), list_of_dirs))
        .sum::<u64>();

    let file_sizes = current_root
        .borrow()
        .files
        .iter()
        .map(|(size, _)| size)
        .sum::<u64>();

    list_of_dirs.push((new_root, subdir_sizes + file_sizes));

    subdir_sizes + file_sizes
}
