fn main() {
    println!("(part 2)");
    let input: &str = include_str!("./input2.txt");
    let output: String = part2(input);
    dbg!(output);
}


// Define an Elf struct to hold the state of each elf
#[derive(Debug)]
struct Elf {
    _id: i32,
    calories_count: i32,
}

impl Elf {
    fn new(_id: i32) -> Self {
        Elf {
            _id,
            calories_count: 0,
        }
    }

    fn update_calories(&mut self, num: i32) {
        self.calories_count += num;
    }
}

// Define an Elves struct to hold the state of the collection of elves
#[derive(Debug)]
struct Elves {
    elves: Vec<Elf>,
    total_elves: i32,
}

impl Elves {
    // Constructor to create a new Elves object
    fn new() -> Self {
        Elves {
            elves: Vec::new(),
            total_elves: 0,
        }
    }

    // Method to add a new elf to the collection
    fn add_elf(&mut self) {
        let new_elf = Elf::new(self.total_elves as i32);
        self.total_elves += 1;
        self.elves.push(new_elf);
    }
    
    // update calories of the last elf in the vector
    fn update_last_elf_calories(&mut self, num: i32) {
        if let Some(elf) = self.elves.last_mut() {
            elf.update_calories(num);
        }
    }

    fn sort_by_calories(&mut self) {
        self.elves.sort_by(|a, b| b.calories_count.cmp(&a.calories_count));
    }

    fn get_top_n_elves_by_calories(&self, n: usize) -> Vec<&Elf> {
        self.elves.iter().take(n).collect()
    }
}


fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let lines_length = lines.len();

    // Create a new Elves object and init with one empty elf
    let mut elves: Elves = Elves::new();
    elves.add_elf();

    lines.iter().enumerate().for_each(|(line_index, value)| {
        println!("Index: {}, Value: {}", line_index, value);

        if value.is_empty() {
            // print!("EMPTY LINE");
            elves.sort_by_calories();
            // println!("ordered elves: {:?}", elves);
            elves.add_elf();
            // println!("new elf, elves: {:?}", elves);
        } else {
            let num: i32 = value.parse().unwrap();
            elves.update_last_elf_calories(num);
            // println!("updated elf, elves: {:?}", elves);

            if line_index == lines_length - 1 {
                // print!("LAST LINE");
                elves.sort_by_calories();
                // println!("ordered elves: {:?}", elves);
            }
        }
    });

    // top n elves to return by calories
    let n: usize = 3;

    // get top n elves, sum their calories, and return the sum in string format
    elves.get_top_n_elves_by_calories(n)
        .iter()
        .map(|elf| elf.calories_count)
        .sum::<i32>()
        .to_string()
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_2() {
        let input: &str = include_str!("./test2.txt");
        let result: String = part2(input);
        assert_eq!(result, "45000".to_string());
    }
}
