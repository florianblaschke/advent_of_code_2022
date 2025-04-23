use std::{fs, time::Instant};

/*
A | X is Rock | 1Point
B | Y is Paper | 2Points
C | Z is Scissors | 3Points

Loss => 0
Draw => 3
Win => 6

(A | X) > (C | Z)
(B | Y) > (A | X)
(C | Z) > (B | Y)
*/

#[derive(Debug)]
struct Game {
    theirs: String,
    ours: String,
}

fn parse_line(game: &str) -> Option<Game> {
    let mut letters = game.split_whitespace();

    let theirs = letters.next()?;
    let ours = letters.next()?;

    Some(Game {
        theirs: theirs.to_string(),
        ours: ours.to_string(),
    })
}

fn string_to_game(s: &str) -> Vec<Game> {
    s.lines().filter_map(|line| parse_line(line)).collect()
}

fn count_score(games: &Vec<Game>) -> u32 {
    let mut score = 0;

    for game in games {
        let theirs = map_move(&game.theirs);
        let ours = map_move(&game.ours);

        if ours == theirs {
            score += ours + 3
        }

        if ours == 1 && theirs == 3 || ours == 2 && theirs == 1 || ours == 3 && theirs == 2 {
            score += ours + 6
        }

        if ours == 1 && theirs == 2 || ours == 2 && theirs == 3 || ours == 3 && theirs == 1 {
            score += ours
        }
    }

    score
}

pub fn run() {
    let content = match fs::read_to_string("src/day02.input.txt") {
        Ok(s) => s,
        Err(e) => panic!("Could not find src/day02.input.txt: {e}"),
    };

    let games = string_to_game(&content);
    // Part 1
    let start_one = Instant::now();
    let score = count_score(&games);
    let duration_part_one = start_one.elapsed();

    // Part 2
    let start_two = Instant::now();
    let rigged_games = games
        .iter()
        .map(|game| map_result_to_move(&game.theirs, &game.ours))
        .collect();

    let rigged_score = count_score(&rigged_games);
    let duration_part_two = start_two.elapsed();

    println!("Final score is: {}", score);
    println!("Took: {:?}", duration_part_one);

    println!("Rigged score is: {}", rigged_score);
    println!("Took: {:?}", duration_part_two);
}

fn map_move(code: &str) -> u32 {
    match code {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("No move found"),
    }
}

enum Result {
    Win,
    Lose,
    Draw,
}

struct Sign {
    lose: String,
    draw: String,
    win: String,
}

fn map_result_to_move(theirs: &str, aimed_res: &str) -> Game {
    let rock = Sign {
        lose: String::from("C"),
        draw: String::from("A"),
        win: String::from("B"),
    };

    let paper = Sign {
        lose: String::from("A"),
        draw: String::from("B"),
        win: String::from("C"),
    };

    let scissors = Sign {
        lose: String::from("B"),
        draw: String::from("C"),
        win: String::from("A"),
    };

    let sign = match theirs {
        "A" => &rock,
        "B" => &paper,
        "C" => &scissors,
        _ => panic!("No valid sign found: {theirs}"),
    };

    let result = aimed_result_to_move(aimed_res);

    let ours = match result {
        Result::Win => sign.win.to_string(),
        Result::Draw => sign.draw.to_string(),
        Result::Lose => sign.lose.to_string(),
    };

    Game {
        theirs: theirs.to_string(),
        ours,
    }
}

fn aimed_result_to_move(res: &str) -> Result {
    match res {
        "X" => Result::Lose,
        "Y" => Result::Draw,
        "Z" => Result::Win,
        _ => panic!("Invalid sign: {res}"),
    }
}
