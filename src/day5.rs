struct AlmanacEntry {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

fn find_section(contents: &str, label: &str) -> Vec<AlmanacEntry> {
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
            .map(|s| {
                let parts: Vec<&str> = s.split_whitespace().collect();
                let dest_start = parts[0].parse::<u64>().unwrap();
                let source_start = parts[1].parse::<u64>().unwrap();
                let length = parts[2].parse::<u64>().unwrap();

                AlmanacEntry {
                    dest_start,
                    source_start,
                    length,
                }
            })
            .collect(),
        _ => vec![],
    }
}

fn find_dest(section: &Vec<AlmanacEntry>, source: u64) -> u64 {
    for entry in section {
        if source < entry.source_start || source > entry.source_start + entry.length {
            continue;
        }

        return entry.dest_start + source - entry.source_start;
    }

    source
}

fn find_source(section: &Vec<AlmanacEntry>, dest: u64) -> u64 {
    for entry in section {
        if dest < entry.dest_start || dest >= entry.dest_start + entry.length {
            continue;
        }

        return entry.source_start + dest - entry.dest_start;
    }

    dest
}

pub mod part1 {
    use super::*;
    use std::fs;

    fn seed_locations(contents: &str) -> u64 {
        let seeds_line: Vec<&str> = contents.lines().take(1).collect();
        let seeds_part: Vec<&str> = seeds_line[0].split(": ").collect();
        let seeds: Vec<u64> = seeds_part[1]
            .split_whitespace()
            .map(|seed| seed.parse::<u64>().unwrap())
            .collect();

        let seed_to_soil = &find_section(contents, "seed-to-soil");
        let soil_to_fertilizer = &find_section(contents, "soil-to-fertilizer");
        let fertilizer_to_water = &find_section(contents, "fertilizer-to-water");
        let water_to_light = &find_section(contents, "water-to-light");
        let light_to_tempature = &find_section(contents, "light-to-temperature");
        let tempature_to_humidity = &find_section(contents, "temperature-to-humidity");
        let humidity_to_location = &find_section(contents, "humidity-to-location");

        let mut min_location = std::u64::MAX;
        let mut min_seed: u64 = 0;
        for seed in seeds {
            let soil = find_dest(seed_to_soil, seed);
            let fertilizer = find_dest(soil_to_fertilizer, soil);
            let water = find_dest(fertilizer_to_water, fertilizer);
            let light = find_dest(water_to_light, water);
            let tempature = find_dest(light_to_tempature, light);
            let humidity = find_dest(tempature_to_humidity, tempature);
            let location = find_dest(humidity_to_location, humidity);

            if location < min_location {
                min_location = location;
                min_seed = seed;
            }
        }

        println!("location = {}, seed = {}", min_location, min_seed);
        min_location
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
pub mod part2 {
    use super::*;
    use std::{fs, vec};

    fn seed_range_locations(contents: &str) -> u64 {
        let seeds_line: Vec<&str> = contents.lines().take(1).collect();
        let seeds_part: Vec<&str> = seeds_line[0].split(": ").collect();
        let seed_ranges: Vec<u64> = seeds_part[1]
            .split_whitespace()
            .map(|seed| seed.parse::<u64>().unwrap())
            .collect();

        let mut seed_tuples: Vec<(u64, u64)> = vec![];

        for i in (0..seed_ranges.len()).step_by(2) {
            seed_tuples.push((seed_ranges[i], seed_ranges[i + 1]))
        }

        let seed_to_soil = &find_section(contents, "seed-to-soil");
        let soil_to_fertilizer = &find_section(contents, "soil-to-fertilizer");
        let fertilizer_to_water = &find_section(contents, "fertilizer-to-water");
        let water_to_light = &find_section(contents, "water-to-light");
        let light_to_tempature = &find_section(contents, "light-to-temperature");
        let tempature_to_humidity = &find_section(contents, "temperature-to-humidity");
        let humidity_to_location = &find_section(contents, "humidity-to-location");

        let mut location: u64 = 0;
        loop {
            let humidity = find_source(humidity_to_location, location);
            let tempature = find_source(tempature_to_humidity, humidity);
            let light = find_source(light_to_tempature, tempature);
            let water = find_source(water_to_light, light);
            let fertilizer = find_source(fertilizer_to_water, water);
            let soil = find_source(soil_to_fertilizer, fertilizer);
            let seed = find_source(seed_to_soil, soil);

            for (start, length) in &seed_tuples {
                let range_start = start.clone();
                let range_end = range_start + length.clone();
                if seed >= range_start && seed < range_end {
                    return location;
                }
            }

            location += 1;
        }
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day5.txt").expect("File not found");

        let result = seed_range_locations(&contents);

        println!("Day 5 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_seed_range_locations() {
            assert_eq!(seed_range_locations("seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4"), 46);
        }
    }
}
