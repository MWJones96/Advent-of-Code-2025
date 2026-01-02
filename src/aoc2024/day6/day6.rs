use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

fn is_path_a_loop(
    obstacles: &HashSet<(i32, i32)>,
    row_count: i32,
    col_count: i32,
    start_point: (i32, i32),
) -> bool {
    let mut pos: (i32, i32) = start_point;
    let mut dir = Dir::Up;

    let mut visited: HashSet<(Dir, u8, u8)> = HashSet::new();

    while pos.0 >= 0 && pos.0 < row_count && pos.1 >= 0 && pos.1 < col_count {
        if visited.contains(&(dir, pos.0 as u8, pos.1 as u8)) {
            return true;
        }
        visited.insert((dir, pos.0 as u8, pos.1 as u8));
        let new_pos = dir.mv(pos);
        if obstacles.contains(&new_pos) {
            dir = dir.turn();
        } else {
            pos = new_pos;
        }
    }

    false
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
    let input = include_str!("input.txt");
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    let mut visited: HashSet<(u8, u8)> = HashSet::new();

    let mut pos: (i32, i32) = (0, 0);
    let mut dir = Dir::Up;

    let mut start_point: (i32, i32) = (0, 0);

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
                    start_point = (row as i32, col as i32);
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

    let mut sum = 0;

    for v in visited {
        if (v.0 as i32, v.1 as i32) == start_point {
            continue;
        }
        obstacles.insert((v.0 as i32, v.1 as i32));
        if is_path_a_loop(&obstacles, row_count, col_count, start_point) {
            sum += 1;
        }
        obstacles.remove(&(v.0 as i32, v.1 as i32));
    }

    println!("(Part 2) Positions for obstruction: {}", sum);
}

pub fn day6() {
    println!("---- DAY 6 ----");
    part1();
    part2();
    println!("");
}
