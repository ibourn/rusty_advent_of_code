use aoc_2022::str_to_cleaned_lines;

fn main() {
    println!("part 2 :");
    let input: &str    = include_str!("./input2.txt");
    let output: String = part2(input);
    println!();
    dbg!(output);
}

fn part2(input: &str) -> String {
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
fn it_works_2() {
    let input: &str    = include_str!("./test2.txt");
    let result: String = part2(input);
    assert_eq!(result, "142".to_string());
    }
}

