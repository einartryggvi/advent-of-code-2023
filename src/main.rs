mod day1;
mod day2;
mod day3;
mod day4;

use std::env;

fn main() {
    let (day, part) = parse_args();

    match (day.as_str(), part.as_str()) {
        ("1", "1") => day1::part1::run(),
        ("1", "2") => day1::part2::run(),
        ("2", "1") => day2::part1::run(),
        ("2", "2") => day2::part2::run(),
        ("3", "1") => day3::part1::run(),
        ("3", "2") => day3::part2::run(),
        ("4", "1") => day4::part1::run(),
        ("4", "2") => day4::part2::run(),
        _ => println!("Invalid day or part"),
    }
}

fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <day> <part>");
        std::process::exit(1);
    }

    let day = &args[1];
    let part = &args[2];

    return (day.to_string(), part.to_string());
}
