use std::collections::HashMap;

fn part1() {
    let input = include_str!("input.txt");
    let input: Vec<&str> = input.split("\n").collect();
    let mut input: Vec<Vec<char>> = input.iter().map(|c| c.chars().collect()).collect();

    let mut split_count: u64 = 0;

    for row in 1..input.len() {
        for col in 0..input[0].len() {
            match input[row - 1][col] {
                '.' => {}
                '^' => {}
                '|' | 'S' => match input[row][col] {
                    '.' => {
                        input[row][col] = '|';
                    }
                    '^' => {
                        input[row][col - 1] = '|';
                        input[row][col + 1] = '|';
                        split_count += 1;
                    }
                    '|' => {}
                    _ => {
                        panic!("Invalid character")
                    }
                },
                _ => {
                    panic!("Invalid character")
                }
            }
        }
    }

    println!("(Part 1) Tachyon beam splits: {}", split_count);
}

fn part2() {
    fn dfs(memo: &mut HashMap<(u64, u64), u64>, input: &Vec<Vec<char>>, row: u64, col: u64) -> u64 {
        if row == input.len() as u64 {
            return 1;
        }

        if memo.contains_key(&(row, col)) {
            return *memo.get(&(row, col)).unwrap();
        }

        match input[row as usize][col as usize] {
            'S' | '.' => {
                let dfs = dfs(memo, input, row + 1, col);
                memo.insert((row, col), dfs);
                return dfs;
            }
            '^' => {
                let dfs = dfs(memo, input, row, col - 1) + dfs(memo, input, row, col + 1);
                memo.insert((row, col), dfs);
                return dfs;
            }
            _ => {
                panic!("Invalid character")
            }
        }
    }

    let input = include_str!("input.txt");
    let input: Vec<&str> = input.split("\n").collect();
    let input: Vec<Vec<char>> = input.iter().map(|c| c.chars().collect()).collect();
    let start_col = input[0].iter().position(|c| *c == 'S').unwrap();

    println!(
        "(Part 2) Number of timelines: {}",
        dfs(&mut HashMap::new(), &input, 0, start_col as u64)
    );
}

pub fn day7() {
    println!("---- DAY 7 ----");
    part1();
    part2();
    println!("");
}
