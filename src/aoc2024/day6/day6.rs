use std::collections::HashSet;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
    fn mv(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Up => (pos.0 - 1, pos.1),
            Dir::Down => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0, pos.1 - 1),
            Dir::Right => (pos.0, pos.1 + 1),
        }
    }
}

fn part1() {
    let input = include_str!("input.txt");
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    let mut visited: HashSet<(u8, u8)> = HashSet::new();

    let mut pos: (i32, i32) = (0, 0);
    let mut dir = Dir::Up;

    let row_count: i32 = input.lines().collect::<Vec<&str>>().len() as i32;
    let col_count: Vec<char> = input.lines().collect::<Vec<&str>>()[0].chars().collect();
    let col_count: i32 = col_count.len() as i32;

    for (row, line) in input.lines().enumerate() {
        for (col, item) in line.chars().enumerate() {
            match item {
                '#' => {
                    obstacles.insert((row as i32, col as i32));
                }
                '^' => {
                    pos = (row as i32, col as i32);
                    visited.insert((row as u8, col as u8));
                }
                _ => {}
            }
        }
    }

    while pos.0 >= 0 && pos.0 < row_count && pos.1 >= 0 && pos.1 < col_count {
        visited.insert((pos.0 as u8, pos.1 as u8));
        let new_pos = dir.mv(pos);
        if obstacles.contains(&new_pos) {
            dir = dir.turn();
        } else {
            pos = new_pos;
        }
    }

    println!("(Part 1) Distinct positions visited: {}", visited.len());
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 6) Part 2 not implemented yet");
}

pub fn day6() {
    println!("---- 2024 DAY 6 ----");
    part1();
    part2();
    println!("");
}
