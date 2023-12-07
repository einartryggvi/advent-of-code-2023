use std::{cmp::Ordering, collections::HashMap, u64};

fn calculate_card_rank(value: &char) -> u64 {
    match value {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            value.to_string().parse::<u64>().unwrap()
        }
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn calculate_hand_rank(cards: &Vec<Card>) -> u64 {
    let mut counts: HashMap<u64, u64> = HashMap::new();

    let mut cards_sorted = (*cards).clone();
    cards_sorted.sort_by_key(|card| card.rank);

    for card in cards_sorted {
        counts.insert(card.rank, counts.get(&card.rank).unwrap_or(&0) + 1);
    }

    let mut counts_vec: Vec<u64> = counts.iter().map(|(_, &count)| count).collect();

    counts_vec.sort();

    let real_counts: &Vec<&u64> = &counts_vec.iter().rev().collect();

    // Five of a kind, where all five cards have the same label: AAAAA
    if *real_counts[0] == 5 {
        return 7;
    }

    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    if *real_counts[0] == 4 {
        return 6;
    }

    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    if *real_counts[0] == 3 && *real_counts[1] == 2 {
        return 5;
    }

    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    if *real_counts[0] == 3 {
        return 4;
    }

    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    if *real_counts[0] == 2 && *real_counts[1] == 2 {
        return 3;
    }

    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    if *real_counts[0] == 2 {
        return 2;
    }

    // High card, where all cards' labels are distinct: 23456
    1
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    if a.rank > b.rank {
        return Ordering::Greater;
    } else if a.rank < b.rank {
        return Ordering::Less;
    }

    for (a, b) in a.cards.iter().zip(b.cards.iter()) {
        if a.rank > b.rank {
            return Ordering::Greater;
        } else if a.rank < b.rank {
            return Ordering::Less;
        }
    }

    Ordering::Less
}

#[derive(Clone, Debug)]
struct Card {
    rank: u64,
}

impl Card {
    fn parse(value: &str) -> Self {
        let chars: Vec<char> = value.chars().collect();
        let val = chars[0];
        Self {
            rank: calculate_card_rank(&val),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    rank: u64,
    bid: u64,
}

impl Hand {
    fn parse(string: &str) -> Self {
        let parts: Vec<&str> = string.split_whitespace().collect();
        let cards: Vec<Card> = parts[0]
            .split("")
            .filter(|s| s.len() > 0)
            .map(|s| Card::parse(s))
            .collect();

        let bid = parts[1].parse::<u64>().unwrap();
        let rank = calculate_hand_rank(&cards);

        Self { cards, rank, bid }
    }
}

pub mod part1 {
    use super::*;
    use std::fs;

    fn total_winnings(contents: &str) -> u64 {
        let mut hands: Vec<Hand> = vec![];

        for line in contents.lines() {
            let hand = Hand::parse(line);
            hands.push(hand);
        }

        let mut winnings: Vec<u64> = vec![];

        hands.sort_by(|a, b| compare_hands(a, b));

        for (i, hand) in hands.iter().enumerate() {
            winnings.push((i as u64 + 1) * hand.bid);
        }

        winnings.iter().sum()
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day7.txt").expect("File not found");

        let result = total_winnings(&contents);

        println!("Day 7 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_total_winnings() {
            assert_eq!(
                total_winnings("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483"),
                6440
            );
        }

        #[test]
        fn test_total_winnings2() {
            assert_eq!(total_winnings("73642 1\n27438 2"), 4);
        }

        // Five of a kind, where all five cards have the same label: AAAAA
        #[test]
        fn test_hand_rank1() {
            assert_eq!(calculate_hand_rank(&Hand::parse("AAAAA 1").cards), 7);
        }

        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        #[test]
        fn test_hand_rank2() {
            assert_eq!(calculate_hand_rank(&Hand::parse("AA8AA 1").cards), 6);
        }

        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        #[test]
        fn test_hand_rank3() {
            assert_eq!(calculate_hand_rank(&Hand::parse("23332 1").cards), 5);
        }

        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        #[test]
        fn test_hand_rank4() {
            assert_eq!(calculate_hand_rank(&Hand::parse("TTT98 1").cards), 4);
        }

        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        #[test]
        fn test_hand_rank5() {
            assert_eq!(calculate_hand_rank(&Hand::parse("23432 1").cards), 3);
        }

        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        #[test]
        fn test_hand_rank6() {
            assert_eq!(calculate_hand_rank(&Hand::parse("A23A4 1").cards), 2);
        }

        // High card, where all cards' labels are distinct: 23456
        #[test]
        fn test_hand_rank7() {
            assert_eq!(calculate_hand_rank(&Hand::parse("23456 1").cards), 1);
        }
    }
}
