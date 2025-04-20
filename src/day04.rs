use std::fs;

pub fn run() {
    let content = match fs::read_to_string("src/day04.input.txt") {
        Ok(s) => s,
        Err(e) => panic!("File day04.input.txt not found: {}", e),
    };

    let teams = to_sections(&content);
    let mut counter = 0;
    let mut counter_part_two = 0;

    for team in &teams {
        if team.a.0 <= team.b.0 && team.a.1 >= team.b.1
            || team.b.0 <= team.a.0 && team.b.1 >= team.a.1
        {
            counter += 1;
        }

        if team.a.0 <= team.b.1 && team.b.0 <= team.a.1 {
            counter_part_two += 1;
        }
    }

    println!("Overlapping: {}", counter);
    println!("Overlapping Part 2: {}", counter_part_two)
}

struct Range(i32, i32);

struct Team {
    a: Range,
    b: Range,
}

fn to_sections(content: &str) -> Vec<Team> {
    content
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(",").unwrap();

            let (fl, fh) = first.split_once("-").unwrap();
            let (sl, sh) = second.split_once("-").unwrap();

            Team {
                a: Range(fl.parse().unwrap(), fh.parse().unwrap()),
                b: Range(sl.parse().unwrap(), sh.parse().unwrap()),
            }
        })
        .collect()
}
