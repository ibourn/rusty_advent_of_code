use std::collections::HashSet;


fn main() {
	println!("(part 1)");
	let input: &str    = include_str!("./input1.txt");
	let output: String = part1( input);
	println!();
	dbg!(output);
}

fn part1(input: &str) ->String {
	let lines : Vec<&str> = input.lines().collect();

	let mut total_priority: i32 = 0;

	lines.iter().filter(|&&line| !line.trim().is_empty()).for_each(|line| {
		//trim the line
		let line = line.trim();
		//split the line in two parts (the middle of the length line)
		let (left, right) = line.split_at(line.len() / 2);
		println!("left: {:?} - right: {:?}", left, right);

		//which char present in the first part is also somewhere in the second part
		let common = left.chars().filter(|&c| right.contains(c)).collect::<String>();
		println!("common: {:?}", common);
		//filter common to have only unique chars
		let common = common.chars().collect::<HashSet<char>>().iter().collect::<String>();
		println!("common: {:?}", common);


		//convert the common chars to a number a-z -> 1-26 / A-Z -> 27-52
		let priority = common.chars().map(|c| {
			if c.is_ascii_lowercase() {
				//b'a' is the ascii value of 'a' (97)
				1 + (c as u8 - b'a')
			} else {
				27 + (c as u8 - b'A')
			}
		}).collect::<Vec<u8>>();

		total_priority += priority.iter().sum::<u8>() as i32;

		// TODO
	});

	total_priority.to_string()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_1() {
		let input: &str    = include_str!("./test1.txt");
		let result: String = part1(input);
		assert_eq!(result, "157".to_string());
	}
}


// cmd
//cargo new DAYX
//paste src
//cargo run --bin part1 ou cargo run
//cargo test