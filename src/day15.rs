fn hash(string: &str) -> usize {
    let mut result = 0;

    for char in string.chars() {
        result += char as usize;
        result *= 17;
        result %= 256;
    }

    result
}

pub mod part1 {
    use super::*;
    use std::fs;

    fn hash_sum(contents: &str) -> usize {
        let mut sum = 0;

        for string in contents.split(',') {
            sum += hash(string);
        }

        sum
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day15.txt").expect("File not found");

        let result = hash_sum(&contents);

        println!("Day 15 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day15_part1a() {
            assert_eq!(hash_sum("HASH"), 52);
        }

        #[test]
        fn test_day15_part1b() {
            assert_eq!(
                hash_sum("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
                1320
            );
        }
    }
}

pub mod part2 {
    use regex::Regex;

    use super::*;
    use std::fs;

    #[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
    enum Action {
        Insert,
        Remove,
    }

    type BoxEntry<'a> = (&'a str, usize, Action, usize);

    fn parse(string: &str) -> BoxEntry {
        let re = Regex::new(r"^(.*)([=\-])([0-9]*)$").unwrap();
        let (_, [p1, p2, p3]) = re.captures(string).expect("Failed to parse").extract();
        let action = if p2 == "=" {
            Action::Insert
        } else if p2 == "-" {
            Action::Remove
        } else {
            panic!("Failed to parse action")
        };

        (p1, hash(p1), action, p3.parse::<usize>().unwrap_or(0))
    }

    fn calculate_focusing_power(contents: &str) -> usize {
        let mut boxes: Vec<Vec<BoxEntry>> = vec![vec![]; 256];

        for string in contents.split(',') {
            let entry = parse(string);

            let (label, box_idx, action, _) = entry;

            if let Some(other_idx) = boxes[box_idx]
                .iter()
                .position(|(other_label, _, _, _)| *other_label == label)
            {
                if action == Action::Insert {
                    boxes[box_idx][other_idx] = entry;
                } else if action == Action::Remove {
                    boxes[box_idx].remove(other_idx);
                }
            } else {
                if action == Action::Insert {
                    boxes[box_idx].push(entry);
                }
            }
        }

        let mut result = 0;
        for (i, entries) in boxes.iter().enumerate() {
            for (j, (_, _, _, focal_length)) in entries.iter().enumerate() {
                result += (i + 1) * (j + 1) * focal_length;
            }
        }
        result
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day15.txt").expect("File not found");

        let result = calculate_focusing_power(&contents);

        println!("Day 15 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day15_part2a() {
            assert_eq!(hash("rn"), 0);
        }
        #[test]
        fn test_day15_part2b() {
            assert_eq!(
                calculate_focusing_power("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
                145
            );
        }
    }
}
