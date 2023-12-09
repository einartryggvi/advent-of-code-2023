fn extrapolate_digits(digits: &Vec<i64>, direction: &ExtrapolateDirection) -> i64 {
    if digits.iter().filter(|&&digit| digit != 0).count() == 0 {
        return 0;
    }

    let mut next = vec![];

    for i in 0..digits.len() - 1 {
        if *direction == ExtrapolateDirection::Right {
            next.push(digits[i + 1] - digits[i]);
        } else {
            next.push(digits[i] - digits[i + 1]);
        }
    }

    let key = if *direction == ExtrapolateDirection::Right {
        digits.len() - 1
    } else {
        0
    };

    digits[key] + extrapolate_digits(&next, direction)
}

fn extrapolate(contents: &str, direction: &ExtrapolateDirection) -> i64 {
    let mut result: i64 = 0;
    for line in contents.lines() {
        let digits: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        result += extrapolate_digits(&digits, direction);
    }

    result
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum ExtrapolateDirection {
    Left,
    Right,
}
pub mod part1 {
    use super::*;
    use std::fs;

    pub fn run() {
        let contents = fs::read_to_string("inputs/day9.txt").expect("File not found");

        let result = extrapolate(&contents, &ExtrapolateDirection::Right);

        println!("Day 9 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day9_part1a() {
            assert_eq!(
                extrapolate("0 3 6 9 12 15", &ExtrapolateDirection::Right),
                18
            );
        }

        #[test]
        fn test_day9_part1b() {
            assert_eq!(
                extrapolate("1 3 6 10 15 21", &ExtrapolateDirection::Right),
                28
            );
        }

        #[test]
        fn test_day9_part1c() {
            assert_eq!(
                extrapolate("10 13 16 21 30 45", &ExtrapolateDirection::Right),
                68
            );
        }

        #[test]
        fn test_day9_part1d() {
            assert_eq!(
                extrapolate("0 3 6 9 12 15", &ExtrapolateDirection::Right)
                    + extrapolate("1 3 6 10 15 21", &ExtrapolateDirection::Right)
                    + extrapolate("10 13 16 21 30 45", &ExtrapolateDirection::Right),
                114
            );
        }
    }
}

pub mod part2 {
    use super::*;
    use std::fs;

    pub fn run() {
        let contents = fs::read_to_string("inputs/day9.txt").expect("File not found");

        let result = extrapolate(&contents, &ExtrapolateDirection::Left);

        println!("Day 9 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day9_part2a() {
            assert_eq!(
                extrapolate("10 13 16 21 30 45", &ExtrapolateDirection::Left),
                5
            );
        }

        #[test]
        fn test_day9_part2b() {
            assert_eq!(
                extrapolate("0 3 6 9 12 15", &ExtrapolateDirection::Left)
                    + extrapolate("1 3 6 10 15 21", &ExtrapolateDirection::Left)
                    + extrapolate("10 13 16 21 30 45", &ExtrapolateDirection::Left),
                2
            );
        }
    }
}
