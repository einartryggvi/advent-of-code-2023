pub mod part1 {
    use std::fs;

    fn count_steps(contents: &str) -> i32 {
        let (directions_str, nodes_str) = contents.split_once("\n\n").unwrap();
        let directions: Vec<&str> = directions_str.split("").filter(|s| !s.is_empty()).collect();
        let nodes = &nodes_str.lines();
        let num_directions = directions.len();

        let mut next_node = "AAA".to_string();
        let mut count: i32 = 0;
        let mut dir_key: usize = 0;

        while next_node != "ZZZ" {
            for node in nodes.clone() {
                let direction = directions[dir_key % num_directions];
                let (part1, part2) = node.split_once(" = ").unwrap();
                let (left_node, right_node) = part2.split_once(", ").unwrap();

                if next_node != part1 {
                    continue;
                }

                dir_key += 1;
                count += 1;

                next_node = if direction == "L" {
                    left_node.replace("(", "")
                } else {
                    right_node.replace(")", "")
                };

                if next_node == "ZZZ" {
                    break;
                }
            }
        }

        count
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day8.txt").expect("File not found");
        let result = count_steps(&contents);

        println!("Day 8 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day8_part1a() {
            assert_eq!(
                count_steps(
                    "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)"
                ),
                2
            );
        }

        #[test]
        fn test_day8_part1b() {
            assert_eq!(
                count_steps("LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)"),
                6
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
        let contents = fs::read_to_string("inputs/day8.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 8 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day8_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
