use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs;
use std::rc::Rc;

#[derive(Debug, PartialEq, PartialOrd)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    sub: BTreeSet<Rc<RefCell<Directory>>>,
    sub_names: BTreeSet<String>,
    files: BTreeSet<(usize, String)>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            name: "/".to_string(),
            parent: None,
            sub: BTreeSet::new(),
            sub_names: BTreeSet::new(),
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

    let contents = fs::read_to_string("sample_files/07/example.txt").unwrap();

    let root_dir = Rc::new(RefCell::new(Directory::new()));
    let mut current_dir: Rc<RefCell<Directory>> = root_dir;

    for line in contents.lines() {
        // Command
        if let Some((_, command)) = line.split_once("$") {
            // Change directory
            if let Some((_, directory)) = command.trim().split_once("cd") {
                println!("Change dir: {}", directory.trim());
                match directory {
                    ".." => {
                        let new_dir = current_dir.borrow().parent.clone().unwrap();
                        current_dir = new_dir;
                    }
                    _ => {
                        if !current_dir.borrow().sub_names.contains(directory) {
                            current_dir
                                .borrow_mut()
                                .sub_names
                                .insert(directory.to_string());
                            let next_dir =
                                Rc::new(RefCell::new(Directory::new().with_name(directory)));
                            // insert new directory
                        }
                    }
                }
            } else if let Some((_, _)) = command.trim().split_once("ls") {
                println!("Listdir: {}", line)
            }
        }
    }
}
