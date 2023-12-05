use regex::Regex;

struct Card {
    id: i32,
    won_numbers: i32,
    points: i32,
}

impl Card {
    fn new(line: &str) -> Self {
        let (_, [card_id, winning_numbers_str, my_numbers_str]) =
            Regex::new(r"^Card\s+([0-9]+):\s+(.*)\s+\|\s+(.*)$")
                .unwrap()
                .captures(line)
                .expect("Failed to parse Card")
                .extract();

        let my_numbers: Vec<i32> = my_numbers_str
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();

        let won_numbers: Vec<i32> = winning_numbers_str
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .filter(|&winning_number| {
                my_numbers
                    .clone()
                    .into_iter()
                    .any(|my_number| my_number == winning_number)
            })
            .collect();

        let points = won_numbers
            .iter()
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 });

        Self {
            id: card_id.parse::<i32>().unwrap(),
            won_numbers: won_numbers.len() as i32,
            points,
        }
    }
}

struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    fn process_cards(&self) -> i32 {
        let mut result = 0;

        for card in &self.cards {
            // Add one for the cards we had in the beginning.
            result += 1 + self.process_card(card);
        }

        result
    }

    fn process_card(&self, card: &Card) -> i32 {
        let won_numbers = card.won_numbers;

        if won_numbers > 0 {
            let mut result: i32 = won_numbers;
            let start = card.id + 1;
            let end = start + won_numbers;
            for i in start..end {
                let other_card = self.cards.iter().find(|card| card.id == i).unwrap();
                result += self.process_card(other_card);
            }

            return result;
        }

        won_numbers
    }
}

pub mod part1 {
    use std::fs;

    use super::Card;

    fn total_points(cards: &str) -> i32 {
        let mut result: i32 = 0;
        for line in cards.lines() {
            let card = Card::new(line);
            result += card.points;
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

pub mod part2 {
    use std::{fs, vec};

    use super::{Card, Pile};

    fn total_cards(contents: &str) -> i32 {
        let mut cards: Vec<Card> = vec![];
        for line in contents.lines() {
            cards.push(Card::new(line));
        }

        let pile = Pile { cards };

        pile.process_cards()
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day4.txt").expect("File not found");

        let result = total_cards(&contents);

        println!("Day 4 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_total_cards() {
            assert_eq!(
              total_cards("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
                30
            );
        }
    }
}
