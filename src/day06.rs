use std::{collections::HashSet, fs};

pub fn run() {
    let content = match fs::read_to_string("src/day06.input.txt") {
        Ok(s) => s,
        Err(e) => panic!("day05.input.txt not found: {}", e),
    };

    for (i, _char) in content.chars().enumerate() {
        let lower_bound = i;
        // Part 1
        // let upper_bound = i + 4;
        // Part 2
        let upper_bound = i + 14;

        if lower_bound > content.len() - 4 {
            break;
        }

        let slice: Vec<char> = content[lower_bound..upper_bound].chars().collect();

        let mut set = HashSet::new();

        for c in &slice {
            set.insert(c);
        }

        if set.len() == slice.len() {
            println!("First at: {:?}", upper_bound);
            println!("For slice: {:?}", slice);
            break;
        }
    }
}
