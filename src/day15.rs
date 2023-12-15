pub mod part1 {
    use std::fs;

    fn hash(string: &str) -> usize {
        let mut result = 0;

        for char in string.chars() {
            result += char as usize;
            result *= 17;
            result %= 256;
        }

        result
    }

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
    use std::fs;

    fn do_stuff(contents: &str) -> i32 {
        contents.len() as i32
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day15.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 15 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day15_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
