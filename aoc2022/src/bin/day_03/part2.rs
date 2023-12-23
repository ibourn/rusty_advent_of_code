use std::collections::HashSet;

fn main() {
	println!("(part 2)");
	let input: &str    = include_str!("./input2.txt");
	let output: String = part2( input);
	dbg!(output);
}

fn part2(input: &str) ->String {
	let lines : Vec<&str> = input.lines().collect();

	let mut total_priority: i32 = 0;

	// group lines by 3 and trim them
	let lines = lines.chunks(3).map(|chunk| {
		chunk.iter().map(|line| line.trim()).collect::<Vec<&str>>()
	}).collect::<Vec<Vec<&str>>>();
	println!("lines: {:?}", lines);

	// for each group of 3 lines find the common chars
	lines.iter().for_each(|group| {
		// compare the first line with the second line to find the common chars then compare the result with the third line to find the common chars
		let common = group[0].chars().filter(|&c| group[1].contains(c)).collect::<String>();
		let common = common.chars().filter(|&c| group[2].contains(c)).collect::<String>();
		println!("common: {:?}", common);

		// filter common to have only unique chars
		let common = common.chars().collect::<HashSet<char>>().iter().collect::<String>();
		// convert the common chars to a number a-z -> 1-26 / A-Z -> 27-52
		let priority = common.chars().map(|c| {
			if c.is_ascii_lowercase() {
				1 + (c as u8 - b'a')
			} else {
				27 + (c as u8 - b'A')
			}
		}).collect::<Vec<u8>>();

		total_priority += priority.iter().sum::<u8>() as i32;
		println!("total_priority: {:?}", total_priority);
	});

	total_priority.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_2() {
		let input: &str    = include_str!("./test2.txt");
		let result: String = part2(input);
		assert_eq!(result, "70".to_string());
	}
}
