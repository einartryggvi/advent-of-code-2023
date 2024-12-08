pub mod part1 {
    use std::{
        cmp::{self, Ordering},
        collections::{HashMap, HashSet, VecDeque},
        fs, result,
    };

    #[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn next_direction(x: usize, y: usize, nx: usize, ny: usize, direction: Direction) -> Direction {
        if direction == Direction::Left || direction == Direction::Right {
            if nx > x {
                return Direction::Down;
            } else if nx < x {
                return Direction::Up;
            }
        } else if direction == Direction::Up || direction == Direction::Down {
            if ny > y {
                return Direction::Right;
            } else if ny < y {
                return Direction::Left;
            }
        }

        direction
    }

    fn next_cells(
        matrix: &Vec<Vec<usize>>,
        x: usize,
        y: usize,
        direction: Direction,
        visited: &[(usize, usize)],
        prev_cost: usize,
    ) -> VecDeque<(usize, usize, usize, Direction)> {
        let next_coordinates: Vec<(isize, isize)> = if direction == Direction::Left {
            vec![(-1, 0), (1, 0), (0, -1)]
        } else if direction == Direction::Right {
            vec![(-1, 0), (1, 0), (0, 1)]
        } else if direction == Direction::Up {
            vec![(-1, 0), (0, -1), (0, 1)]
        } else if direction == Direction::Down {
            vec![(1, 0), (0, -1), (0, 1)]
        } else {
            panic!("Incorrect direction")
        };

        next_coordinates
            .iter()
            .map(|(dx, dy)| {
                if (*dx < 0 && x == 0) || (*dy < 0 && y == 0) {
                    return (-1, -1);
                }

                (dx + x as isize, dy + y as isize)
            })
            .filter(|(nx, ny)| {
                *nx >= 0
                    && *ny >= 0
                    && (*nx as usize) < matrix.len()
                    && (*ny as usize) < matrix[0].len()
                    && !visited
                        .iter()
                        .any(|(x, y)| (*nx as usize == *x && (*ny as usize) == *y))
            })
            .map(|(nx, ny)| {
                (
                    nx as usize,
                    ny as usize,
                    // prev_cost + matrix[nx as usize][ny as usize],
                    prev_cost + 1,
                    next_direction(x, y, nx as usize, ny as usize, direction),
                )
            })
            .collect::<VecDeque<(usize, usize, usize, Direction)>>()
    }

    fn traverse_matrix(
        matrix: &Vec<Vec<usize>>,
        results: &mut Vec<usize>,
        mut current_cost: Vec<usize>,
        x: usize,
        y: usize,
        direction: Direction,
        direction_count: usize,
        mut visited: HashMap<(usize, usize), bool>,
    ) {
        // Mark start tile as visited
        let mut visited: Vec<(usize, usize)> = vec![(x, y)];

        let mut dir_count: HashMap<Direction, usize> = HashMap::new();

        // Get valid next tile for starting node
        let mut next_tiles = VecDeque::new();
        next_tiles.push_back((x, y, 0, direction));

        while !next_tiles.is_empty() {
            let (nx, ny, ncost, nd) = next_tiles.pop_front().unwrap();

            visited.push((nx, ny));

            if nx == matrix.len() - 1 && ny == matrix[0].len() - 1 {
                results.push(ncost);
                continue;
            }

            // Find all next tiles for iteration
            for n in next_cells(matrix, nx, ny, nd, &visited, ncost) {
                // if *dir_count.get(&nd).unwrap_or(&0) == 3 {
                //     dir_count.remove(&nd);
                //     continue;
                // }

                dir_count
                    .entry(nd)
                    .and_modify(|count| *count = *count + 1)
                    .or_insert(1);
                next_tiles.push_back(n);
            }
        }

        for x in 0..matrix.len() {
            for y in 0..matrix[0].len() {
                if visited.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn dijsktra(matrix: &Vec<Vec<usize>>) -> usize {
        let mut unvisited: HashSet<(isize, isize, Direction)> = HashSet::new();
        let mut dist: HashMap<(isize, isize, Direction), usize> = HashMap::new();

        for x in 0..matrix.len() {
            for y in 0..matrix[0].len() {
                for dir in [
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                ] {
                    unvisited.insert((x as isize, y as isize, dir));
                    dist.insert((x as isize, y as isize, dir), usize::MAX);
                }
            }
        }

        dist.entry((0, 0, Direction::Left)).and_modify(|v| *v = 0);
        dist.entry((0, 0, Direction::Right)).and_modify(|v| *v = 0);
        dist.entry((0, 0, Direction::Up)).and_modify(|v| *v = 0);
        dist.entry((0, 0, Direction::Down)).and_modify(|v| *v = 0);

        loop {
            if let Some((x, y, prev_dir)) = dist
                .iter()
                .filter(|((x, y, d), _)| unvisited.contains(&(*x, *y, *d)))
                .min_by_key(|entry| entry.1)
                .map(|(key, _value)| *key)
            {
                for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    let nx = dx + x;
                    let ny = dy + y;
                    if nx < 0
                        || ny < 0
                        || nx > matrix.len() as isize - 1
                        || ny > matrix[0].len() as isize - 1
                    {
                        continue;
                    }

                    let dir =
                        next_direction(x as usize, y as usize, nx as usize, ny as usize, prev_dir);
                    let prev_dist = *dist.get(&(x, y, prev_dir)).unwrap();
                    let curr_dist = *dist.get(&(nx, ny, dir)).unwrap();
                    dist.entry((nx, ny, dir)).and_modify(|d| {
                        *d = curr_dist.min(prev_dist + matrix[nx as usize][ny as usize])
                    });
                }

                unvisited.remove(&(x, y, prev_dir));
            } else {
                break;
            }
        }

        for dir in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            for x in 0..matrix.len() {
                for y in 0..matrix[0].len() {
                    print!("{}\t", dist.get(&(x as isize, y as isize, dir)).unwrap());
                }
                println!();
            }
            println!("-------------------------");
        }

        let (mut x, mut y) = (0, 0);
        let mut dir = Direction::Right;
        let mut dir_count = 0;
        let mut visited = vec![(0, 0)];
        let mut total = 0;
        loop {
            let mut cost = usize::MAX;
            let mut candidate = (0, 0, Direction::Up);

            let next = next_cells(&matrix, x, y, dir, &visited, 0);
            if next.len() == 0 {
                break;
            }

            for (nx, ny, _, nd) in next {
                visited.push((nx, ny));
                println!("{}", dist.get(&(nx as isize, ny as isize, nd)).unwrap());
                if dist.get(&(nx as isize, ny as isize, nd)).unwrap() < &cost {
                    cost = *dist.get(&(nx as isize, ny as isize, nd)).unwrap();
                    candidate = (nx as isize, ny as isize, nd);
                }
            }

            x = candidate.0 as usize;
            y = candidate.1 as usize;

            total += matrix[x][y];

            if x == matrix.len() - 1 && y == matrix[0].len() - 1 {
                break;
            }
        }

        total

        // *dist
        //     .get(&(
        //         matrix.len() as isize - 1,
        //         matrix[0].len() as isize - 1,
        //         Direction::Right,
        //     ))
        //     .unwrap_or(&0)
    }

    fn lookahead(matrix: &Vec<Vec<usize>>) -> usize {
        let mut visited = vec![];
        let (mut x, mut y, mut d) = (matrix.len() - 1, matrix[0].len() - 1, Direction::Up);

        let mut total = 0;
        let mut dir_count = 0;
        loop {
            let mut cost = usize::MAX;
            let mut candidate = (0, 0, Direction::Up);

            let next = next_cells(&matrix, x, y, d, &visited, 0);
            if next.len() == 0 {
                break;
            }

            for (nx, ny, _, nd) in next {
                visited.push((nx, ny));
                if nd == d {
                    dir_count += 1;
                } else {
                    dir_count = 0;
                }
                d = nd;
                if dir_count <= 3 && matrix[nx][ny] < cost {
                    cost = matrix[nx][ny];
                    candidate = (nx as isize, ny as isize, nd);
                }
            }

            x = candidate.0 as usize;
            y = candidate.1 as usize;

            total += matrix[x][y];

            if x == matrix.len() - 1 && y == matrix[0].len() - 1 {
                break;
            }
        }
        total
    }

    fn recursive(matrix: &Vec<Vec<usize>>, x: isize, y: isize) -> usize {
        println!("{} {}", x, y);
        if x < 0 || y < 0 || x > matrix.len() as isize - 1 || y > matrix[0].len() as isize - 1 {
            return 0;
        }

        if x == matrix.len() as isize - 1 && y == matrix[0].len() as isize - 1 {
            return matrix[x as usize][y as usize];
        }

        matrix[x as usize][y as usize] + recursive(matrix, x + 1, y)
    }

    fn rev_minmize(matrix: &Vec<Vec<usize>>) -> usize {
        let mut visited = vec![];
        let (mut x, mut y, mut d) = (matrix.len() - 1, matrix[0].len() - 1, Direction::Up);

        let mut total = 0;
        let mut dir_count = 0;
        loop {
            visited.push((x, y));

            let mut cost = usize::MAX;
            let mut candidate = (0, 0, Direction::Left);

            let next = next_cells(&matrix, x, y, d, &visited, 0);
            if next.len() == 0 {
                break;
            }

            if x == 12 && y == 11 {
                println!("{:?}", next);
            }

            for (nx, ny, _, nd) in next {
                println!(
                    "({},{}) => ({},{}) candidate => {}",
                    x, y, nx as usize, ny as usize, matrix[nx as usize][ny as usize]
                );
                if (nd != d || dir_count <= 3) && matrix[nx][ny] < cost {
                    cost = matrix[nx][ny];
                    candidate = (nx as isize, ny as isize, nd);
                }
            }

            println!(
                "({},{}) => ({},{}) selected => {}",
                x,
                y,
                candidate.0 as usize,
                candidate.1 as usize,
                matrix[candidate.0 as usize][candidate.1 as usize]
            );
            println!("---------------------------------");

            x = candidate.0 as usize;
            y = candidate.1 as usize;

            if candidate.2 == d {
                dir_count += 1;
            } else {
                dir_count = 0;
            }
            d = candidate.2;

            if x == 0 && y == 0 {
                break;
            }

            total += matrix[x][y];
        }

        total
    }

    fn min_cost_path(contents: &str) -> usize {
        let matrix: Vec<Vec<usize>> = contents
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|line: &&str| !line.is_empty())
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();

        // let mut results: Vec<usize> = vec![];

        // traverse_matrix(
        //     &matrix,
        //     &mut results,
        //     vec![],
        //     0,
        //     0,
        //     Direction::Right,
        //     0,
        //     HashMap::new(),
        // );

        // println!("{:?}", results);

        // dijsktra(&matrix)
        recursive(&matrix, 0, 0)
        // lookahead(&matrix)
        // rev_minmize(&matrix)

        // 392 -- too low
        // *results.iter().min().unwrap()
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day17.txt").expect("File not found");

        let result = min_cost_path(&contents);

        println!("Day 17 Part 1: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day17_part1() {
            assert_eq!(min_cost_path("2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533"), 102);
        }
    }
}

pub mod part2 {
    use std::fs;

    fn do_stuff(contents: &str) -> i32 {
        contents.len() as i32
    }

    pub fn run() {
        let contents = fs::read_to_string("inputs/day17.txt").expect("File not found");

        let result = do_stuff(&contents);

        println!("Day 17 Part 2: {}", result);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day17_part2() {
            assert_eq!(do_stuff(""), 0);
        }
    }
}
