use std::fs;
use std::rc::Rc;

struct Directory {
    name: String,
    parent: Option<Rc<Directory>>,
    sub: Option<Vec<Rc<Directory>>>,
    files: Option<Vec<(usize, String)>>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            name: "/".to_string(),
            parent: None,
            sub: Some(Vec::new()),
            files: Some(Vec::new()),
        }
    }

    fn with_name(self, name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            ..self
        }
    }

    fn with_parent(self, parent: Rc<Directory>) -> Directory {
        Directory {
            parent: Some(parent),
            ..self
        }
    }
}

pub fn no_space_left() {
    println!("Running day 7");

    let contents = fs::read_to_string("sample_files/07/example.txt").unwrap();

    let mut directory_list = Vec::<Rc<Directory>>::new();
    directory_list.push(Rc::new(Directory::new().with_name("/")));
}
