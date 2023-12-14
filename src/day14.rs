pub mod part1 {
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
            for _ in 0..matrix.len() {
                let mut i = 0;
                while i < col.len() - 1 {
                    if col[i] == "." && col[i + 1] == "O" {
                        col[i] = "O";
                        col[i + 1] = ".";
                    }
                    i += 1;
                }
            }

            for (i, key) in col.iter().enumerate() {
                if *key == "O" {
                    count += col.len() - i;
                }
            }
            cols.push(col);
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
    use std::fs;

    fn do_stuff(contents: &str) -> i32 {
        contents.len() as i32
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day14.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 14 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day14_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
