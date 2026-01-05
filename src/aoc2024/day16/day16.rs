use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }

    fn get_next(&self) -> (Direction, Direction) {
        match self {
            Direction::North => (Direction::East, Direction::West),
            Direction::South => (Direction::East, Direction::West),
            Direction::West => (Direction::North, Direction::South),
            Direction::East => (Direction::North, Direction::South),
        }
    }
}

fn part1() {
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start_idx = (0, 0);
    for (row, line) in input.lines().enumerate() {
        let mut grid_line = vec![];
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_idx = (row, col);
            }
            grid_line.push(ch);
        }
        grid.push(grid_line);
    }

    let mut end_score = 0;
    let mut frontier: VecDeque<(usize, usize, Direction, u64)> = VecDeque::new();
    frontier.push_back((start_idx.0, start_idx.1, Direction::East, 0));
    'outer: while !frontier.is_empty() {
        let frontier_len = frontier.len();
        for _ in 0..frontier_len {
            let (row, col, dir, score) = frontier.pop_front().unwrap();
            let mut next_idx = (row, col);
            let mut steps = 0;
            while grid[next_idx.0][next_idx.1] != '#' {
                if grid[next_idx.0][next_idx.1] == 'E' {
                    end_score = score + steps;
                    break 'outer;
                }
                if grid[next_idx.0][next_idx.1] == '*' {
                    break;
                }
                grid[next_idx.0][next_idx.1] = '*';

                let sides = dir.get_next();
                let sides = [sides.0, sides.1].map(|d| (d.get_vector(), d));
                let sides = [
                    (
                        (
                            (next_idx.0 as isize + sides[0].0.0) as usize,
                            (next_idx.1 as isize + sides[0].0.1) as usize,
                        ),
                        sides[0].1,
                    ),
                    (
                        (
                            (next_idx.0 as isize + sides[1].0.0) as usize,
                            (next_idx.1 as isize + sides[1].0.1) as usize,
                        ),
                        sides[1].1,
                    ),
                ];

                for (idx, dir) in sides {
                    if grid[idx.0][idx.1] == '.' {
                        frontier.push_back((idx.0, idx.1, dir, score + 1000 + steps + 1));
                    }
                }

                let next_dir = dir.get_vector();
                next_idx = (
                    (next_idx.0 as isize + next_dir.0) as usize,
                    (next_idx.1 as isize + next_dir.1) as usize,
                );
                steps += 1;
            }
        }
    }

    println!("(Part 1) Shortest path to end: {}", end_score);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 16) Part 2 not implemented yet");
}

pub fn day16() {
    println!("---- DAY 16 ----");
    part1();
    part2();
    println!("");
}
