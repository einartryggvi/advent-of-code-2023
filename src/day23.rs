pub mod part1 {
    use std::fs;

    fn do_stuff(contents: &str) -> i32 {
        contents.len() as i32
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day23.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 23 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day23_part1() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}

pub mod part2 {
    use std::fs;

    fn do_stuff(contents: &str) -> i32 {
        contents.len() as i32
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day23.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 23 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day23_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
