pub mod part1 {
    use std::fs;

    fn extrapolate_digits(digits: &Vec<i64>) -> i64 {
        if digits.iter().filter(|&&digit| digit != 0).count() == 0 {
            return 0;
        }

        let mut next = vec![];

        for i in 0..digits.len() - 1 {
            next.push(digits[i + 1] - digits[i]);
        }

        digits[digits.len() - 1] + extrapolate_digits(&next)
    }

    fn extrapolate(contents: &str) -> i64 {
        let mut result: i64 = 0;
        for line in contents.lines() {
            let digits: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            result += extrapolate_digits(&digits);
        }

        result
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day9.txt").expect("File not found");

        let result = extrapolate(&contents);

        println!("Day 9 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day9_part1a() {
            assert_eq!(extrapolate("0 3 6 9 12 15"), 18);
        }

        #[test]
        fn test_day9_part1b() {
            assert_eq!(extrapolate("1 3 6 10 15 21"), 28);
        }

        #[test]
        fn test_day9_part1c() {
            assert_eq!(extrapolate("10 13 16 21 30 45"), 68);
        }

        #[test]
        fn test_day9_part1d() {
            assert_eq!(
                extrapolate("0 3 6 9 12 15")
                    + extrapolate("1 3 6 10 15 21")
                    + extrapolate("10 13 16 21 30 45"),
                114
            );
        }
    }
}

pub mod part2 {
    use std::fs;

    fn do_stuff(contents: &str) -> i32 {
        contents.len() as i32
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day9.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 9 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day9_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
