use std::{fs, str};

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

pub fn run() {
    let content = match fs::read_to_string("src/day05.input.txt") {
        Ok(s) => s,
        Err(e) => panic!("day05.input.txt not found: {}", e),
    };

    let (crates_string, instructions_string) = content.split_once("\n\n").unwrap();

    let instructions: Vec<Instruction> = instructions_string
        .lines()
        .map(|line| {
            let frags: Vec<&str> = line.split(" ").collect();

            Instruction {
                quantity: frags[1].parse().unwrap(),
                from: frags[3].parse().unwrap(),
                to: frags[5].parse().unwrap(),
            }
        })
        .collect();

    let mut crates: Vec<Vec<char>> = vec![Vec::new(); 9];

    for line in crates_string.lines() {
        if line.trim().starts_with('1') {
            break;
        }

        for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
            let slot = str::from_utf8(chunk).unwrap();

            if let Some(char) = slot.chars().nth(1) {
                if char != ' ' {
                    crates[i].insert(0, char);
                }
            }
        }
    }

    for instruction in &instructions {
        // crate_mover_9000(instruction, &mut crates);
        crate_mover_9001(instruction, &mut crates)
    }

    let solution = get_letters(crates);

    println!("Crates: {:?}", solution)
}

fn _crate_mover_9000(instruction: &Instruction, crate_array: &mut Vec<Vec<char>>) {
    for _i in 0..instruction.quantity {
        let cr = crate_array[instruction.from - 1].pop().unwrap_or('😡');

        if cr != '😡' {
            crate_array[instruction.to - 1].push(cr);
        }
    }
}

fn crate_mover_9001(instruction: &Instruction, crates: &mut Vec<Vec<char>>) {
    let length = crates[instruction.from - 1].len();
    let slice = crates[instruction.from - 1].split_off(length - instruction.quantity);

    crates[instruction.to - 1].extend(slice);
}

fn get_letters(crates: Vec<Vec<char>>) -> String {
    let mut solution = String::new();

    for arr in crates {
        if let Some(&last) = arr.last() {
            solution.push(last)
        }
    }

    solution
}
