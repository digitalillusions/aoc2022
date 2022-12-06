use std::collections::HashSet;
use std::fs;

pub fn tuning_trouble() {
    println!("Running day 6");
    let contents = fs::read_to_string("sample_files/06/sample.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    for (i, block) in contents.windows(4).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(block);
        if set.len() == 4 {
            println!("{:?}", block);
            println!("\tPart 1: {}", i + 4);
            break;
        }
    }

    for (i, block) in contents.windows(14).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(block);
        if set.len() == 14 {
            println!("{:?}", block);
            println!("\tPart 2: {}", i + 14);
            break;
        }
    }
}
