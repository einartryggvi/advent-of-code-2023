fn count_spaces_before(col: &Vec<&str>, mut start: isize) -> usize {
    let mut count = 0;

    while start >= 0 {
        if col[start as usize] == "#" {
            break;
        }

        if col[start as usize] == "." {
            count += 1;
        }

        start -= 1;
    }

    count
}

pub mod part1 {
    use super::*;
    use std::fs;

    fn calculate_load(contents: &str) -> usize {
        let mut count = 0;

        let matrix: Vec<Vec<&str>> = contents
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|line: &&str| !line.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        let mut cols = vec![];
        for y in 0..matrix[0].len() {
            let mut col: Vec<&str> = vec![];
            for x in 0..matrix.len() {
                col.push(matrix[x][y]);
            }
            cols.push(col);
        }

        for col in &cols {
            for (i, key) in col.iter().enumerate() {
                if *key == "O" {
                    count += col.len() - i + count_spaces_before(&col, i as isize);
                }
            }
        }

        count
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day14.txt").expect("File not found");

        let result = calculate_load(&contents);

        println!("Day 14 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day14_part1() {
            assert_eq!(calculate_load("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#...."), 136);
        }
    }
}

pub mod part2 {
    use super::*;
    use std::fs;

    fn rotate_matrix(matrix: &mut Vec<Vec<&str>>) {
        let n = matrix.len();

        for i in 0..n {
            for j in i..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        for row in matrix.iter_mut() {
            row.reverse();
        }
    }

    fn tilt<'a>(keys: Vec<&'a str>) -> Vec<&'a str> {
        let mut new_keys = vec!["."; keys.len()];
        for (i, key) in keys.iter().enumerate() {
            if *key == "O" {
                let new_idx = i - count_spaces_before(&keys, i as isize);
                new_keys[new_idx] = "O";
            } else if *key == "#" {
                new_keys[i] = "#";
            }
        }

        new_keys
    }

    fn cycle(matrix: &mut Vec<Vec<&str>>) {
        rotate_matrix(matrix);
        rotate_matrix(matrix);
        rotate_matrix(matrix);

        for i in 0..matrix.len() {
            matrix[i] = tilt(matrix[i].clone());
        }
        rotate_matrix(matrix);
        for i in 0..matrix.len() {
            matrix[i] = tilt(matrix[i].clone());
        }
        rotate_matrix(matrix);
        for i in 0..matrix.len() {
            matrix[i] = tilt(matrix[i].clone());
        }
        rotate_matrix(matrix);
        for i in 0..matrix.len() {
            matrix[i] = tilt(matrix[i].clone());
        }
        rotate_matrix(matrix);
        rotate_matrix(matrix);
        // rotate_matrix(matrix);
        // rotate_matrix(matrix);
        // rotate_matrix(matrix);
    }

    fn calculate_cycle_load(contents: &str) -> usize {
        let mut count = 0;
        let mut matrix: Vec<Vec<&str>> = contents
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|line: &&str| !line.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        for _ in 0..10000 {
            cycle(&mut matrix);
        }

        let mut cols = vec![];
        for y in 0..matrix[0].len() {
            let mut col: Vec<&str> = vec![];
            for x in 0..matrix.len() {
                col.push(matrix[x][y]);
            }
            cols.push(col);
        }

        for col in &cols {
            for (i, key) in col.iter().enumerate() {
                if *key == "O" {
                    count += col.len() - i;
                }
            }
        }

        count
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day14.txt").expect("File not found");

        let result = calculate_cycle_load(&contents);

        println!("Day 14 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day14_part2() {
            assert_eq!(calculate_cycle_load("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#...."), 64);
        }
    }
}
