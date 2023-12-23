# Solutions to [Advent of Code](https://adventofcode.com/)

---

## setting up :

- **aoc_rs.sh** is a small script in bash to prepare a new Advent of Code project in rust.
  (this is my first bash script, feel free to indicate any errors or propose better solutions)

- It creates a new folder with the given name, a new rust project inside and 25 daily projects, with 2 parts each. **lib.rs** in src folder can be used for the reusable code.
  It also creates a README.md file with links to the daily projects and a .gitignore file with the necessary entries.

- **Usage :**

  `chmod +x aoc_rs.sh` to make it executable

  `./aoc_rs.sh <folder_name> <AOC_year>`

  then
  `cargo run --bin day01_part1`

  or
  `cargo test --bin day01_part1`
