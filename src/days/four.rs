use std::fs;

pub fn camp_cleanup() {
    println!("Running day 4");

    let contents = fs::read_to_string("sample_files/04/sample.txt").unwrap();
    let ranges = contents
        .lines()
        .map(|item| {
            let mut elves = item.split(',');
            let (elf1, elf2) = (elves.next().unwrap(), elves.next().unwrap());
            let mut elf1_range = elf1.split('-').map(|item| item.parse::<i32>().unwrap());
            let mut elf2_range = elf2.split('-').map(|item| item.parse::<i32>().unwrap());
            (
                (elf1_range.next().unwrap(), elf1_range.next().unwrap()),
                (elf2_range.next().unwrap(), elf2_range.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let fully_intersecting_ranges = ranges
        .iter()
        .filter(|(a, b)| ((b.0 <= a.0) && (a.1 <= b.1)) || ((a.0 <= b.0) && (b.1 <= a.1)))
        .collect::<Vec<_>>();

    println!("\tPart 1: {:?}", fully_intersecting_ranges.len());

    let partially_overlapping_ranges = ranges
        .iter()
        .filter(|(a, b)| ((a.0 <= b.0) && (b.0 <= a.1)) || ((b.0 <= a.0) && (a.0 <= b.1)))
        .collect::<Vec<_>>();

    println!("\tPart 2: {:?}", partially_overlapping_ranges.len());
}
