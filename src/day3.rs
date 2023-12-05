pub mod part1 {
    use std::fs;
    use substring::Substring;

    fn is_symbol(letter: char) -> bool {
        !matches!(
            letter,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.'
        )
    }

    fn is_symbol_in_range(line: &str, start: i32, end: i32) -> bool {
        let chars: Vec<char> = line.chars().collect();
        let max = (chars.len() as i32) - 1;

        for i in start..(end + 1) {
            if i >= 0 && i <= max && is_symbol(chars[i as usize]) {
                return true;
            }
        }

        false
    }

    fn engine_schematic_sum(schematic: &str) -> i32 {
        let mut result: i32 = 0;
        let lines: Vec<&str> = schematic.lines().collect();
        let lines_max = (lines.len() - 1) as i32;

        for (line_idx, line) in schematic.lines().enumerate() {
            let chars_len = line.len();
            let mut digit_start_idx: i32 = -1;
            let mut digit_end_idx: i32 = -1;

            for (letter_idx, letter) in line.chars().enumerate() {
                match letter.to_string().parse::<i32>() {
                    Ok(_digit) => {
                        if digit_start_idx < 0 {
                            digit_start_idx = letter_idx as i32;
                        }
                    }
                    Err(_error) => {
                        if digit_start_idx >= 0 {
                            // The previous index was the last numeric letter
                            digit_end_idx = (letter_idx as i32) - 1;
                        }
                    }
                }

                // Handle end of line
                if digit_start_idx >= 0 && digit_end_idx < 0 && letter_idx == chars_len - 1 {
                    digit_end_idx = letter_idx as i32;
                }

                if digit_start_idx >= 0 && digit_end_idx >= 0 && digit_end_idx >= digit_start_idx {
                    let digit =
                        line.substring(digit_start_idx as usize, (digit_end_idx as usize) + 1);

                    let prev_line_idx = (line_idx as i32) - 1;
                    let next_line_idx = (line_idx as i32) + 1;

                    let symbol_found_prev_line = prev_line_idx >= 0
                        && is_symbol_in_range(
                            lines[prev_line_idx as usize],
                            digit_start_idx - 1,
                            digit_end_idx + 1,
                        );

                    let symbol_found_curr_line =
                        is_symbol_in_range(line, digit_start_idx - 1, digit_end_idx + 1);

                    let symbol_found_next_line = next_line_idx <= lines_max
                        && is_symbol_in_range(
                            lines[next_line_idx as usize],
                            digit_start_idx - 1,
                            digit_end_idx + 1,
                        );

                    let symbol_found =
                        symbol_found_prev_line || symbol_found_curr_line || symbol_found_next_line;

                    if symbol_found {
                        match digit.parse::<i32>() {
                            Ok(digit) => result += digit,
                            Err(_error) => {}
                        }
                    }

                    digit_start_idx = -1;
                    digit_end_idx = -1;
                }
            }
        }

        result
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day3.txt").expect("File not found");

        let result = engine_schematic_sum(&contents);

        println!("Day 3 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_engine_schematic_sum() {
            assert_eq!(
                engine_schematic_sum("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."),
                4361
            );
        }

        #[test]
        fn test_engine_schematic_sum2() {
            assert_eq!(
              engine_schematic_sum("...*......\n467..114..\n..........\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."),
              4361
          );
        }

        #[test]
        fn test_engine_schematic_sum3() {
            assert_eq!(
              engine_schematic_sum("...*......\n467..114..\n..........\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n..........\n.664.598..\n...$.*...."),
              4361
          );
        }

        #[test]
        fn test_engine_schematic_sum4a() {
            assert_eq!(engine_schematic_sum("............*975"), 975);
        }

        #[test]
        fn test_engine_schematic_sum4b() {
            assert_eq!(
                engine_schematic_sum("............*...\n.............975"),
                975
            );
        }

        #[test]
        fn test_engine_schematic_sum5() {
            assert_eq!(engine_schematic_sum("975*............"), 975);
        }

        #[test]
        fn test_engine_schematic_sum6() {
            assert_eq!(engine_schematic_sum("*975............"), 975);
        }

        #[test]
        fn test_engine_schematic_sum7() {
            assert_eq!(engine_schematic_sum(".....+.58."), 0);
        }

        #[test]
        fn test_engine_schematic_sum8() {
            assert_eq!(engine_schematic_sum("....694@980..."), 1674);
        }

        #[test]
        fn test_engine_schematic_sum9() {
            assert_eq!(engine_schematic_sum("........\n.24..4..\n......*."), 4);
        }

        #[test]
        fn test_engine_schematic_sum10() {
            assert_eq!(engine_schematic_sum("........\n.34....*\n......24"), 24);
        }

        #[test]
        fn test_engine_schematic_sum11() {
            assert_eq!(engine_schematic_sum("*.......\n.34....."), 34);
        }

        #[test]
        fn test_engine_schematic_sum12() {
            assert_eq!(engine_schematic_sum("........\n.+34....."), 34);
        }

        #[test]
        fn test_engine_schematic_sum13() {
            assert_eq!(engine_schematic_sum("........\n.-34....."), 34);
        }

        #[test]
        fn test_engine_schematic_sum14() {
            assert_eq!(engine_schematic_sum("......34\n........."), 0);
        }

        #[test]
        fn test_engine_schematic_sum15() {
            assert_eq!(engine_schematic_sum("12+.\n...."), 12);
        }

        #[test]
        fn test_engine_schematic_sum16() {
            assert_eq!(
                engine_schematic_sum("$......$\n.1....1.\n.1....1.\n$......$"),
                4
            );
        }

        #[test]
        fn test_engine_schematic_sum17() {
            assert_eq!(
                engine_schematic_sum("$..\n.11\n.11\n$..\n..$\n11.\n11.\n..$"),
                44
            );
        }

        #[test]
        fn test_engine_schematic_sum18a() {
            assert_eq!(engine_schematic_sum("12...\n..#.."), 12);
        }

        #[test]
        fn test_engine_schematic_sum18b() {
            assert_eq!(engine_schematic_sum("12...\n...#."), 0);
        }
    }
}

pub mod part2 {
    use std::{collections::HashSet, fs};

    fn is_digit(letter: char) -> bool {
        matches!(
            letter,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
        )
    }

    fn expand_digit(line: &str, start: usize) -> i32 {
        let chars: Vec<char> = line.chars().collect();
        let max = chars.len() - 1;

        let mut result = String::new();

        if start >= max || !is_digit(chars[start]) {
            return 0;
        }

        for i in (0..start).rev() {
            let letter = chars[i];
            if !is_digit(letter) {
                break;
            }

            result.insert(0, letter);
        }

        for &letter in chars.iter().take(max + 1).skip(start) {
            if !is_digit(letter) {
                break;
            }

            result.push(letter);
        }

        match result.parse::<i32>() {
            Ok(digit) => digit,
            Err(_error) => 0,
        }
    }

    fn gear_ratios(schematic: &str) -> i32 {
        let mut result: i32 = 0;
        let lines: Vec<&str> = schematic.lines().collect();
        let lines_max = (lines.len() - 1) as i32;

        for (line_idx, line) in schematic.lines().enumerate() {
            for (letter_idx, letter) in line.chars().enumerate() {
                if letter == '*' {
                    let prev_line_idx = (line_idx as i32) - 1;
                    let next_line_idx = (line_idx as i32) + 1;

                    let mut prev_line_digit_left = 0;
                    let mut prev_line_digit_middle = 0;
                    let mut prev_line_digit_right = 0;
                    if prev_line_idx >= 0 {
                        if letter_idx > 0 {
                            prev_line_digit_left =
                                expand_digit(lines[prev_line_idx as usize], letter_idx - 1);
                        }
                        prev_line_digit_middle =
                            expand_digit(lines[prev_line_idx as usize], letter_idx);
                        prev_line_digit_right =
                            expand_digit(lines[prev_line_idx as usize], letter_idx + 1);
                    }

                    let mut curr_line_digit_left = 0;
                    if letter_idx > 0 {
                        curr_line_digit_left = expand_digit(line, letter_idx - 1)
                    }
                    let curr_line_digit_right = expand_digit(line, letter_idx + 1);

                    let mut next_line_digit_left = 0;
                    let mut next_line_digit_middle = 0;
                    let mut next_line_digit_right = 0;
                    if next_line_idx <= lines_max {
                        if letter_idx > 0 {
                            next_line_digit_left =
                                expand_digit(lines[next_line_idx as usize], letter_idx - 1);
                        }
                        next_line_digit_middle =
                            expand_digit(lines[next_line_idx as usize], letter_idx);
                        next_line_digit_right =
                            expand_digit(lines[next_line_idx as usize], letter_idx + 1);
                    }

                    let unique = HashSet::from([
                        prev_line_digit_left,
                        prev_line_digit_middle,
                        prev_line_digit_right,
                        curr_line_digit_left,
                        curr_line_digit_right,
                        next_line_digit_left,
                        next_line_digit_middle,
                        next_line_digit_right,
                    ]);

                    let filtered: Vec<&i32> = unique.iter().filter(|&&digit| digit > 0).collect();

                    if filtered.len() == 2 {
                        let mut mult = 1;
                        for digit in filtered {
                            mult *= digit;
                        }
                        result += mult;
                    }
                }
            }
        }

        result
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day3.txt").expect("File not found");

        let result = gear_ratios(&contents);

        println!("Day 3 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_gear_ratio1() {
            assert_eq!(
              gear_ratios("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."),
              467835
          );
        }

        #[test]
        fn test_gear_ratio2() {
            assert_eq!(gear_ratios("....832*105..@.."), 87360);
        }

        #[test]
        fn test_gear_ratio3() {
            assert_eq!(gear_ratios("....832.105..@..\n.......*........"), 87360);
        }

        #[test]
        fn test_gear_ratio4() {
            assert_eq!(gear_ratios("......832....@..\n.......*........"), 0);
        }

        #[test]
        fn test_gear_ratio5() {
            assert_eq!(gear_ratios("....832*.......\n........105..@."), 87360);
        }

        #[test]
        fn test_gear_ratio6() {
            assert_eq!(gear_ratios("*832.......\n.105..@...."), 87360);
        }

        #[test]
        fn test_gear_ratio7() {
            assert_eq!(gear_ratios("......832*\n.......105"), 87360);
        }

        #[test]
        fn test_gear_ratio8() {
            assert_eq!(gear_ratios(".......832\n......105*"), 87360);
        }

        #[test]
        fn test_gear_ratio9() {
            assert_eq!(gear_ratios(".+.................\n...442...997..187..\n....*....*.......=.\n...963.926.39../...\n...........*....63.\n.....591...127....."), 1353821);
        }

        #[test]
        fn test_gear_ratio10() {
            assert_eq!(gear_ratios("...9...\n...*...\n...3..."), 27);
        }
    }
}
