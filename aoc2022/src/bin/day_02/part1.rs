fn main() {
	println!("(part 1)");
	let input: &str    = include_str!("./input1.txt");
	let output: String = part1( input);
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
    left: Move,  // Using the renamed Moves enum
    right: Move, // Using the renamed Moves enum
}

impl Play {
    // Method to create a new play using Move instances
    // fn new_with_moves(left: Move, right: Move) -> Self {
    //     Play { left, right }
    // }

    // Method to create a new play using characters and converting them to Move instances
    fn new_with_chars(left_char: char, right_char: char) -> Option<Self> {
        let left = Move::from_char(left_char)?;
        let right = Move::from_char(right_char)?;

        Some(Play { left, right })
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


fn part1(input: &str) ->String {
	let lines : Vec<&str> = input.lines().collect();

	// for each line, get the line and split it on " " to get 2 chars. With this 2 chars create a play
	let mut result: i32 = 0;

	for line in lines {
	    let trimmed_line = line.trim();
   		let chars: Vec<char> = trimmed_line.chars().filter(|&c| !c.is_whitespace()).collect();

		println!("chars: {:?}", chars);
        if let Some(play) = Play::new_with_chars(chars[0], chars[1]) {
            // Evaluate the right player (play and move points)
            let play_result = play.evaluate_right();
            let move_points = play.move_points_right();
            println!("play_result: {:?}", play_result);

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
	fn it_works_1() {
		let input: &str    = include_str!("./test1.txt");
		let result: String = part1(input);
		assert_eq!(result, "15".to_string());
	}
}