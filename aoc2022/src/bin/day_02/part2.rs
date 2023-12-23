fn main() {
	println!("(part 2)");
	let input: &str    = include_str!("./input2.txt");
	let output: String = part2( input);
	println!();
	dbg!(output);
}


// Enum for the moves
#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

// Enum for result
#[derive(Debug, Copy, Clone)]
enum Result {
	Z, //Win,
	Y, //Draw,
	X, //Lose,
}

impl Move {
    // Method to convert char to Move
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            'A' | 'X' => Some(Move::Rock),
            'B' | 'Y' => Some(Move::Paper),
            'C' | 'Z' => Some(Move::Scissor),
            _ => None,
        }
    }
}

// Struct representing a play
#[derive(Debug, Copy, Clone)]
struct Play {
    left: Move,  
    right: Move, 
}

// struct partial play : left play and result
#[derive(Debug, Copy, Clone)]
struct PartialPlay {
	left: Move, 
	result: Result, 
}

// impl Play {
//     // Method to create a new play using Move instances
//     // fn new_with_moves(left: Move, right: Move) -> Self {
//     //     Play { left, right }
//     // }

//     // Method to create a new play using characters and converting them to Move instances
//     fn new_with_chars(left_char: char, right_char: char) -> Option<Self> {
//         let left = Move::from_char(left_char)?;
//         let right = Move::from_char(right_char)?;

//         Some(Play { left, right })
//     }
// }

impl PartialPlay {
	fn new_with_chars(left_char: char, result_char: char) -> Option<Self> {
		let left = Move::from_char(left_char)?;
		let result = match result_char {
			'X' => Result::X,
			'Y' => Result::Y,
			'Z' => Result::Z,
			_ => return None,
		};

		Some(PartialPlay { left, result })
	}
}

// Trait for evaluating a play
trait EvaluatePlay {
    fn evaluate_right(&self) -> i32;
	fn move_points_right(&self) -> i32;
}

// Implementation of the trait for the Play struct
impl EvaluatePlay for Play {
    fn evaluate_right(&self) -> i32 {
        match (self.left, self.right) {
            (Move::Rock, Move::Scissor) | (Move::Paper, Move::Rock) | (Move::Scissor, Move::Paper) => 0,
            (Move::Rock, Move::Paper) | (Move::Paper, Move::Scissor) | (Move::Scissor, Move::Rock) => 6,
            _ => 3,
        }
    }
	fn move_points_right(&self) -> i32 {
		match self.right {
			Move::Rock => 1,
			Move::Paper => 2,
			Move::Scissor => 3,
		}
	}
}

// function to give the right play to fit the result of partial play
fn get_right_play(partial_play: PartialPlay) -> Play {
	match partial_play {
		PartialPlay {left: Move::Rock, result: Result::X} => Play {left: Move::Rock, right: Move::Scissor},
		PartialPlay {left: Move::Rock, result: Result::Y} => Play {left: Move::Rock, right: Move::Rock},
		PartialPlay {left: Move::Rock, result: Result::Z} => Play {left: Move::Rock, right: Move::Paper},
		PartialPlay {left: Move::Paper, result: Result::X} => Play {left: Move::Paper, right: Move::Rock},
		PartialPlay {left: Move::Paper, result: Result::Y} => Play {left: Move::Paper, right: Move::Paper},
		PartialPlay {left: Move::Paper, result: Result::Z} => Play {left: Move::Paper, right: Move::Scissor},
		PartialPlay {left: Move::Scissor, result: Result::X} => Play {left: Move::Scissor, right: Move::Paper},
		PartialPlay {left: Move::Scissor, result: Result::Y} => Play {left: Move::Scissor, right: Move::Scissor},
		PartialPlay {left: Move::Scissor, result: Result::Z} => Play {left: Move::Scissor, right: Move::Rock},
	}
}

impl EvaluatePlay for PartialPlay {
	fn evaluate_right(&self) -> i32 {
		match (self.left, self.result) {
			(Move::Rock, Result::X) | (Move::Paper, Result::X) | (Move::Scissor, Result::X) => 0,
			(Move::Rock, Result::Y) | (Move::Paper, Result::Y) | (Move::Scissor, Result::Y) => 6,
			_ => 3,
		}
	}
	fn move_points_right(&self) -> i32 {
		match self.left {
			Move::Rock => 1,
			Move::Paper => 2,
			Move::Scissor => 3,
		}
	}
}


fn part2(input: &str) ->String {
	let lines : Vec<&str> = input.lines().collect();

	// for each line, get the line and split it on " " to get 2 chars
	// then with this 2 chars create a play

	let mut result: i32 = 0;

	for line in lines {
		let trimmed_line = line.trim();
		let chars: Vec<char> = trimmed_line.chars().filter(|&c| !c.is_whitespace()).collect();

		println!("chars: {:?}", chars);
		if let Some(partial_play) = PartialPlay::new_with_chars(chars[0], chars[1]) {
			println!("partial_play: {:?}", partial_play);
			let right_play = get_right_play(partial_play);
			println!("right_play: {:?}", right_play);
			let play = Play {left: partial_play.left, right: right_play.right};
			println!("play: {:?}", play);
			let play_result = play.evaluate_right();
			let move_points = play.move_points_right();
			println!("play_result: {:?}", play_result);
			println!("move_points: {:?}", move_points);

			// Add these points to result
			result += play_result + move_points;
		} else {
			println!("Invalid line: {}", line);
		}
	}

	//convert result to string
	result.to_string()


}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_2() {
		let input: &str    = include_str!("./test2.txt");
		let result: String = part2(input);
		assert_eq!(result, "12".to_string());
	}
}