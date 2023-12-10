use std::vec;

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn parse(lines: &str) -> Self {
        Self {
            tiles: lines
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    line.split("")
                        .filter(|line| !line.is_empty())
                        .enumerate()
                        .map(|(j, v)| Tile::parse(i, j, v))
                        .collect::<Vec<Tile>>()
                })
                .collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        if x > self.tiles.len() - 1 {
            return None;
        }

        let line = &self.tiles[x];
        if y > line.len() - 1 {
            return None;
        }

        Some(&line[y])
    }

    fn find_start(&self) -> (usize, usize) {
        for (i, line) in self.tiles.iter().enumerate() {
            for (j, tile) in line.iter().enumerate() {
                if tile.direction == TileDirection::Start {
                    return (i, j);
                }
            }
        }

        (0, 0)
    }

    fn next_tiles(&self, x: usize, y: usize, visited: &[(usize, usize, u64)]) -> Vec<&Tile> {
        let next_coordinates: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let tile = self.get(x, y).unwrap();
        let mut next_tiles: Vec<&Tile> = vec![];

        for (dx, dy) in next_coordinates {
            if (dx < 0 && x == 0) || (dy < 0 && y == 0) {
                continue;
            }

            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if visited.iter().any(|(x, y, _)| nx == *x && ny == *y) {
                continue;
            }

            if let Some(next_tile) = self.get(nx, ny) {
                if tile.is_valid_from(nx as isize, ny as isize)
                    && next_tile.is_valid_from(x as isize, y as isize)
                {
                    next_tiles.push(next_tile);
                }
            };
        }

        next_tiles
    }

    fn traverse_from_loop(&self, x: usize, y: usize) -> u64 {
        let mut count: u64 = 0;

        // Mark start tile as visited
        let mut visited: Vec<(usize, usize, u64)> = vec![(x, y, 0)];

        // Get valid next tile for starting node
        let mut next_tiles = self.next_tiles(x, y, &visited);

        while !next_tiles.is_empty() {
            count += 1;

            let mut new_next_tiles = vec![];

            // Mark all next tiles as visited
            for next_tile in &next_tiles {
                visited.push((next_tile.x, next_tile.y, count));
            }

            // Find all next tiles for iteration
            for next_tile in &next_tiles {
                new_next_tiles.extend(self.next_tiles(next_tile.x, next_tile.y, &visited));
            }

            next_tiles = new_next_tiles;
        }

        count
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TileDirection {
    Start,
    Ground,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
}

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    direction: TileDirection,
}

impl Tile {
    fn parse(x: usize, y: usize, value: &str) -> Self {
        Self {
            x,
            y,
            direction: match value {
                "S" => TileDirection::Start,
                "." => TileDirection::Ground,
                "|" => TileDirection::Vertical,
                "-" => TileDirection::Horizontal,
                "L" => TileDirection::BendNE,
                "J" => TileDirection::BendNW,
                "7" => TileDirection::BendSW,
                "F" => TileDirection::BendSE,
                _ => panic!("Incorrect tile direction {}", value),
            },
        }
    }

    fn is_valid_from(&self, prev_x: isize, prev_y: isize) -> bool {
        let curr_x = self.x as isize;
        let curr_y = self.y as isize;

        match self.direction {
            TileDirection::Vertical => {
                curr_y == prev_y && (curr_x == prev_x + 1 || curr_x == prev_x - 1)
            }
            TileDirection::Horizontal => {
                curr_x == prev_x && (curr_y == prev_y + 1 || curr_y == prev_y - 1)
            }
            TileDirection::BendNE => {
                (curr_x == prev_x + 1 && curr_y == prev_y)
                    || (curr_x == prev_x && curr_y == prev_y - 1)
            }
            TileDirection::BendNW => {
                (curr_x == prev_x && curr_y == prev_y + 1)
                    || (curr_x == prev_x + 1 && curr_y == prev_y)
            }
            TileDirection::BendSW => {
                (curr_x == prev_x && curr_y == prev_y + 1)
                    || (curr_x == prev_x - 1 && curr_y == prev_y)
            }
            TileDirection::BendSE => {
                (curr_x == prev_x && curr_y == prev_y - 1)
                    || (curr_x == prev_x - 1 && curr_y == prev_y)
            }
            TileDirection::Start => true,
            TileDirection::Ground => false,
        }
    }
}

pub mod part1 {
    use super::*;
    use std::fs;

    fn steps_to_furthest_tile(contents: &str) -> u64 {
        let map = Map::parse(contents);

        let (x, y) = map.find_start();

        map.traverse_from_loop(x, y)
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day10.txt").expect("File not found");

        let result = steps_to_furthest_tile(&contents);

        println!("Day 10 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day10_part1a() {
            assert_eq!(
                steps_to_furthest_tile(".....\n.S-7.\n.|.|.\n.L-J.\n....."),
                4
            );
        }

        #[test]
        fn test_day10_part1b() {
            assert_eq!(
                steps_to_furthest_tile(".....\n.F-7.\n.|.|.\n.L-S.\n....."),
                4
            );
        }

        #[test]
        fn test_day10_part1c() {
            assert_eq!(
                steps_to_furthest_tile(".....\n.F-S.\n.|.|.\n.L-J.\n....."),
                4
            );
        }

        #[test]
        fn test_day10_part1d() {
            assert_eq!(
                steps_to_furthest_tile(".....\n.F-7.\n.|.|.\n.S-J.\n....."),
                4
            );
        }

        #[test]
        fn test_day10_part1e() {
            assert_eq!(
                steps_to_furthest_tile("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ..."),
                8
            );
        }

        #[test]
        fn test_day10_part1f() {
            assert_eq!(
                steps_to_furthest_tile("7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ"),
                8
            );
        }

        #[test]
        fn test_day10_part1g() {
            assert_eq!(steps_to_furthest_tile("S-7\n|.|\nL-J"), 4);
        }

        #[test]
        fn test_day10_part1h() {
            assert_eq!(
                steps_to_furthest_tile("-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF"),
                4
            );
        }

        #[test]
        fn test_day10_part1i() {
            assert_eq!(
                steps_to_furthest_tile("-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF"),
                4
            );
        }
    }
}

pub mod part2 {
    use std::fs;

    fn do_stuff(contents: &str) -> i32 {
        contents.lines().count() as i32
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day10.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 10 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day10_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
