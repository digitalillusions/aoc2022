use std::fs;

pub fn calorie_counting() {
    let input = fs::read_to_string("sample_files/01/sample.txt").unwrap();
    let mut calories_per_elf = Vec::new();
    calories_per_elf.push(Vec::new());
    for line in input.lines() {
        if line.trim().is_empty() {
            calories_per_elf.push(Vec::new());
        } else {
            calories_per_elf
                .last_mut()
                .unwrap()
                .push(line.parse::<usize>().unwrap());
        }
    }
    let mut total_calories_per_elf = calories_per_elf
        .iter()
        .map(|vec| vec.iter().sum::<usize>())
        .collect::<Vec<_>>();
    total_calories_per_elf.sort();
    println!(
        "Part 1: Largest number of calories carried by a single elf {}",
        total_calories_per_elf.last().unwrap()
    );

    println!(
        "Part 1: Number of calories carried by the elves with the 3 largest amounts of calories {}",
        total_calories_per_elf.iter().rev().take(3).sum::<usize>()
    );
}
