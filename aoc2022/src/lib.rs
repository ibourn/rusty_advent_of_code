// src/lib.rs

pub fn str_to_cleaned_lines(input: &str) -> Vec<String> {
input.lines()
    .filter(|line| !line.trim().is_empty())
    .map(String::from)
    .collect()
}
