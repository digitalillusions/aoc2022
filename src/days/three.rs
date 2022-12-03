use std::fs;
use std::collections::{HashSet,HashMap};
use std::iter::zip;

pub fn rucksack(){
	let priority_map = HashMap::<char,i32>::from_iter(zip(['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'],1..53));

	let rucksacks = fs::read_to_string("sample_files/03/sample.txt").unwrap().lines().map(|line| {
		let contents = line.chars().collect::<Vec<_>>();
		let (a,b) = contents.split_at(contents.len()/2);
		(HashSet::<char>::from_iter(a.iter().cloned()),HashSet::<char>::from_iter(b.iter().cloned()))
	}).collect::<Vec<_>>();

	let intersections = rucksacks.iter().map(|(a,b)| {
		let inter = a.intersection(b).collect::<HashSet<&char>>();
		inter.into_iter().next().unwrap()
	}).collect::<Vec<_>>();

	let prio_sum = intersections.iter().map(|&item| priority_map.get(item).unwrap()).sum::<i32>();
	println!("Part 1: {:?}", prio_sum);

	let badges = rucksacks.iter().map(|(a, b)| a.union(b).collect::<HashSet<_>>()).collect::<Vec<_>>().chunks(3).map(|a| {
		let b = a.get(0).unwrap().intersection(a.get(1).unwrap()).copied().collect::<HashSet<&char>>();
		b.intersection(a.get(2).unwrap()).copied().next().unwrap()
	}).collect::<Vec<_>>();

	let badges_prio_sum = badges.iter().map(|&item| priority_map.get(item).unwrap()).sum::<i32>();
	println!("Part 2: {:?}", badges_prio_sum);
}