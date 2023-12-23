fn main() {
	println!("(part 1)");
	let input: &str    = include_str!("./input1.txt");
	let output: String = part1( input);
	println!();
	dbg!(output);
}

fn part1(input: &str) ->String {
	let lines : Vec<&str> = input.lines().collect();

	let mut calories_max_count: i32 = 0;
	let mut elf_id: i32 = 0;
	let mut elf_counter: i32 = 1;

	let mut calories_count: i32 = 0;

	//get the length of lines
	let lines_length = lines.len();

    lines.iter().enumerate().for_each(|(line_index, value)| {
        println!("Index: {}, Value: {}", line_index, value);

		println!("elf_id loop entry: {}", elf_id);

		if value.is_empty() {
			if calories_count > calories_max_count {
				calories_max_count = calories_count;

				elf_id = elf_counter as i32;

				println!("calories_max_count: {}", calories_max_count);
				println!("elf_id: {}", elf_id);
			}
			calories_count = 0;
			elf_counter += 1;
		} else {
			let num: i32 = value.parse().unwrap();
				//add the number to the counter
			calories_count += num;
			//test if cal_count > cal_max_count and if so, replace it

			if line_index == lines_length - 1 {
				if calories_count > calories_max_count {
					calories_max_count = calories_count;

					elf_id = elf_counter as i32;

					println!("calories_max_count: {}", calories_max_count);
					println!("elf_id: {}", elf_id);
				}
			}
		}
    });

	if calories_count > 0 {
		elf_id = elf_counter as i32;

		println!("calories_count: {}", calories_max_count);
		println!("elf_id: {}", elf_id);		
	}

	if calories_count > calories_max_count {
		calories_max_count = calories_count;

		elf_id = elf_counter as i32;

		println!("calories_max_count: {}", calories_max_count);
		println!("elf_id: {}", elf_id);
	}
	//cmd cargo fmt => format the code

	calories_max_count.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_1() {
		let input: &str    = include_str!("./test1.txt");
		let result: String = part1(input);
		assert_eq!(result, "24000".to_string());
	}
}