use std::env;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "1" => day01::run()?,
        "2" => day02::run(),
        "3" => day03::run(),
        "4" => day04::run(),
        "5" => day05::run(),
        "6" => day06::run(),
        _ => println!("Day not done yet"),
    }

    Ok(())
}
