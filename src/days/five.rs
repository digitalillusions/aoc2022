use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn supply_stacks() {
    println!("Running day 5");
    // let content = fs::read_to_string("sample_files/05/example.txt").unwrap();
    let content = fs::read_to_string("sample_files/05/sample.txt").unwrap();
    let mut stacks_str = Vec::new();
    let mut stacks = HashMap::<u32, VecDeque<char>>::new();
    let mut columns = HashMap::new();
    let mut instructions = Vec::new();

    for line in content.lines() {
        if !line.is_empty() {
            if line.contains("move") {
                let (_, instruction) = line.split_once("move").unwrap();
                let (a, instruction) = instruction.split_once("from").unwrap();
                let (b, c) = instruction.split_once("to").unwrap();
                instructions.push((
                    a.trim().parse::<u32>().unwrap(),
                    b.trim().parse::<u32>().unwrap(),
                    c.trim().parse::<u32>().unwrap(),
                ));
            } else {
                let mut remaining_line = line;
                let mut loc_count = 0;
                if line.contains("1") {
                    while let Some(loc) = remaining_line.find(|c: char| !c.is_whitespace()) {
                        let column = remaining_line[loc..loc + 1].parse::<u32>().unwrap();
                        stacks.insert(column, VecDeque::<char>::new());

                        loc_count += loc;
                        columns.insert(column, loc_count);

                        remaining_line = &remaining_line[loc + 1..];
                        loc_count += 1;
                    }
                } else {
                    stacks_str.push(line);
                }
            }
        }
    }

    for stack in stacks_str {
        for (column, loc) in columns.iter() {
            let bx = &stack[*loc..loc + 1];
            if !bx.trim().is_empty() {
                stacks
                    .get_mut(column)
                    .unwrap()
                    .push_front(bx.chars().next().unwrap());
            }
        }
    }

    let mut stacks2 = stacks.clone();

    for (a, b, c) in &instructions {
        for _ in 0..*a {
            let moved_box = stacks.get_mut(b).unwrap().pop_back().unwrap();
            stacks.get_mut(c).unwrap().push_back(moved_box);
        }
    }

    let mut output: Vec<(&u32, &char)> =
        stacks.iter().map(|(k, v)| (k, v.back().unwrap())).collect();
    output.sort();

    println!("\tPart 1: {:?}", output);

    for (a, b, c) in &instructions {
        let mut moved_boxes = Vec::new();
        for _ in 0..*a {
            moved_boxes.push(stacks2.get_mut(b).unwrap().pop_back().unwrap());
        }
        stacks2.get_mut(c).unwrap().extend(moved_boxes.iter().rev());
    }

    let mut output: Vec<(&u32, &char)> = stacks2
        .iter()
        .map(|(k, v)| (k, v.back().unwrap()))
        .collect();
    output.sort();

    println!("\tPart 2: {:?}", output);
}
