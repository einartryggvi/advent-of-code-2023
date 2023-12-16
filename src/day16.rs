pub mod part1 {
    use std::{collections::HashMap, fs};

    #[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn traverse_matrix(
        matrix: &Vec<Vec<&str>>,
        visited: &mut HashMap<(usize, usize), usize>,
        visited_from_direction: &mut HashMap<(usize, usize, Direction), bool>,
        x: usize,
        y: usize,
        direction: Direction,
    ) {
        if x > matrix.len() - 1 || y > matrix[x].len() - 1 {
            return;
        }

        visited.entry((x, y)).and_modify(|c| *c = *c + 1);

        if *visited_from_direction
            .get(&(x, y, direction))
            .unwrap_or(&false)
        {
            return;
        }

        visited_from_direction.insert((x, y, direction), true);

        if direction == Direction::Left {
            if y > 0 && (matrix[x][y] == "." || matrix[x][y] == "-") {
                traverse_matrix(matrix, visited, visited_from_direction, x, y - 1, direction);
            }

            if matrix[x][y] == "|" {
                if x > 0 {
                    traverse_matrix(
                        matrix,
                        visited,
                        visited_from_direction,
                        x - 1,
                        y,
                        Direction::Up,
                    );
                }
                traverse_matrix(
                    matrix,
                    visited,
                    visited_from_direction,
                    x + 1,
                    y,
                    Direction::Down,
                );
            }

            if matrix[x][y] == "\\" {
                if x > 0 {
                    traverse_matrix(
                        matrix,
                        visited,
                        visited_from_direction,
                        x - 1,
                        y,
                        Direction::Up,
                    );
                }
            }

            if matrix[x][y] == "/" {
                traverse_matrix(
                    matrix,
                    visited,
                    visited_from_direction,
                    x + 1,
                    y,
                    Direction::Down,
                );
            }
        }

        if direction == Direction::Right {
            if y < matrix[x].len() - 1 && (matrix[x][y] == "." || matrix[x][y] == "-") {
                traverse_matrix(matrix, visited, visited_from_direction, x, y + 1, direction);
            }

            if matrix[x][y] == "|" {
                if x > 0 {
                    traverse_matrix(
                        matrix,
                        visited,
                        visited_from_direction,
                        x - 1,
                        y,
                        Direction::Up,
                    );
                }
                traverse_matrix(
                    matrix,
                    visited,
                    visited_from_direction,
                    x + 1,
                    y,
                    Direction::Down,
                );
            }

            if matrix[x][y] == "\\" {
                traverse_matrix(
                    matrix,
                    visited,
                    visited_from_direction,
                    x + 1,
                    y,
                    Direction::Down,
                );
            }

            if matrix[x][y] == "/" {
                if x > 0 {
                    traverse_matrix(
                        matrix,
                        visited,
                        visited_from_direction,
                        x - 1,
                        y,
                        Direction::Up,
                    );
                }
            }
        }

        if direction == Direction::Up {
            if x > 0 && (matrix[x][y] == "." || matrix[x][y] == "|") {
                traverse_matrix(matrix, visited, visited_from_direction, x - 1, y, direction);
            }

            if matrix[x][y] == "-" {
                if y > 0 {
                    traverse_matrix(
                        matrix,
                        visited,
                        visited_from_direction,
                        x,
                        y - 1,
                        Direction::Left,
                    );
                }
                traverse_matrix(
                    matrix,
                    visited,
                    visited_from_direction,
                    x,
                    y + 1,
                    Direction::Right,
                );
            }

            if matrix[x][y] == "\\" {
                if y > 0 {
                    traverse_matrix(
                        matrix,
                        visited,
                        visited_from_direction,
                        x,
                        y - 1,
                        Direction::Left,
                    );
                }
            }

            if matrix[x][y] == "/" {
                traverse_matrix(
                    matrix,
                    visited,
                    visited_from_direction,
                    x,
                    y + 1,
                    Direction::Right,
                );
            }
        }

        if direction == Direction::Down {
            if x < matrix.len() - 1 && (matrix[x][y] == "." || matrix[x][y] == "|") {
                traverse_matrix(matrix, visited, visited_from_direction, x + 1, y, direction);
            }

            if matrix[x][y] == "-" {
                if y > 0 {
                    traverse_matrix(
                        matrix,
                        visited,
                        visited_from_direction,
                        x,
                        y - 1,
                        Direction::Left,
                    );
                }
                traverse_matrix(
                    matrix,
                    visited,
                    visited_from_direction,
                    x,
                    y + 1,
                    Direction::Right,
                );
            }

            if matrix[x][y] == "\\" {
                traverse_matrix(
                    matrix,
                    visited,
                    visited_from_direction,
                    x,
                    y + 1,
                    Direction::Right,
                );
            }

            if matrix[x][y] == "/" {
                if y > 0 {
                    traverse_matrix(
                        matrix,
                        visited,
                        visited_from_direction,
                        x,
                        y - 1,
                        Direction::Left,
                    );
                }
            }
        }
    }

    fn count_energized_tiles(contents: &str) -> usize {
        let matrix: Vec<Vec<&str>> = contents
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|line: &&str| !line.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
        for (x, row) in matrix.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                visited.insert((x, y), 0);
            }
        }

        let mut visited_from_direction: HashMap<(usize, usize, Direction), bool> = HashMap::new();

        traverse_matrix(
            &matrix,
            &mut visited,
            &mut visited_from_direction,
            0,
            0,
            Direction::Right,
        );

        visited.iter().filter(|(_, &count)| count > 0).count()
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day16.txt").expect("File not found");

        let result = count_energized_tiles(&contents);

        println!("Day 16 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day16_part1() {
            assert_eq!(
                count_energized_tiles(
                    &fs::read_to_string("inputs/day16-example.txt").expect("File not found")
                ),
                46
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
        let contents = fs::read_to_string("inputs/day16.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 16 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day16_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
