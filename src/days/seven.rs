use std::fs;
use std::rc::Rc;

struct Directory {
    parent: Option<Rc<Directory>>,
    sub: Option<Vec<Rc<Directory>>>,
    files: Option<Vec<(usize, String)>>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            parent: None,
            sub: Some(Vec::new()),
            files: Some(Vec::new()),
        }
    }
}

pub fn no_space_left() {
    println!("Running day 7");

    let contents = fs::read_to_string("sample_files/07/example.txt").unwrap();

    let mut filesystem = Directory::new();
}
