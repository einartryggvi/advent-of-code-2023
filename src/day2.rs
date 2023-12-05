use regex::Regex;
use std::fs;

const MAX_REDS: i32 = 12;
const MAX_GREENS: i32 = 13;
const MAX_BLUES: i32 = 14;

struct Game {
    id: i32,
    reveals: Vec<Reveal>,
}

impl Game {
    fn is_legit(&self) -> bool {
        for reveal in self.reveals.iter() {
            if !reveal.is_legit() {
                return false;
            }
        }

        true
    }

    fn max_red(&self) -> i32 {
        let mut max_red: i32 = 1;

        for reveal in self.reveals.iter() {
            if reveal.red > max_red {
                max_red = reveal.red;
            }
        }

        max_red
    }

    fn max_green(&self) -> i32 {
        let mut max_green: i32 = 1;

        for reveal in self.reveals.iter() {
            if reveal.green > max_green {
                max_green = reveal.green;
            }
        }

        max_green
    }

    fn max_blue(&self) -> i32 {
        let mut max_blue: i32 = 1;

        for reveal in self.reveals.iter() {
            if reveal.blue > max_blue {
                max_blue = reveal.blue;
            }
        }

        max_blue
    }

    fn power(&self) -> i32 {
        self.max_red() * self.max_green() * self.max_blue()
    }
}

struct Reveal {
    red: i32,
    green: i32,
    blue: i32,
}

impl Reveal {
    fn is_legit(&self) -> bool {
        self.red <= MAX_REDS && self.green <= MAX_GREENS && self.blue <= MAX_BLUES
    }
}

fn parse_color(regex: &str, reveal_str: &str) -> i32 {
    let re = Regex::new(regex).expect("failed to create color regex");

    match re.captures(reveal_str) {
        Some(captures) => captures
            .get(1)
            .expect("Failed to get capture")
            .as_str()
            .parse::<i32>()
            .unwrap(),
        None => 0,
    }
}

fn create_reveal(reveal_str: &str) -> Reveal {
    Reveal {
        red: parse_color(r"([0-9]+) red", reveal_str),
        green: parse_color(r"([0-9]+) green", reveal_str),
        blue: parse_color(r"([0-9]+) blue", reveal_str),
    }
}

pub mod part1 {
    use super::*;

    pub fn run() {
        let contents = fs::read_to_string("inputs/day2.txt").expect("File not found");

        let mut result: i32 = 0;
        let re = Regex::new(r"^Game ([0-9]+): (.*)$").unwrap();
        for line in contents.split('\n') {
            let (_, [game_id, reveals_str]) = re
                .captures(line)
                .expect("Failed to parse game ID")
                .extract();

            let mut reveals = Vec::new();

            for reveal in reveals_str.split(';') {
                reveals.push(create_reveal(reveal))
            }

            let game = Game {
                id: game_id.to_string().parse::<i32>().unwrap(),
                reveals,
            };

            if game.is_legit() {
                result += game.id;
            }
        }

        println!("Day 2 Part 1: {}", result)
    }
}

pub mod part2 {
    use super::*;

    pub fn run() {
        let contents = fs::read_to_string("inputs/day2.txt").expect("File not found");

        let mut result: i32 = 0;
        let re = Regex::new(r"^Game ([0-9]+): (.*)$").unwrap();
        for line in contents.split('\n') {
            let (_, [game_id, reveals_str]) = re
                .captures(line)
                .expect("Failed to parse game ID")
                .extract();

            let mut reveals = Vec::new();

            for reveal in reveals_str.split(';') {
                reveals.push(create_reveal(reveal))
            }

            let game = Game {
                id: game_id.to_string().parse::<i32>().unwrap(),
                reveals,
            };

            result += game.power();
        }

        println!("Day 2 Part 2: {}", result)
    }
}
