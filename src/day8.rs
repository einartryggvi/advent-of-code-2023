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

    fn count_steps(&self, initial_node: &Node) -> u64 {
        let mut next_node = initial_node;
        let mut count: u64 = 0;
        let mut dir_key: usize = 0;

        while !next_node.node.ends_with('Z') {
            let direction = self.get_direction(dir_key);
            next_node = if direction == "L" {
                self.find_node(&next_node.left)
            } else {
                self.find_node(&next_node.right)
            };

            dir_key += 1;
            count += 1;
        }

        count
    }
}
pub mod part1 {
    use super::*;
    use std::fs;

    fn count_steps(contents: &str) -> u64 {
        let map = Map::parse(contents);
        let initial_node = map.find_node(&"AAA".to_string());

        map.count_steps(initial_node)
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

    fn count_steps(contents: &str) -> u64 {
        let map = Map::parse(contents);

        map.nodes
            .iter()
            .filter(|node| node.node.ends_with('A'))
            .map(|initial_node| map.count_steps(initial_node))
            .fold(1, lcm)
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
