use std::vec;

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn parse(lines: &str) -> Self {
        // Pad the graph with ground
        let line_length = lines.lines().next().map_or(0, |line| line.len() + 2); // +2 for the added '.' at the start and end
        let dot_line = ".".repeat(line_length);

        Self {
            tiles: std::iter::once(dot_line.clone())
                .chain(lines.lines().map(|line| format!(".{}.", line)))
                .chain(std::iter::once(dot_line))
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

    fn next_tiles(
        &self,
        x: usize,
        y: usize,
        visited: &[(usize, usize, u64)],
        skip_valid_check: bool,
    ) -> Vec<&Tile> {
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
                if skip_valid_check
                    || (tile.is_valid_from(nx as isize, ny as isize)
                        && next_tile.is_valid_from(x as isize, y as isize))
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
        let mut next_tiles = self.next_tiles(x, y, &visited, false);

        while !next_tiles.is_empty() {
            count += 1;

            let mut new_next_tiles = vec![];

            // Mark all next tiles as visited
            for next_tile in &next_tiles {
                visited.push((next_tile.x, next_tile.y, count));
            }

            // Find all next tiles for iteration
            for next_tile in &next_tiles {
                new_next_tiles.extend(self.next_tiles(next_tile.x, next_tile.y, &visited, false));
            }

            next_tiles = new_next_tiles;
        }

        count
    }

    fn get_loop_vertices(&self) -> Vec<(usize, usize)> {
        let mut vertices: Vec<(usize, usize)> = vec![];
        let mut visited: Vec<(usize, usize, u64)> = vec![];

        let start_node = self.find_start();
        let mut next_node = start_node;

        loop {
            vertices.push(next_node);
            visited.push((next_node.0, next_node.1, 0));

            let next_nodes = self.next_tiles(next_node.0, next_node.1, &visited, false);

            if next_nodes.is_empty() {
                break;
            }

            next_node = (next_nodes[0].x, next_nodes[0].y);
        }

        vertices
    }

    fn populate_enclosed_regions(&self, debug: bool) -> u64 {
        let anchor_points: Vec<(usize, usize)> = self.get_loop_vertices();

        let loop_tiles = anchor_points.iter().map(|(x, y)| self.get(*x, *y).unwrap());

        let mut tiles: Vec<Vec<String>> = self
            .tiles
            .iter()
            .map(|line| line.iter().map(|tile| tile.value.clone()).collect())
            .collect();

        if debug {
            println!("Raw tiles:");
            self.print(&tiles);
        }

        // Traverse the graph and assign a counter to each tile
        for (i, tile) in loop_tiles.clone().enumerate() {
            tiles[tile.x][tile.y] = (i + 1).to_string();
        }

        if debug {
            println!("Tiles with counter:");
            self.print(&tiles);
        }

        // Replace all non-loop tiles with ground (.)
        for (x, lines) in tiles.clone().iter().enumerate() {
            for (y, _) in lines.iter().enumerate() {
                if !loop_tiles.clone().any(|tile| tile.x == x && tile.y == y) {
                    tiles[x][y] = ".".to_string();
                }
            }
        }

        if debug {
            println!("Replace junk with ground:");
            self.print(&tiles);
        }

        // Expand the graph, add ground between all rows and colums
        let mut expanded_tiles: Vec<Vec<String>> = vec![];
        for x in 0..tiles.len() {
            let mut row: Vec<String> = vec![];
            for y in 0..tiles[x].len() {
                if let Ok(value) = tiles[x][y].parse::<u64>() {
                    row.push((value * 2).to_string());
                } else {
                    row.push(tiles[x][y].clone());
                }
                row.push(".".to_string());
            }

            let expanded_row_len = row.len();
            expanded_tiles.push(row);
            if x < tiles.len() - 1 {
                expanded_tiles.push((0..expanded_row_len).map(|_| ".".to_string()).collect());
            }
        }

        if debug {
            println!("Expanded tiles:");
            self.print(&expanded_tiles);
        }

        // Fill in missing tiles in the loop
        // let next_coordinates: [(isize, isize); 4] = [(-2, 0), (2, 0), (0, -2), (0, 2)];
        let mut max_val: i64 = 0;
        let mut min_val: i64 = 999999;
        for x in 0..expanded_tiles.len() {
            for y in 0..expanded_tiles[x].len() {
                if let Ok(val) = expanded_tiles[x][y].parse::<i64>() {
                    if val > max_val {
                        max_val = val;
                    }

                    if val < min_val {
                        min_val = val;
                    }
                }
                let left = if y > 0 {
                    expanded_tiles[x][y - 1].parse::<u64>().unwrap_or(0) as i64
                } else {
                    0
                };
                let right = if y < expanded_tiles[x].len() - 1 {
                    expanded_tiles[x][y + 1].parse::<u64>().unwrap_or(0) as i64
                } else {
                    0
                };
                let up = if x > 0 {
                    expanded_tiles[x - 1][y].parse::<u64>().unwrap_or(0) as i64
                } else {
                    0
                };
                let down = if x < expanded_tiles.len() - 1 {
                    expanded_tiles[x + 1][y].parse::<u64>().unwrap_or(0) as i64
                } else {
                    0
                };

                if left > 0 && right > 0 && (left - right == -2 || left - right == 2) {
                    expanded_tiles[x][y] = (left.min(right) + 1).to_string();
                } else if up > 0 && down > 0 && (up - down == -2 || up - down == 2) {
                    expanded_tiles[x][y] = (up.min(down) + 1).to_string();
                }
            }
        }

        let minxmax = min_val + max_val;
        for x in 0..expanded_tiles.len() {
            for y in 0..expanded_tiles[x].len() {
                let left = if y > 0 {
                    expanded_tiles[x][y - 1].parse::<u64>().unwrap_or(0) as i64
                } else {
                    0
                };
                let right = if y < expanded_tiles[x].len() - 1 {
                    expanded_tiles[x][y + 1].parse::<u64>().unwrap_or(0) as i64
                } else {
                    0
                };
                let up = if x > 0 {
                    expanded_tiles[x - 1][y].parse::<u64>().unwrap_or(0) as i64
                } else {
                    0
                };
                let down = if x < expanded_tiles.len() - 1 {
                    expanded_tiles[x + 1][y].parse::<u64>().unwrap_or(0) as i64
                } else {
                    0
                };

                if left + right == minxmax || up + down == minxmax {
                    expanded_tiles[x][y] = (min_val - 1).to_string();
                }
            }
        }

        if debug {
            println!("Expanded counted tiles:");
            self.print(&expanded_tiles);
        }

        // Traverse the graph and mark nodes outside the loop
        let mut next_nodes: Vec<(usize, usize)> = vec![(0, 0)];
        while let Some((x, y)) = next_nodes.pop() {
            let next_coordinates: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dx, dy) in next_coordinates {
                if (dx < 0 && x == 0) || (dy < 0 && y == 0) {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0
                    || ny < 0
                    || nx > expanded_tiles.len() as isize - 1
                    || ny > expanded_tiles[0].len() as isize - 1
                {
                    continue;
                }

                if expanded_tiles[nx as usize][ny as usize] == "." {
                    next_nodes.push((nx as usize, ny as usize));
                }
            }

            expanded_tiles[x][y] = "O".to_string();
        }

        if debug {
            println!("Outside tiles marked:");
            self.print(&expanded_tiles);
        }

        // Shrink graph back to original size
        let mut shrunk_tiles: Vec<Vec<String>> = vec![];
        for (x, rows) in expanded_tiles.iter().enumerate() {
            let mut shrunk_row = vec![];
            for (y, row) in rows.iter().enumerate() {
                if y % 2 == 0 {
                    shrunk_row.push(row.clone());
                }
            }

            if x % 2 == 0 {
                shrunk_tiles.push(shrunk_row);
            }
        }

        if debug {
            println!("Shrunk graph:");
            self.print(&shrunk_tiles);
        }

        let mut enclosed_points: Vec<(usize, usize)> = vec![];
        for (x, rows) in shrunk_tiles.iter().enumerate() {
            for (y, row) in rows.iter().enumerate() {
                if row == "." {
                    enclosed_points.push((x, y));
                }
            }
        }

        // Count all ground tiles that are left
        shrunk_tiles
            .iter()
            .flatten()
            .filter(|value| *value == ".")
            .count() as u64
    }

    fn print(&self, tiles: &Vec<Vec<String>>) {
        for line in tiles {
            for tile in line {
                print!("{:>4}", tile);
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Clone)]
struct Tile {
    x: usize,
    y: usize,
    value: String,
    direction: TileDirection,
}

impl Tile {
    fn parse(x: usize, y: usize, value: &str) -> Self {
        Self {
            x,
            y,
            value: value.to_string(),
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
    use super::*;
    use std::fs;

    fn count_enclosed_tiles(contents: &str, debug: bool) -> u64 {
        Map::parse(contents).populate_enclosed_regions(debug)
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day10.txt").expect("File not found");

        let result = count_enclosed_tiles(&contents, false);

        println!("Day 10 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day10_part2a() {
            assert_eq!(count_enclosed_tiles("...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........", true), 4);
        }

        #[test]
        fn test_day10_part2b() {
            assert_eq!(
                count_enclosed_tiles(
                    "..........\n.S------7.\n.|F----7|.\n.||....||.\n.||....||.\n.|L-7F-J|.\n.|..||..|.\n.L--JL--J.\n..........", true
                ),
                4
            );
        }

        #[test]
        fn test_day10_part2c() {
            assert_eq!(
                count_enclosed_tiles(
                    ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ..."
                ,true),
                8
            );
        }

        #[test]
        fn test_day10_part2d() {
            assert_eq!(
                count_enclosed_tiles(
                    "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L"
                ,true),
                10
            );
        }
    }
}
