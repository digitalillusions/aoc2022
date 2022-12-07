use std::fs;

pub fn no_space_left() {
    println!("Running day 7");

    let contents = fs::read_to_string("sample_files/07/example.txt").unwrap();
}
