use std::{collections::BTreeSet, fs};

pub fn rope_bridge() {
    println!("Running day 9");
    let content = fs::read_to_string("sample_files/09/sample.txt").unwrap();
    let instructions = content
        .lines()
        .map(|line| {
            let (dir, count) = line.split_once(" ").unwrap();
            (dir, count.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut rope_elements: Vec<(i32, i32)> = Vec::new();
    rope_elements.resize(10, (0, 0));

    let mut all_rope_element_locs = Vec::new();
    all_rope_element_locs.resize(10, BTreeSet::from([(0, 0)]));

    for (dir, count) in instructions.iter() {
        for _ in 0..*count {
            let head_loc = rope_elements.get_mut(0).unwrap();
            match *dir {
                "U" => head_loc.1 += 1,
                "D" => head_loc.1 -= 1,
                "R" => head_loc.0 += 1,
                "L" => head_loc.0 -= 1,
                _ => println!("Invalid command given"),
            };

            for i in 0..rope_elements.len() - 1 {
                let head = rope_elements.get(i).unwrap().clone();
                let tail = rope_elements.get_mut(i + 1).unwrap();

                let distance = (head.0 - tail.0, head.1 - tail.1);
                let abs_distance = (distance.0.abs().clamp(0, 2), distance.1.abs().clamp(0, 2));

                let tail_loc_update = match abs_distance {
                    (2, 2) => (distance.0 / abs_distance.0, distance.1 / abs_distance.1),
                    (1, 2) => (distance.0, distance.1 / abs_distance.1),
                    (2, 1) => (distance.0 / abs_distance.0, distance.1),
                    (0, 2) => (0, distance.1 / abs_distance.1),
                    (2, 0) => (distance.0 / abs_distance.0, 0),
                    _ => (0, 0),
                };

                *tail = (tail.0 + tail_loc_update.0, tail.1 + tail_loc_update.1);
                all_rope_element_locs.get_mut(i + 1).unwrap().insert(*tail);
            }
        }
    }

    println!(
        "\tPart 1: {:?}",
        all_rope_element_locs.get(1).unwrap().len()
    );
    println!(
        "\tPart 2: {:?}",
        all_rope_element_locs.last().unwrap().len()
    );
}
