#!/bin/bash

# Author: ibourn

# This script creates a new Advent of Code project in Rust.
# It creates a new folder with the given name, a new Rust project inside and 25 daily projects, with 2 parts each.
# It also creates a README.md file with links to the daily projects and a .gitignore file with the necessary entries.

# Usage: ./aoc_rs.sh <folder_name> <AOC_year>
# chmod +x aoc_rs.sh to make it executable

# then run 
# cargo run --bin day01_part1
# or
# cargo test --bin day01_part1

# Function to check if a command is available
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if cargo is installed
if ! command_exists cargo; then
    echo "Error: cargo is not installed."
    echo "To install Rust and cargo, visit: https://www.rust-lang.org/learn/get-started"
    exit 1
fi

# Check if other necessary tools are installed (add more tools if needed)
if ! command_exists rustc || ! command_exists rustup; then
    echo "Error: Rust tools are not installed."
    echo "To install Rust and cargo, visit: https://www.rust-lang.org/learn/get-started"
    exit 1
fi

# Function to check if the project name is in snake_case
is_snake_case() {
    local name="$1"
    if [[ ! "$name" =~ ^[a-z0-9_]+$ ]]; then
        echo "Error: Project name should be in snake_case."
        echo "Example: my_project_name"
        exit 1
    fi
}

folder_name=$1
AOC_year=$2
# check folder_name is in snake_case
is_snake_case "$folder_name"

# check folder_name is not empty
if [ "$#" -lt 2 ]; then
    echo "Usage: $0 <folder_name> <AOC_year>"
    exit 1
fi

# check AOC_year is a number between 2015 and current year
if ! [[ "$AOC_year" =~ ^[0-9]+$ ]] ; then
    echo "error: AOC_year is not a number" >&2; exit 1
fi

if [ "$AOC_year" -lt 2015 ] || [ "$AOC_year" -gt $(date +"%Y") ]; then
    echo "error: AOC_year is not between 2015 and current year" >&2; exit 1
fi

echo "creating new project with cargo new --lib"
cargo new --lib $folder_name
cd "$folder_name"
# git init not needed, cargo new does it ??

for i in {1..25}
do
	# day folder 
	day_folder=$(printf "day_%02d" "$i")
    mkdir -p "src/bin/$day_folder"

    # Create input and test files
    touch "src/bin/$day_folder/input1.txt"
    touch "src/bin/$day_folder/input2.txt"
    touch "src/bin/$day_folder/test1.txt"
    touch "src/bin/$day_folder/test2.txt"

    # Create daily README.md
	echo "[Day $i - Advent of Code $AOC_year](https://adventofcode.com/$AOC_year/day/$i)

**--- Day $i : XXXXXX ---**

**--- part ONE : ---**

**--- part TWO : ---**
" > "src/bin/$day_folder/README.md"

    for j in {1..2}
    do
        # Create part1.rs and part2.rs
        echo "use $folder_name::str_to_cleaned_lines;

fn main() {
    println!(\"part $j :\");
    let input: &str    = include_str!(\"./input$j.txt\");
    let output: String = part$j(input);
    println!();
    dbg!(output);
}

fn part$j(input: &str) -> String {
    let lines : Vec<String> = str_to_cleaned_lines(input);

    lines.iter().for_each(|line| {
        println!(\"{:?}\", line);
        // TODO
    });
    \"todo!()\".to_string()
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn it_works_$j() {
    let input: &str    = include_str!(\"./test$j.txt\");
    let result: String = part$j(input);
    assert_eq!(result, \"142\".to_string());
    }
}
" > "src/bin/$day_folder/part$j.rs"

        # convert i into a string with 2 digits padding with 0
        conv_i=$(printf "%02d" "$i")
        # add bin to Cargo.toml
        echo "[[bin]]
name = \"day${conv_i}_part${j}\"
path = \"src/bin/day_${conv_i}/part${j}.rs\"
" >> Cargo.toml

    done
done

# Create README.md 
echo "# [Advent of Code $AOC_year](https://adventofcode.com/$AOC_year)

" > "README.md"

# links to daily README.md
for i in {1..25}
do
    # convert i into a string with 2 digits padding with 0
    conv_i=$(printf "%02d" "$i")

    # calculate column index (1 or 2)
    col=$((i % 2 + 1))

    # append the line to the appropriate column
    if [ $col -eq 1 ]; then
        echo "  ||   Day $conv_i - [part 1](./src/bin/day_${conv_i}/part1.rs) - [part 2](./src/bin/day_${conv_i}/part2.rs)    " >> "README.md"
    else
        echo -n "Day $conv_i - [part 1](./src/bin/day_${conv_i}/part1.rs) - [part 2](./src/bin/day_${conv_i}/part2.rs)" >> "README.md"
    fi
done

# add cmd to run solutions and tests (pass a line before)
echo -e "\n\n" >> "README.md"
echo "### to run a project : " >> "README.md"
echo '```bash' >> README.md
echo "cargo [run | test] --bin dayXX_partY" >> "README.md"
echo '```' >> README.md

# Create .gitignore
echo "# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
# Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb" > ".gitignore"

# create fn in lib.rs
echo "// src/lib.rs

pub fn str_to_cleaned_lines(input: &str) -> Vec<String> {
input.lines()
    .filter(|line| !line.trim().is_empty())
    .map(String::from)
    .collect()
}" > "src/lib.rs"

# display actions done with this script
echo "AOC folder initialized with 25 daily projects !"
# cmd to run
echo "to run a project : cargo [run | test] --bin dayXX_partY"