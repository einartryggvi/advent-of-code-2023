pub mod part1 {
    use std::fs;

    fn find_section(contents: &str, label: &str) -> Vec<String> {
        let mut start_idx: Option<usize> = None;
        let mut end_idx: Option<usize> = None;

        let lines = contents.lines();
        let max = lines.clone().count() - 1;
        let first_line = format!("{} map:", label);
        for (i, line) in lines.enumerate() {
            if line == first_line {
                start_idx = Some(i + 1);
            } else if i == max {
                end_idx = Some(i + 1);
                break;
            } else if line.is_empty() && start_idx.is_some() && end_idx.is_none() {
                end_idx = Some(i);
                break;
            }
        }

        match (start_idx, end_idx) {
            (Some(start), Some(end)) => contents
                .lines()
                .skip(start)
                .take(end - start)
                .map(|s| s.to_string())
                .collect(),
            _ => vec![],
        }
    }

    fn find_dest(section: Vec<String>, source: u64) -> u64 {
        for line in section {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let dest_start = parts[0].parse::<u64>().unwrap();
            let source_start = parts[1].parse::<u64>().unwrap();
            let length = parts[2].parse::<u64>().unwrap();

            if source < source_start || source > source_start + length {
                continue;
            }

            return dest_start + source - source_start;
        }

        source
    }

    fn seed_locations(contents: &str) -> u64 {
        let seeds_line: Vec<&str> = contents.lines().take(1).collect();
        let seeds_part: Vec<&str> = seeds_line[0].split(": ").collect();
        let seeds: Vec<u64> = seeds_part[1]
            .split_whitespace()
            .map(|seed| seed.parse::<u64>().unwrap())
            .collect();

        let mut locations: Vec<u64> = vec![];

        for seed in seeds {
            let soil = find_dest(find_section(contents, "seed-to-soil"), seed);
            let fertilizer = find_dest(find_section(contents, "soil-to-fertilizer"), soil);
            let water = find_dest(find_section(contents, "fertilizer-to-water"), fertilizer);
            let light = find_dest(find_section(contents, "water-to-light"), water);
            let tempature = find_dest(find_section(contents, "light-to-temperature"), light);
            let humidity = find_dest(find_section(contents, "temperature-to-humidity"), tempature);
            let location = find_dest(find_section(contents, "humidity-to-location"), humidity);

            locations.push(location.clone());
        }

        locations.into_iter().min().unwrap()
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day5.txt").expect("File not found");

        let result = seed_locations(&contents);

        println!("Day 5 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_seed_locations() {
            assert_eq!(seed_locations("seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4"), 35);
        }
    }
}
