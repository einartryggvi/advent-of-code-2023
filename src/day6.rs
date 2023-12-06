fn calculate_distance(time: u64, hold: u64) -> u64 {
    let time_left = time - hold;
    hold * time_left
}

pub mod part1 {
    use super::*;
    use regex::Regex;
    use std::fs;

    fn calculate_winning_margin(contents: &str) -> u64 {
        let lines: Vec<&str> = contents.lines().collect();

        let lines_parsed: Vec<&str> = lines
            .iter()
            .map(|line| {
                let (_, [numbers]) = Regex::new(r"^.*: (.*)$")
                    .expect("Invalid regex")
                    .captures(line)
                    .expect("Failed to parse line")
                    .extract();

                numbers
            })
            .collect();

        let games = lines_parsed[0]
            .split_whitespace()
            .map(|number| number.parse::<u64>().unwrap())
            .zip(
                lines_parsed[1]
                    .split_whitespace()
                    .map(|number| number.parse::<u64>().unwrap()),
            );

        let mut margin_of_error: Vec<u64> = Vec::new();

        for (time, best_distance) in games {
            let mut count_winnable: u64 = 0;

            for t in 0..(time + 1) {
                let hold = time - t;

                if calculate_distance(time, hold) > best_distance {
                    count_winnable += 1;
                }
            }

            margin_of_error.push(count_winnable);
        }

        margin_of_error.iter().product()
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day6.txt").expect("File not found");

        let result = calculate_winning_margin(&contents);

        println!("Day 6 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate_winning_margin() {
            assert_eq!(
                calculate_winning_margin("Time:      7  15   30\nDistance:  9  40  200"),
                288
            );
        }

        #[test]
        fn test_distance1() {
            assert_eq!(calculate_distance(7, 0), 0);
        }

        #[test]
        fn test_distance2() {
            assert_eq!(calculate_distance(7, 1), 6);
        }

        #[test]
        fn test_distance3() {
            assert_eq!(calculate_distance(7, 2), 10);
        }

        #[test]
        fn test_distance4() {
            assert_eq!(calculate_distance(7, 3), 12);
        }

        #[test]
        fn test_distance5() {
            assert_eq!(calculate_distance(7, 4), 12);
        }

        #[test]
        fn test_distance6() {
            assert_eq!(calculate_distance(7, 5), 10);
        }

        #[test]
        fn test_distance7() {
            assert_eq!(calculate_distance(7, 6), 6);
        }

        #[test]
        fn test_distance8() {
            assert_eq!(calculate_distance(7, 7), 0);
        }
    }
}

pub mod part2 {
    use super::*;
    use regex::Regex;
    use std::fs;

    fn calculate_winnings(contents: &str) -> u64 {
        let lines: Vec<&str> = contents.lines().collect();

        let lines_parsed: Vec<&str> = lines
            .iter()
            .map(|line| {
                let (_, [numbers]) = Regex::new(r"^.*: (.*)$")
                    .expect("Invalid regex")
                    .captures(line)
                    .expect("Failed to parse line")
                    .extract();

                numbers
            })
            .collect();

        let time = lines_parsed[0].replace(' ', "").parse::<u64>().unwrap();
        let best_distance = lines_parsed[1].replace(' ', "").parse::<u64>().unwrap();

        let mut count_winnable = 0;
        for t in 0..(time + 1) {
            let hold = time - t;

            if calculate_distance(time, hold) > best_distance {
                count_winnable += 1;
            }
        }

        count_winnable
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day6.txt").expect("File not found");

        let result = calculate_winnings(&contents);

        println!("Day 6 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate_winnings() {
            assert_eq!(
                calculate_winnings("Time:      7  15   30\nDistance:  9  40  200"),
                71503
            );
        }
    }
}
