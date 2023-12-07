mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        ("5", "1") => day5::part1::run(),
        ("5", "2") => day5::part2::run(),
        ("6", "1") => day6::part1::run(),
        ("6", "2") => day6::part2::run(),
        ("7", "1") => day7::part1::run(),
        ("7", "2") => day7::part2::run(),
        ("8", "1") => day8::part1::run(),
        ("8", "2") => day8::part2::run(),
        ("9", "1") => day9::part1::run(),
        ("9", "2") => day9::part2::run(),
        ("10", "1") => day10::part1::run(),
        ("10", "2") => day10::part2::run(),
        ("11", "1") => day11::part1::run(),
        ("11", "2") => day11::part2::run(),
        ("12", "1") => day12::part1::run(),
        ("12", "2") => day12::part2::run(),
        ("13", "1") => day13::part1::run(),
        ("13", "2") => day13::part2::run(),
        ("14", "1") => day14::part1::run(),
        ("14", "2") => day14::part2::run(),
        ("15", "1") => day15::part1::run(),
        ("15", "2") => day15::part2::run(),
        ("16", "1") => day16::part1::run(),
        ("16", "2") => day16::part2::run(),
        ("17", "1") => day17::part1::run(),
        ("17", "2") => day17::part2::run(),
        ("18", "1") => day18::part1::run(),
        ("18", "2") => day18::part2::run(),
        ("19", "1") => day19::part1::run(),
        ("19", "2") => day19::part2::run(),
        ("20", "1") => day20::part1::run(),
        ("20", "2") => day20::part2::run(),
        ("21", "1") => day21::part1::run(),
        ("21", "2") => day21::part2::run(),
        ("22", "1") => day22::part1::run(),
        ("22", "2") => day22::part2::run(),
        ("23", "1") => day23::part1::run(),
        ("23", "2") => day23::part2::run(),
        ("24", "1") => day24::part1::run(),
        ("24", "2") => day24::part2::run(),
        ("25", "1") => day25::part1::run(),
        ("25", "2") => day25::part2::run(),
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

    (day.to_string(), part.to_string())
}
