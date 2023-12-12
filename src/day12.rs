pub mod part1 {
    use std::fs;

    use regex::Regex;

    fn generate_combos(chars: Vec<&str>) -> Vec<Vec<&str>> {
        let mut combos: Vec<Vec<&str>> = vec![];
        if let Some(next_unknown) = chars.iter().position(|&c| c == "?") {
            let mut as_broken = chars.clone();
            as_broken[next_unknown] = "#";
            combos.push(as_broken.clone());

            let mut as_working = chars.clone();
            as_working[next_unknown] = ".";
            combos.push(as_working.clone());

            combos.extend(generate_combos(as_broken));
            combos.extend(generate_combos(as_working));
        }

        combos
            .iter()
            .filter(|chars| !chars.iter().any(|&c| c == "?"))
            .cloned() // This clones each Vec<&str> from &Vec<&str>
            .collect::<Vec<Vec<&str>>>()
    }

    fn is_valid(combo: &str, num_broken: &Vec<usize>) -> bool {
        let re = Regex::new(r"(#+)").unwrap();
        let groups: Vec<&str> = re.find_iter(combo).map(|m| m.as_str()).collect();

        if groups.len() != num_broken.len() {
            return false;
        }

        for (i, group) in groups.iter().enumerate() {
            if i > num_broken.len() - 1 {
                return false;
            }

            if group.len() != num_broken[i] {
                return false;
            }
        }

        true
    }

    fn count_arrangements(line: &str) -> usize {
        let (p1, p2) = line.split_once(' ').unwrap();

        let chars: Vec<&str> = p1.split("").filter(|s| !s.is_empty()).collect();
        let num_broken: Vec<usize> = p2
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let mut count = 0;
        let combos = generate_combos(chars);
        for combo in combos {
            if is_valid(combo.join("").as_str(), &num_broken) {
                count += 1;
            }
        }

        count
    }

    fn sum_arrangements(contents: &str) -> usize {
        let mut count = 0;
        for line in contents.lines() {
            count += count_arrangements(line);
        }

        count
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day12.txt").expect("File not found");

        let result = sum_arrangements(&contents);
        println!("Day 12 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day12_part1a() {
            assert_eq!(count_arrangements("???.### 1,1,3"), 1);
        }

        #[test]
        fn test_day12_part1b() {
            assert_eq!(count_arrangements(".??..??...?##. 1,1,3"), 4);
        }

        #[test]
        fn test_day12_part1c() {
            assert_eq!(count_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        }

        #[test]
        fn test_day12_part1d() {
            assert_eq!(count_arrangements("????.#...#... 4,1,1"), 1);
        }

        #[test]
        fn test_day12_part1e() {
            assert_eq!(count_arrangements("????.######..#####. 1,6,5"), 4);
        }

        #[test]
        fn test_day12_part1f() {
            assert_eq!(count_arrangements("?###???????? 3,2,1"), 10);
        }
    }
}

pub mod part2 {
    use std::fs;

    fn do_stuff(contents: &str) -> i32 {
        contents.len() as i32
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day12.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 12 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day12_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
