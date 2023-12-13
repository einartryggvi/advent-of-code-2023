fn is_perfect_reflection(start: usize, rows: &Vec<Vec<&str>>) -> bool {
    let mut left = start as isize;
    let mut right = start + 1;

    if right > rows.len() - 1 {
        return false;
    }

    while left >= 0 && right < rows.len() {
        if rows[left as usize] != rows[right] {
            return false;
        }

        left -= 1;
        right += 1;
    }

    true
}

pub mod part1 {
    use super::*;
    use std::fs;

    fn count_reflections(contents: &str) -> usize {
        let matrix: Vec<Vec<&str>> = contents
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|line: &&str| !line.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        let mut count = 0;

        for x in 0..matrix.len() {
            if is_perfect_reflection(x, &matrix) {
                count += (x + 1) * 100;
            }
        }

        let mut cols = vec![];
        for y in 0..matrix[0].len() {
            let mut col: Vec<&str> = vec![];
            for x in 0..matrix.len() {
                col.push(matrix[x][y]);
            }
            cols.push(col);
        }

        for y in 0..cols.len() {
            if is_perfect_reflection(y, &cols) {
                count += y + 1;
            }
        }

        if count == 0 {
            println!("---------------------------------");
            for row in &matrix {
                for col in row {
                    print!("{}", col);
                }
                println!();
            }
            println!("---------------------------------");
        }

        count as usize
    }

    fn count_total_reflections(contents: &str) -> usize {
        let mut count = 0;

        for part in contents.split("\n\n") {
            count += count_reflections(part);
        }

        count
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day13.txt").expect("File not found");

        let result = count_total_reflections(&contents);

        println!("Day 13 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day13_part1a() {
            assert_eq!(
                count_total_reflections(
                    "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#"
                ),
                405
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
        let contents = fs::read_to_string("inputs/day13.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 13 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day13_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
