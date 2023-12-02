pub mod part1 {
    use std::fs;
    pub fn run() {
        let contents = fs::read_to_string("inputs/day1.txt").expect("File not found");

        let mut result = 0;

        for line in contents.split("\n").into_iter() {
            let mut first_digit = -1;
            let mut last_digit = -1;

            for letter in line.split("").into_iter() {
                match letter.parse::<i32>() {
                    Ok(letter_numeric) => {
                        if first_digit < 0 {
                            first_digit = letter_numeric;
                        }

                        last_digit = letter_numeric;
                    }
                    Err(_error) => {}
                }
            }

            if first_digit >= 0 && last_digit >= 0 {
                match format!("{}{}", first_digit, last_digit).parse::<i32>() {
                    Ok(numeric) => {
                        result += numeric;
                    }
                    Err(_error) => {}
                }
            }
        }

        println!("Day 1 Part 1: {}", result);
    }
}

pub mod part2 {
    use std::fs;

    fn line_to_numerics(line: &str) -> String {
        let mut result = "".to_string();

        for letter in line.split("").into_iter() {
            result.push_str(letter);
            result = result
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9");
        }

        result
    }

    fn line_to_numerics_reverse(line: &str) -> String {
        let mut result = "".to_string();

        for letter in line.chars().rev() {
            result = format!("{}{}", letter.to_string(), result);
            result = result
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9");
        }

        result
    }

    fn line_to_digit(line: &str) -> i32 {
        let mut first_digit = -1;
        let mut last_digit = -1;

        let line_numerics = line_to_numerics(line);
        let line_numerics_reverse = line_to_numerics_reverse(line);

        for letter in line_numerics.split("").into_iter() {
            match letter.parse::<i32>() {
                Ok(letter_numeric) => {
                    if first_digit < 0 {
                        first_digit = letter_numeric;
                    }
                }
                Err(_error) => {}
            }
        }

        for letter in line_numerics_reverse.split("").into_iter() {
            match letter.parse::<i32>() {
                Ok(letter_numeric) => {
                    last_digit = letter_numeric;
                }
                Err(_error) => {}
            }
        }

        if first_digit >= 0 && last_digit >= 0 {
            match format!("{}{}", first_digit, last_digit).parse::<i32>() {
                Ok(numeric) => {
                    return numeric;
                }
                Err(_error) => {}
            }
        }

        0
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day1.txt").expect("File not found");

        let mut result = 0;

        for line in contents.split("\n").into_iter() {
            result += line_to_digit(line)
        }

        println!("Day 1 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_line_to_digit1() {
            assert_eq!(line_to_digit("two1nine"), 29);
        }
        #[test]
        fn test_line_to_digit2() {
            assert_eq!(line_to_digit("eightwothree"), 83);
        }
        #[test]
        fn test_line_to_digit3() {
            assert_eq!(line_to_digit("abcone2threexyz"), 13);
        }
        #[test]
        fn test_line_to_digit4() {
            assert_eq!(line_to_digit("xtwone3four"), 24);
        }
        #[test]
        fn test_line_to_digit5() {
            assert_eq!(line_to_digit("4nineeightseven2"), 42);
        }
        #[test]
        fn test_line_to_digit6() {
            assert_eq!(line_to_digit("zoneight234"), 14);
        }
        #[test]
        fn test_line_to_digit7() {
            assert_eq!(line_to_digit("7pqrstsixteen"), 76);
        }
        #[test]
        fn test_line_to_digit8() {
            assert_eq!(line_to_digit("qkdoneighttwo1one3"), 13);
        }
    }
}
