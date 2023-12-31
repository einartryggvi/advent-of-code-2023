use std::{cmp::Ordering, collections::HashMap, u64};

fn calculate_card_rank(value: &char, wildcard_value: u64) -> u64 {
    match value {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            value.to_string().parse::<u64>().unwrap()
        }
        'T' => 10,
        'J' => wildcard_value,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn calculate_hand_rank_wildcard(cards: &Vec<Card>, wildcard: bool) -> u64 {
    if !wildcard {
        return calculate_hand_rank(cards);
    }

    let mut ranks: Vec<u64> = vec![];

    for rank in 1..15 {
        let new_cards: Vec<Card> = cards
            .clone()
            .iter()
            .map(|c: &Card| {
                if c.rank == 1 {
                    Card { rank }
                } else {
                    c.clone()
                }
            })
            .collect();

        ranks.push(calculate_hand_rank(&new_cards));
    }

    *ranks.iter().max().unwrap_or(&0)
}

fn calculate_hand_rank(cards: &Vec<Card>) -> u64 {
    let mut cards_sorted = (*cards).clone();
    cards_sorted.sort();

    let counts = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card.rank).or_insert(0) += 1;
        acc
    });

    let mut real_counts: Vec<u64> = counts.values().cloned().collect();
    real_counts.sort_by(|a, b| b.cmp(a));

    // Five of a kind, where all five cards have the same label: AAAAA
    if real_counts[0] == 5 {
        return 7;
    }

    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    if real_counts[0] == 4 {
        return 6;
    }

    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    if real_counts[0] == 3 && real_counts[1] == 2 {
        return 5;
    }

    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    if real_counts[0] == 3 {
        return 4;
    }

    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    if real_counts[0] == 2 && real_counts[1] == 2 {
        return 3;
    }

    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    if real_counts[0] == 2 {
        return 2;
    }

    // High card, where all cards' labels are distinct: 23456
    1
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    match a.rank.cmp(&b.rank) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            for (a, b) in a.cards.iter().zip(b.cards.iter()) {
                match a.rank.cmp(&b.rank) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => {}
                }
            }

            Ordering::Less
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Card {
    rank: u64,
}

impl Card {
    fn parse(value: &str, wildcard: bool) -> Self {
        let chars: Vec<char> = value.chars().collect();
        let val = chars[0];
        let wildcard_value = if wildcard { 1 } else { 11 };

        Self {
            rank: calculate_card_rank(&val, wildcard_value),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<Card>,
    rank: u64,
    bid: u64,
}

impl Hand {
    fn parse(string: &str, wildcard: bool) -> Self {
        let parts: Vec<&str> = string.split_whitespace().collect();
        let cards: Vec<Card> = parts[0]
            .split("")
            .filter(|s| !s.is_empty())
            .map(|c| Card::parse(c, wildcard))
            .collect();

        let bid = parts[1].parse::<u64>().unwrap();
        let rank = calculate_hand_rank_wildcard(&cards, wildcard);

        Self { cards, rank, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_hands(self, other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub mod part1 {
    use super::*;
    use std::fs;

    fn total_winnings(contents: &str) -> u64 {
        let mut hands: Vec<Hand> = vec![];

        for line in contents.lines() {
            let hand = Hand::parse(line, false);
            hands.push(hand);
        }

        hands.sort();

        hands
            .into_iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u64 * h.bid)
            .sum()
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
            assert_eq!(Hand::parse("AAAAA 1", false).rank, 7);
        }

        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        #[test]
        fn test_hand_rank2() {
            assert_eq!(Hand::parse("AA8AA 1", false).rank, 6);
        }

        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        #[test]
        fn test_hand_rank3() {
            assert_eq!(Hand::parse("23332 1", false).rank, 5);
        }

        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        #[test]
        fn test_hand_rank4() {
            assert_eq!(Hand::parse("TTT98 1", false).rank, 4);
        }

        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        #[test]
        fn test_hand_rank5() {
            assert_eq!(Hand::parse("23432 1", false).rank, 3);
        }

        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        #[test]
        fn test_hand_rank6() {
            assert_eq!(Hand::parse("A23A4 1", false).rank, 2);
        }

        // High card, where all cards' labels are distinct: 23456
        #[test]
        fn test_hand_rank7() {
            assert_eq!(Hand::parse("23456 1", false).rank, 1);
        }
    }
}
pub mod part2 {
    use super::*;
    use std::fs;

    fn total_winnings(contents: &str) -> u64 {
        let mut hands: Vec<Hand> = vec![];

        for line in contents.lines() {
            let hand = Hand::parse(line, true);
            hands.push(hand);
        }

        hands.sort();

        hands
            .into_iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u64 * h.bid)
            .sum()
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day7.txt").expect("File not found");

        let result = total_winnings(&contents);

        println!("Day 7 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_total_winnings() {
            assert_eq!(
                total_winnings("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483"),
                5905
            );
        }

        // Five of a kind, where all five cards have the same label: AAAAA
        #[test]
        fn test_hand_rank1() {
            assert_eq!(Hand::parse("AAAAA 1", true).rank, 7);
        }

        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        #[test]
        fn test_hand_rank2() {
            assert_eq!(Hand::parse("AA8AA 1", true).rank, 6);
        }

        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        #[test]
        fn test_hand_rank3() {
            assert_eq!(Hand::parse("23332 1", true).rank, 5);
        }

        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        #[test]
        fn test_hand_rank4() {
            assert_eq!(Hand::parse("TTT98 1", true).rank, 4);
        }

        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        #[test]
        fn test_hand_rank5() {
            assert_eq!(Hand::parse("23432 1", true).rank, 3);
        }

        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        #[test]
        fn test_hand_rank6() {
            assert_eq!(Hand::parse("A23A4 1", true).rank, 2);
        }

        // High card, where all cards' labels are distinct: 23456
        #[test]
        fn test_hand_rank7() {
            assert_eq!(Hand::parse("23456 1", true).rank, 1);
        }

        #[test]
        fn test_hand_rank8() {
            assert_eq!(Hand::parse("32T3K 1", true).rank, 2);
        }

        #[test]
        fn test_hand_rank9() {
            assert_eq!(Hand::parse("KK677 1", true).rank, 3);
        }

        #[test]
        fn test_hand_rank10() {
            assert_eq!(Hand::parse("T55J5 1", true).rank, 6);
        }

        #[test]
        fn test_hand_rank11() {
            assert_eq!(Hand::parse("KTJJT 1", true).rank, 6);
        }

        #[test]
        fn test_hand_rank12() {
            assert_eq!(Hand::parse("QQQJA 1", true).rank, 6);
        }
    }
}
