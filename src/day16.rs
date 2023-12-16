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

pub mod part1 {
    use super::*;

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
    use super::*;
    use std::fs;

    fn count_energized_tiles_multiple_entrypoints(contents: &str) -> usize {
        let matrix: Vec<Vec<&str>> = contents
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|line: &&str| !line.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        let mut configurations: HashMap<(usize, usize), usize> = HashMap::new();

        for x in 0..matrix.len() {
            let mut visited1: HashMap<(usize, usize), usize> = HashMap::new();
            for (x, row) in matrix.iter().enumerate() {
                for (y, _) in row.iter().enumerate() {
                    visited1.insert((x, y), 0);
                }
            }

            let mut visited2: HashMap<(usize, usize), usize> = HashMap::new();
            for (x, row) in matrix.iter().enumerate() {
                for (y, _) in row.iter().enumerate() {
                    visited1.insert((x, y), 0);
                }
            }

            let mut visited_from_direction1: HashMap<(usize, usize, Direction), bool> =
                HashMap::new();

            traverse_matrix(
                &matrix,
                &mut visited1,
                &mut visited_from_direction1,
                x,
                0,
                Direction::Right,
            );

            configurations.insert(
                (x, 0),
                visited1.iter().filter(|(_, &count)| count > 0).count(),
            );

            let mut visited_from_direction2: HashMap<(usize, usize, Direction), bool> =
                HashMap::new();

            traverse_matrix(
                &matrix,
                &mut visited2,
                &mut visited_from_direction2,
                x,
                matrix[x].len() - 1,
                Direction::Left,
            );

            configurations.insert(
                (x, matrix[x].len() - 1),
                visited2.iter().filter(|(_, &count)| count > 0).count(),
            );
        }

        for y in 0..matrix[0].len() {
            let mut visited1: HashMap<(usize, usize), usize> = HashMap::new();
            for (x, row) in matrix.iter().enumerate() {
                for (y, _) in row.iter().enumerate() {
                    visited1.insert((x, y), 0);
                }
            }

            let mut visited2: HashMap<(usize, usize), usize> = HashMap::new();
            for (x, row) in matrix.iter().enumerate() {
                for (y, _) in row.iter().enumerate() {
                    visited1.insert((x, y), 0);
                }
            }

            let mut visited_from_direction1: HashMap<(usize, usize, Direction), bool> =
                HashMap::new();

            traverse_matrix(
                &matrix,
                &mut visited1,
                &mut visited_from_direction1,
                0,
                y,
                Direction::Down,
            );

            configurations.insert(
                (0, y),
                visited1.iter().filter(|(_, &count)| count > 0).count(),
            );

            let mut visited_from_direction2: HashMap<(usize, usize, Direction), bool> =
                HashMap::new();

            traverse_matrix(
                &matrix,
                &mut visited2,
                &mut visited_from_direction2,
                matrix.len() - 1,
                y,
                Direction::Up,
            );

            configurations.insert(
                (matrix.len() - 1, y),
                visited2.iter().filter(|(_, &count)| count > 0).count(),
            );
        }

        *configurations.values().max().unwrap()
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day16.txt").expect("File not found");

        let result = count_energized_tiles_multiple_entrypoints(&contents);

        println!("Day 16 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day16_part2() {
            assert_eq!(
                count_energized_tiles_multiple_entrypoints(
                    &fs::read_to_string("inputs/day16-example.txt").expect("File not found")
                ),
                51
            );
        }
    }
}
