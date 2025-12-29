fn part1() {
    fn dfs(
        grid: &Vec<Vec<char>>,
        word: &str,
        idx: usize,
        grid_pos: (isize, isize),
        dir: (isize, isize),
    ) -> u64 {
        if idx >= word.len() {
            return 1;
        }
        if grid_pos.0 < 0 || grid_pos.0 >= grid.len() as isize {
            return 0;
        }
        if grid_pos.1 < 0 || grid_pos.1 >= grid[0].len() as isize {
            return 0;
        }

        if grid[grid_pos.0 as usize][grid_pos.1 as usize] == word.chars().nth(idx).unwrap() {
            return dfs(
                grid,
                word,
                idx + 1,
                (grid_pos.0 + dir.0, grid_pos.1 + dir.1),
                dir,
            );
        }

        return 0;
    }

    let input = include_str!("input.txt");
    let word = "XMAS";
    let mut sum = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            sum += dfs(&grid, word, 0, (row as isize, col as isize), (1, 0))
                + dfs(&grid, word, 0, (row as isize, col as isize), (-1, 0))
                + dfs(&grid, word, 0, (row as isize, col as isize), (0, 1))
                + dfs(&grid, word, 0, (row as isize, col as isize), (0, -1))
                + dfs(&grid, word, 0, (row as isize, col as isize), (1, 1))
                + dfs(&grid, word, 0, (row as isize, col as isize), (-1, -1))
                + dfs(&grid, word, 0, (row as isize, col as isize), (1, -1))
                + dfs(&grid, word, 0, (row as isize, col as isize), (-1, 1))
        }
    }
    println!("(Part 1) 'XMAS' appearances in wordsearch: {}", sum);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 4) Part 2 not implemented yet");
}

pub fn day4() {
    println!("---- 2024 DAY 4 ----");
    part1();
    part2();
    println!("");
}
