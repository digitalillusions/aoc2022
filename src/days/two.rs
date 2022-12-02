use std::fs;

pub fn rock_paper_scissors(){
	let rules = fs::read_to_string("sample_files/02/sample.txt").unwrap().lines().map(|item| {
		let game = item.split(" ").map(|x| x.parse::<char>().unwrap()).collect::<Vec<_>>();
		(game.get(0).unwrap().clone(), game.get(1).unwrap().clone())
	}).collect::<Vec<_>>();

	let score: Vec<i32> = rules.iter().map(|(a, b)| {
		match b{
			'X' => match a {
				'A' => 1 + 3,
				'B' => 1 + 0,
				'C' => 1 + 6,
				_ => 0,

			},
			'Y' => match a {
				'A' => 2 + 6,
				'B' => 2 + 3,
				'C' => 2 + 0,
				_ => 0,
			},
			'Z' => match a {
				'A' => 3 + 0,
				'B' => 3 + 6,
				'C' => 3 + 3,
				_ => 0,
			},
			_ => 0
		}
	}).collect();

	println!("Score of game: {:?}", score.iter().sum::<i32>());

	let score_fixed: Vec<i32> = rules.iter().map(|(a, b)| {
		match a{
			'A' => match b {
				'X' => 3 + 0,
				'Y' => 1 + 3,
				'Z' => 2 + 6,
				_ => 0,

			},
			'B' => match b {
				'X' => 1 + 0,
				'Y' => 2 + 3,
				'Z' => 3 + 6,
				_ => 0,
			},
			'C' => match b {
				'X' => 2 + 0,
				'Y' => 3 + 3,
				'Z' => 1 + 6,
				_ => 0,
			},
			_ => 0
		}
	}).collect();

	println!("Score of fixed game: {:?}", score_fixed.iter().sum::<i32>());
}