fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    dx + dy
}
pub mod part1 {
    use super::*;
    use std::fs;

    fn distance_sum(contents: &str) -> i64 {
        let matrix: Vec<Vec<&str>> = contents
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        let mut expanded_matrix: Vec<Vec<&str>> = vec![vec![]];

        for line in matrix.iter() {
            let mut expanded_line: Vec<&str> = vec![];

            for (y, val) in line.iter().enumerate() {
                let column: Vec<&str> = matrix.iter().map(|l| l[y]).collect();
                expanded_line.push(val);
                if column.iter().all(|&val| val == ".") {
                    expanded_line.push(val);
                }
            }

            expanded_matrix.push(expanded_line.clone());
            if line.iter().all(|&val| val == ".") {
                expanded_matrix.push(expanded_line.clone());
            }
        }

        let mut galaxies: Vec<(usize, usize)> = vec![];
        for (x, line) in expanded_matrix.iter().enumerate() {
            for (y, val) in line.iter().enumerate() {
                if *val == "#" || val.parse::<i64>().is_ok() {
                    galaxies.push((x, y));
                }
            }
        }

        let mut edges: Vec<((usize, usize), (usize, usize))> = vec![];
        for i in 0..galaxies.len() {
            for j in i..galaxies.len() {
                edges.push((galaxies[i], galaxies[j]));
            }
        }

        let mut sum = 0;
        for ((x1, y1), (x2, y2)) in edges {
            sum += manhattan_distance(x1 as i64, y1 as i64, x2 as i64, y2 as i64);
        }

        sum
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day11.txt").expect("File not found");

        let result = distance_sum(&contents);

        println!("Day 11 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day11_part1() {
            assert_eq!(distance_sum("...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#....."), 374);
        }
    }
}

pub mod part2 {
    use super::*;
    use std::{collections::HashMap, fs};

    fn distance_sum(contents: &str, expand_factor: usize) -> i64 {
        let matrix: Vec<Vec<&str>> = contents
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        let mut multipliers_x: HashMap<usize, usize> = HashMap::new();
        let mut multipliers_y: HashMap<usize, usize> = HashMap::new();

        for (x, line) in matrix.iter().enumerate() {
            let should_multiply_line = line.iter().all(|&val| val == ".");

            for (y, _) in line.iter().enumerate() {
                let column: Vec<&str> = matrix.iter().map(|l| l[y]).collect();
                let should_multiply_column = column.iter().all(|&val| val == ".");

                if should_multiply_line {
                    multipliers_x.insert(x, expand_factor);
                }

                if should_multiply_column {
                    multipliers_y.insert(y, expand_factor);
                }
            }
        }

        let mut galaxies: Vec<(usize, usize)> = vec![];
        let mut x_extra = 0;
        for (x, line) in matrix.iter().enumerate() {
            x_extra += *multipliers_x.get(&x).unwrap_or(&1) - 1;

            let mut y_extra = 0;
            for (y, val) in line.iter().enumerate() {
                y_extra += *multipliers_y.get(&y).unwrap_or(&1) - 1;
                if *val == "#" || val.parse::<i64>().is_ok() {
                    galaxies.push((x + x_extra, y + y_extra));
                }
            }
        }

        let mut edges: Vec<((usize, usize), (usize, usize))> = vec![];
        for i in 0..galaxies.len() {
            for j in i..galaxies.len() {
                edges.push((galaxies[i], galaxies[j]));
            }
        }

        let mut sum: i64 = 0;
        for ((x1, y1), (x2, y2)) in edges {
            sum += manhattan_distance(x1 as i64, y1 as i64, x2 as i64, y2 as i64);
        }

        sum
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day11.txt").expect("File not found");

        let result = distance_sum(&contents, 1000000);

        println!("Day 11 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day11_part2() {
            assert_eq!(distance_sum("...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....", 100), 8410);
        }
    }
}
