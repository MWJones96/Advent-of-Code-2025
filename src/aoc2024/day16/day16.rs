use std::collections::HashMap;
use std::u64;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_vector(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
            Direction::East => (row, col + 1),
        }
    }

    fn get_next(&self) -> [Direction; 2] {
        match self {
            Direction::North => [Direction::East, Direction::West],
            Direction::South => [Direction::East, Direction::West],
            Direction::West => [Direction::North, Direction::South],
            Direction::East => [Direction::North, Direction::South],
        }
    }
}

fn part1() {
    fn dfs(
        seen: &mut HashMap<(usize, usize), u64>,
        grid: &Vec<Vec<char>>,
        row: usize,
        col: usize,
        dir: Direction,
        curr_score: u64,
    ) -> u64 {
        if grid[row][col] == '#' {
            return u64::MAX;
        }
        if grid[row][col] == 'E' {
            return curr_score;
        }
        if curr_score >= *seen.entry((row, col)).or_insert(u64::MAX) {
            return u64::MAX;
        }

        *seen.get_mut(&(row, col)).unwrap() = curr_score;

        let sides = dir.get_next().map(|d| (d.get_vector(row, col), d));

        let (f_row, f_col) = dir.get_vector(row, col);
        let mut min = dfs(seen, grid, f_row, f_col, dir, curr_score + 1);
        for (turn_coords, turn_dir) in sides {
            min = min.min(dfs(
                seen,
                grid,
                turn_coords.0,
                turn_coords.1,
                turn_dir,
                curr_score + 1001,
            ));
        }

        min
    }

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

    println!(
        "(Part 1) Shortest path to end: {}",
        dfs(
            &mut HashMap::new(),
            &grid,
            start_idx.0,
            start_idx.1,
            Direction::East,
            0
        )
    );
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(Part 2) Part 2 not implemented yet");
}

pub fn day16() {
    println!("---- DAY 16 ----");
    part1();
    part2();
    println!("");
}
