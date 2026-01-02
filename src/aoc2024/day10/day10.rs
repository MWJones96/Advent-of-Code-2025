use std::collections::HashSet;

fn dfs2(grid: &Vec<Vec<u8>>, row: i8, col: i8, curr: u8) -> u64 {
    if row < 0 || row >= grid.len() as i8 {
        return 0;
    }
    if col < 0 || col >= grid[0].len() as i8 {
        return 0;
    }
    if grid[row as usize][col as usize] != curr {
        return 0;
    }

    if curr == 9 {
        return 1;
    }

    return dfs2(grid, row + 1, col, curr + 1)
        + dfs2(grid, row, col + 1, curr + 1)
        + dfs2(grid, row - 1, col, curr + 1)
        + dfs2(grid, row, col - 1, curr + 1);
}

fn dfs(seen: &mut HashSet<(u8, u8)>, grid: &Vec<Vec<u8>>, row: i8, col: i8, curr: u8) {
    if row < 0 || row >= grid.len() as i8 {
        return;
    }
    if col < 0 || col >= grid[0].len() as i8 {
        return;
    }
    if grid[row as usize][col as usize] != curr {
        return;
    }

    if curr == 9 {
        seen.insert((row as u8, col as u8));
        return;
    }

    dfs(seen, grid, row + 1, col, curr + 1);
    dfs(seen, grid, row, col + 1, curr + 1);
    dfs(seen, grid, row - 1, col, curr + 1);
    dfs(seen, grid, row, col - 1, curr + 1);
}

fn part1() {
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<u8>> =
        vec![vec![0; input.lines().collect::<Vec<_>>().len()]; input.lines().count()];
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if !c.is_digit(10) {
                grid[row][col] = 0xff;
            } else {
                grid[row][col] = c.to_digit(10).unwrap() as u8;
            }
        }
    }

    let mut sum: u64 = 0;
    for row in grid.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            let mut seen: HashSet<(u8, u8)> = HashSet::new();
            dfs(&mut seen, &grid, row.0 as i8, col.0 as i8, 0);
            sum += seen.len() as u64;
        }
    }
    println!("(Part 1) Sum of trailhead scores: {}", sum);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<u8>> =
        vec![vec![0; input.lines().collect::<Vec<_>>().len()]; input.lines().count()];
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if !c.is_digit(10) {
                grid[row][col] = 0xff;
            } else {
                grid[row][col] = c.to_digit(10).unwrap() as u8;
            }
        }
    }

    let mut sum: u64 = 0;
    for row in grid.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            sum += dfs2(&grid, row.0 as i8, col.0 as i8, 0);
        }
    }
    println!("(Part 2) Sum of trailhead ratings: {}", sum);
}

pub fn day10() {
    println!("---- DAY 10 ----");
    part1();
    part2();
    println!("");
}
