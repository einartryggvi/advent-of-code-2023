use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    node: String,
    left: String,
    right: String,
}

impl Node {
    fn parse(node: &str) -> Self {
        let (part1, part2) = node.split_once(" = ").unwrap();
        let (left_node, right_node) = part2.split_once(", ").unwrap();

        Self {
            node: part1.to_string(),
            left: left_node.to_string().replace('(', ""),
            right: right_node.to_string().replace(')', ""),
        }
    }
}

#[derive(Debug)]
struct Map {
    directions: Vec<String>,
    num_directions: usize,
    nodes: Vec<Node>,
    nodes_map: HashMap<String, Node>,
}

impl Map {
    fn parse(contents: &str) -> Self {
        let (directions_str, nodes_str) = contents.split_once("\n\n").unwrap();
        let directions: Vec<String> = directions_str
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        let nodes: Vec<Node> = nodes_str.lines().map(Node::parse).collect();
        let num_directions = directions.len();

        let mut nodes_map: HashMap<String, Node> = HashMap::new();
        for line in nodes_str.lines() {
            let node = Node::parse(line);
            nodes_map.insert(node.node.clone(), node);
        }

        Self {
            directions,
            num_directions,
            nodes,
            nodes_map,
        }
    }

    fn get_direction(&self, dir_key: usize) -> String {
        self.directions[dir_key % self.num_directions].clone()
    }

    fn find_node(&self, key: &String) -> &Node {
        self.nodes_map.get(key).unwrap()
    }
}
pub mod part1 {
    use super::*;
    use std::fs;

    fn count_steps(contents: &str) -> i32 {
        let mut next_node = "AAA".to_string();
        let mut count: i32 = 0;
        let mut dir_key: usize = 0;

        let map = Map::parse(contents);

        while next_node != "ZZZ" {
            for node in &map.nodes {
                let direction = map.get_direction(dir_key);

                if next_node != node.node {
                    continue;
                }

                dir_key += 1;
                count += 1;

                next_node = if direction == "L" {
                    node.left.clone()
                } else {
                    node.right.clone()
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
    use super::*;
    use num::integer::lcm;
    use std::fs;

    fn lcm_vec(numbers: Vec<u64>) -> u64 {
        numbers.into_iter().fold(1, lcm)
    }

    fn count_steps(contents: &str) -> u64 {
        let map = Map::parse(contents);

        let start_nodes: Vec<&Node> = map
            .nodes
            .iter()
            .filter(|node| node.node.ends_with('A'))
            .collect();

        let mut counts: Vec<u64> = vec![];
        for node in &start_nodes {
            let mut next_node = *node;
            let mut count: u64 = 0;
            let mut dir_key: usize = 0;

            while !next_node.node.ends_with('Z') {
                let direction = map.get_direction(dir_key);
                next_node = if direction == "L" {
                    map.find_node(&next_node.left)
                } else {
                    map.find_node(&next_node.right)
                };

                dir_key += 1;
                count += 1;
            }

            counts.push(count);
        }

        lcm_vec(counts)
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day8.txt").expect("File not found");

        let result = count_steps(&contents);

        println!("Day 8 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day8_part2a() {
            assert_eq!(
                count_steps(
                    "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)"
                ),
                6
            );
        }
    }
}
