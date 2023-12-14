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
        fn test_day13_part1() {
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

    use super::is_perfect_reflection;

    fn diff(vec1: &Vec<&str>, vec2: &Vec<&str>) -> usize {
        let mut count = 0;
        for (v1, v2) in vec1.iter().zip(vec2.iter()) {
            if v1 != v2 {
                count += 1;
            }
        }

        count
    }

    fn is_perfect_reflection_with_smudge(
        start: usize,
        rows: &mut Vec<Vec<&str>>,
        mut smudge_found: bool,
    ) -> (bool, bool) {
        let mut left = start as isize;
        let mut right = start + 1;

        if right > rows.len() - 1 {
            return (false, false);
        }

        let mut diff_idx = -1;

        while left >= 0 && right < rows.len() {
            let diff = diff(&rows[left as usize], &rows[right]);

            if rows[left as usize] != rows[right] && (diff != 1 || smudge_found) {
                return (false, false);
            }

            if diff == 1 {
                smudge_found = true;
            }

            if smudge_found && diff_idx < 0 {
                diff_idx = left;
            }
            left -= 1;
            right += 1;
        }

        if diff_idx >= 0 {
            rows[diff_idx as usize] = rows[diff_idx as usize + 1].clone();
        }

        (true, smudge_found)
    }

    fn count_reflections(contents: &str) -> usize {
        let mut matrix: Vec<Vec<&str>> = contents
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split("")
                    .filter(|line: &&str| !line.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect();

        let mut count = 0;
        let mut smudge_found = false;
        for x in 0..matrix.len() {
            if is_perfect_reflection(x, &matrix) {
                continue;
            }
            let (perfect, smudge) = is_perfect_reflection_with_smudge(x, &mut matrix, false);
            if smudge {
                smudge_found = true;
            }
            if perfect {
                count += (x + 1) * 100;
                break;
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
                continue;
            }
            let (perfect, smudge) = is_perfect_reflection_with_smudge(y, &mut cols, smudge_found);
            if smudge {
                smudge_found = true;
            }
            if perfect {
                count += y + 1;
                break;
            }
        }

        count
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

        println!("Day 13 Part 2: {}", result);
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day13_part2() {
            assert_eq!(
            count_total_reflections(
                "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#"
            ),
            400
        );
        }
    }
}
