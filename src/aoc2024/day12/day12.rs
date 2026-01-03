use std::collections::HashSet;

fn dfs(
    grid: &mut Vec<Vec<char>>,
    seen: &mut HashSet<(u8, u8)>,
    row: i16,
    col: i16,
    curr: char,
    area: &mut u64,
    perimeter: &mut u64,
) {
    if row < 0 || row >= grid.len() as i16 {
        return;
    }
    if col < 0 || col >= grid[0].len() as i16 {
        return;
    }
    if seen.contains(&(row as u8, col as u8)) {
        return;
    }
    if curr != grid[row as usize][col as usize] {
        return;
    }

    if row == 0 || grid[(row - 1) as usize][col as usize] != curr {
        *perimeter += 1;
    }
    if row == grid.len() as i16 - 1 || grid[(row + 1) as usize][col as usize] != curr {
        *perimeter += 1;
    }
    if col == 0 || grid[row as usize][(col - 1) as usize] != curr {
        *perimeter += 1;
    }
    if col == grid[0].len() as i16 - 1 || grid[row as usize][(col + 1) as usize] != curr {
        *perimeter += 1;
    }
    *area += 1;

    seen.insert((row as u8, col as u8));
    dfs(grid, seen, row + 1, col, curr, area, perimeter);
    dfs(grid, seen, row - 1, col, curr, area, perimeter);
    dfs(grid, seen, row, col + 1, curr, area, perimeter);
    dfs(grid, seen, row, col - 1, curr, area, perimeter);
}

fn part1() {
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<char>> = vec![];
    let mut seen: HashSet<(u8, u8)> = HashSet::new();
    let mut price: u64 = 0;
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let ch = grid[row][col];
            let mut area = 0;
            let mut perimeter = 0;
            dfs(
                &mut grid,
                &mut seen,
                row as i16,
                col as i16,
                ch,
                &mut area,
                &mut perimeter,
            );

            price += area * perimeter;
        }
    }
    println!("(Part 1) Total price of fencing: {}", price);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(Part 2) Part 2 not implemented yet");
}

pub fn day12() {
    println!("---- DAY 12 ----");
    part1();
    part2();
    println!("");
}
