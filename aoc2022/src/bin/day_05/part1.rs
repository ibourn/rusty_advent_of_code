use aoc_2022::str_to_cleaned_lines;

fn main() {
    println!("part 1 :");
    let input: &str    = include_str!("./input1.txt");
    let output: String = part1(input);
    println!();
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines : Vec<String> = str_to_cleaned_lines(input);

    lines.iter().for_each(|line| {
        println!("{:?}", line);
        // TODO
    });
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn it_works_1() {
    let input: &str    = include_str!("./test1.txt");
    let result: String = part1(input);
    assert_eq!(result, "142".to_string());
    }
}

