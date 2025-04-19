use std::{collections::HashSet, fs};

pub fn run() {
    let content = match fs::read_to_string("src/day03.input.txt") {
        Ok(s) => s,
        Err(e) => panic!("Could not read day03.input.txt: {}", e),
    };

    let rucksacks: Vec<&str> = content.lines().collect();

    // Part 1
    let mut priorities = 0;
    for rucksack in &rucksacks {
        let duplicate = check_chars(rucksack).unwrap();
        let value = char_to_points(duplicate);
        priorities += value;
    }

    println!("Priorities are: {}", priorities);

    // Part 2
    let groups = rucksacks.chunks(3);
    let mut group_priorites = 0;

    for group in groups {
        let mut first_member = HashSet::new();
        let mut second_member = HashSet::new();
        let mut third_member = HashSet::new();

        first_member.extend(group[0].chars());
        second_member.extend(group[1].chars());
        third_member.extend(group[2].chars());

        let values = first_member.intersection(&second_member);

        for val in values {
            if third_member.contains(val) {
                group_priorites += char_to_points(*val);
            }
        }
    }

    println!("The group priorities are: {}", group_priorites);
}

fn check_chars(s: &str) -> Option<char> {
    let (first, second) = s.split_at(s.len() / 2);

    let mut first_compartment = HashSet::new();
    let mut second_compartment = HashSet::new();

    first_compartment.extend(first.chars());
    second_compartment.extend(second.chars());

    first_compartment
        .intersection(&second_compartment)
        .next()
        .copied()
}

fn char_to_points(char: char) -> u32 {
    match char {
        'a'..='z' => (char as u32) - ('a' as u32) + 1,
        'A'..='Z' => (char as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid char: {}", char),
    }
}
