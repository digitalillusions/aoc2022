use std::{collections::BTreeSet, fs};

pub fn rope_bridge() {
    let content = fs::read_to_string("sample_files/09/example.txt").unwrap();
    let instructions = content
        .lines()
        .map(|line| {
            let (dir, count) = line.split_once(" ").unwrap();
            (dir, count.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    println!("Instructions: {:?}", instructions);

    let mut head_loc: (u32, u32) = (0, 0);
    let mut tail_loc: (u32, u32) = (0, 0);

    let mut all_tail_locs: BTreeSet<(u32, u32)> = BTreeSet::new();

    for (dir, count) in instructions.iter() {}
}
