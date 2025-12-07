use std::collections::HashSet;

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
    let input = include_str!("test_input.txt");
    println!("(Part 2) not implemented yet");
}

pub fn day7() {
    println!("---- DAY 7 ----");
    part1();
    part2();
    println!("");
}
