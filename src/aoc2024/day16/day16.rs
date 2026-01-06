use std::collections::{HashMap, HashSet};
use std::u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_next_step_forward(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
            Direction::East => (row, col + 1),
        }
    }

    fn get_turn_directions(&self, row: usize, col: usize) -> [((usize, usize), Direction); 2] {
        match self {
            Direction::North => [
                (
                    Direction::East.get_next_step_forward(row, col),
                    Direction::East,
                ),
                (
                    Direction::West.get_next_step_forward(row, col),
                    Direction::West,
                ),
            ],
            Direction::South => [
                (
                    Direction::East.get_next_step_forward(row, col),
                    Direction::East,
                ),
                (
                    Direction::West.get_next_step_forward(row, col),
                    Direction::West,
                ),
            ],
            Direction::West => [
                (
                    Direction::North.get_next_step_forward(row, col),
                    Direction::North,
                ),
                (
                    Direction::South.get_next_step_forward(row, col),
                    Direction::South,
                ),
            ],
            Direction::East => [
                (
                    Direction::North.get_next_step_forward(row, col),
                    Direction::North,
                ),
                (
                    Direction::South.get_next_step_forward(row, col),
                    Direction::South,
                ),
            ],
        }
    }
}

fn dfs(
    seen: &mut HashMap<(usize, usize, Direction), u64>,
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    dir: Direction,
    curr_score: u64,
    best_so_far: &mut u64,
) -> u64 {
    if curr_score > *best_so_far {
        return u64::MAX;
    }
    if grid[row][col] == '#' {
        return u64::MAX;
    }
    if grid[row][col] == 'E' {
        *best_so_far = curr_score.min(*best_so_far);
        return curr_score;
    }
    if seen.contains_key(&(row, col, dir)) {
        let v = seen.get(&(row, col, dir)).unwrap();
        if curr_score > *v {
            return u64::MAX;
        }
        seen.insert((row, col, dir), curr_score.min(*v));
    } else {
        seen.insert((row, col, dir), curr_score);
    }

    let turns: [((usize, usize), Direction); 2] = dir.get_turn_directions(row, col);

    let (f_row, f_col) = dir.get_next_step_forward(row, col);
    let mut min = dfs(seen, grid, f_row, f_col, dir, curr_score + 1, best_so_far);
    for (turn_coords, turn_dir) in turns {
        min = min.min(dfs(
            seen,
            grid,
            turn_coords.0,
            turn_coords.1,
            turn_dir,
            curr_score + 1001,
            best_so_far,
        ));
    }

    min
}

fn dfs2(
    seen: &mut HashMap<(usize, usize, Direction), u64>,
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    dir: Direction,
    curr_score: u64,
    best_score: u64,
    best_so_far: &mut u64,
    best_tiles: &mut HashSet<(usize, usize)>,
) -> u64 {
    if curr_score > *best_so_far {
        return u64::MAX;
    }
    if grid[row][col] == '#' {
        return u64::MAX;
    }
    if grid[row][col] == 'E' {
        *best_so_far = curr_score.min(*best_so_far);
        best_tiles.insert((row, col));
        return curr_score;
    }
    if seen.contains_key(&(row, col, dir)) {
        let v = seen.get(&(row, col, dir)).unwrap();
        if curr_score > *v {
            return u64::MAX;
        }
        seen.insert((row, col, dir), curr_score.min(*v));
    } else {
        seen.insert((row, col, dir), curr_score);
    }

    let turns: [((usize, usize), Direction); 2] = dir.get_turn_directions(row, col);

    let (f_row, f_col) = dir.get_next_step_forward(row, col);
    let mut min = dfs2(
        seen,
        grid,
        f_row,
        f_col,
        dir,
        curr_score + 1,
        best_score,
        best_so_far,
        best_tiles,
    );
    for (turn_coords, turn_dir) in turns {
        min = min.min(dfs2(
            seen,
            grid,
            turn_coords.0,
            turn_coords.1,
            turn_dir,
            curr_score + 1001,
            best_score,
            best_so_far,
            best_tiles,
        ));
    }

    if min == best_score {
        best_tiles.insert((row, col));
    }

    min
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

    let mut best_so_far = u64::MAX;
    let best_score = dfs(
        &mut HashMap::new(),
        &grid,
        start_idx.0,
        start_idx.1,
        Direction::East,
        0,
        &mut best_so_far,
    );

    println!("(Part 1) Shortest path to end: {}", best_score);
}

fn part2() {
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

    let mut best_so_far = u64::MAX;
    let best_score = dfs(
        &mut HashMap::new(),
        &grid,
        start_idx.0,
        start_idx.1,
        Direction::East,
        0,
        &mut best_so_far,
    );

    let mut best_tiles = HashSet::new();
    dfs2(
        &mut HashMap::new(),
        &grid,
        start_idx.0,
        start_idx.1,
        Direction::East,
        0,
        best_score,
        &mut best_so_far,
        &mut best_tiles,
    );
    println!("(Part 2) Number of best path tiles: {}", best_tiles.len());
}

pub fn day16() {
    println!("---- DAY 16 ----");
    part1();
    part2();
    println!("");
}
