use regex::Regex;

struct Card {
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
}

impl Card {
    fn new(line: &str) -> Self {
        let (_, [winning_numbers_str, my_numbers_str]) =
            Regex::new(r"^Card\s+[0-9]+:\s+(.*)\s+\|\s+(.*)$")
                .unwrap()
                .captures(line)
                .expect("Failed to parse Card")
                .extract();

        let winning_numbers: Vec<i32> = winning_numbers_str
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();

        let my_numbers: Vec<i32> = my_numbers_str
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();

        Self {
            winning_numbers,
            my_numbers,
        }
    }

    fn points(&self) -> i32 {
        let mut points: i32 = 0;

        for winning_number in self.winning_numbers.clone() {
            let have_number = self
                .my_numbers
                .clone()
                .into_iter()
                .find(|&my_number| my_number == winning_number)
                .is_some();

            if have_number {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        points
    }
}

pub mod part1 {
    use std::fs;

    use super::Card;

    fn total_points(cards: &str) -> i32 {
        let mut result: i32 = 0;
        for line in cards.lines() {
            let card = Card::new(line);
            result += card.points();
        }

        result
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day4.txt").expect("File not found");

        let result = total_points(&contents);

        println!("Day 4 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_card_points() {
            assert_eq!(
              total_points("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
                13
            );
        }
    }
}
