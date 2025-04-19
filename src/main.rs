use std::env;
mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "1" => day01::run(),
        "2" => day02::run(),
        _ => println!("Day not done yet"),
    }
}
