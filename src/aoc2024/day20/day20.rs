use std::collections::HashMap;

fn dfs(
    grid: &Vec<&str>,
    seen: &mut HashMap<(usize, usize), usize>,
    row: usize,
    col: usize,
    depth: usize,
) {
    if grid[row].chars().nth(col).unwrap() == '#' {
        return;
    }
    if seen.contains_key(&(row, col)) {
        return;
    }

    seen.insert((row, col), depth);

    dfs(grid, seen, row + 1, col, depth + 1);
    dfs(grid, seen, row - 1, col, depth + 1);
    dfs(grid, seen, row, col + 1, depth + 1);
    dfs(grid, seen, row, col - 1, depth + 1);
}

fn part1() {
    let input = include_str!("input.txt");
    let grid: Vec<&str> = input.split("\n").collect();
    let mut skips: u64 = 0;
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();
    for (row_i, row) in grid.iter().enumerate() {
        for (col_i, col) in row.chars().enumerate() {
            if col == 'E' {
                dfs(&grid, &mut seen, row_i, col_i, 0);
                break;
            }
        }
    }

    for (row, col) in seen.keys() {
        let dist_to_end = *seen.get(&(*row, *col)).unwrap();
        if *row >= 2 && seen.contains_key(&(row - 2, *col)) {
            let skip_dist = 2 + seen.get(&(row - 2, *col)).unwrap();
            let timesave = dist_to_end as i64 - skip_dist as i64;
            if timesave >= 100 {
                skips += 1;
            }
        }
        if seen.contains_key(&(row + 2, *col)) {
            let skip_dist = 2 + seen.get(&(row + 2, *col)).unwrap();
            let timesave = dist_to_end as i64 - skip_dist as i64;
            if timesave >= 100 {
                skips += 1;
            }
        }
        if *col >= 2 && seen.contains_key(&(*row, col - 2)) {
            let skip_dist = 2 + seen.get(&(*row, col - 2)).unwrap();
            let timesave = dist_to_end as i64 - skip_dist as i64;
            if timesave >= 100 {
                skips += 1;
            }
        }
        if seen.contains_key(&(*row, col + 2)) {
            let skip_dist = 2 + seen.get(&(*row, col + 2)).unwrap();
            let timesave = dist_to_end as i64 - skip_dist as i64;
            if timesave >= 100 {
                skips += 1;
            }
        }
    }
    println!("(Part 1) Cheats saving >= 100ps: {}", skips);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 20) Part 2 not implemented yet");
}

pub fn day20() {
    println!("---- DAY 20 ----");
    part1();
    part2();
    println!("");
}
