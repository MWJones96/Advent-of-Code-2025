use std::collections::{HashSet, VecDeque};

fn does_path_exist(
    start: (u8, u8),
    end: (u8, u8),
    obstacles: &HashSet<(u8, u8)>,
    width: u8,
    height: u8,
) -> bool {
    let mut queue: VecDeque<(u8, u8)> = VecDeque::new();
    queue.push_back(start);

    let mut seen: HashSet<(u8, u8)> = HashSet::new();
    seen.insert(start);
    while !queue.is_empty() {
        let queue_len = queue.len();
        for _ in 0..queue_len {
            let (row, col) = queue.pop_front().unwrap();
            if (row, col) == end {
                return true;
            }
            let next: [(i8, i8); 4] = [
                (row as i8 + 1, col as i8),
                (row as i8 - 1, col as i8),
                (row as i8, col as i8 + 1),
                (row as i8, col as i8 - 1),
            ];
            for (n_row, n_col) in next {
                if n_row < 0 || n_row >= height as i8 {
                    continue;
                }
                if n_col < 0 || n_col >= width as i8 {
                    continue;
                }
                if seen.contains(&(n_row as u8, n_col as u8)) {
                    continue;
                }
                if obstacles.contains(&(n_row as u8, n_col as u8)) {
                    continue;
                }
                queue.push_back((n_row as u8, n_col as u8));
                seen.insert((n_row as u8, n_col as u8));
            }
        }
    }

    return false;
}

fn part1() {
    let input = include_str!("input.txt");
    let lines: Vec<_> = input.lines().collect();
    let mut obstacles: HashSet<(u8, u8)> = HashSet::new();
    let mut height: u8 = 0;
    let mut width: u8 = 0;
    for line in &lines[0..1024] {
        let line = line.split_once(",").unwrap();
        let (col, row) = (line.0.parse::<u8>().unwrap(), line.1.parse::<u8>().unwrap());
        obstacles.insert((row, col));
        height = height.max(row + 1);
        width = width.max(col + 1);
    }
    let start = (0, 0);
    let end = (height - 1, width - 1);

    let mut queue: VecDeque<(u8, u8)> = VecDeque::new();
    queue.push_back(start);

    let mut seen: HashSet<(u8, u8)> = HashSet::new();
    seen.insert(start);
    let mut idx = 0;
    'outer: while !queue.is_empty() {
        let queue_len = queue.len();
        for _ in 0..queue_len {
            let (row, col) = queue.pop_front().unwrap();
            if (row, col) == end {
                break 'outer;
            }
            let next: [(i8, i8); 4] = [
                (row as i8 + 1, col as i8),
                (row as i8 - 1, col as i8),
                (row as i8, col as i8 + 1),
                (row as i8, col as i8 - 1),
            ];
            for (n_row, n_col) in next {
                if n_row < 0 || n_row >= height as i8 {
                    continue;
                }
                if n_col < 0 || n_col >= width as i8 {
                    continue;
                }
                if seen.contains(&(n_row as u8, n_col as u8)) {
                    continue;
                }
                if obstacles.contains(&(n_row as u8, n_col as u8)) {
                    continue;
                }
                queue.push_back((n_row as u8, n_col as u8));
                seen.insert((n_row as u8, n_col as u8));
            }
        }
        idx += 1;
    }

    println!("(Part 1) Shortest path with 1024 obstacles: {}", idx);
}

fn part2() {
    let input = include_str!("input.txt");
    let lines: Vec<_> = input.lines().collect();
    let mut obstacles: Vec<(u8, u8)> = Vec::new();
    let mut height: u8 = 0;
    let mut width: u8 = 0;
    for line in &lines {
        let line = line.split_once(",").unwrap();
        let (col, row) = (line.0.parse::<u8>().unwrap(), line.1.parse::<u8>().unwrap());
        obstacles.push((row, col));
        height = height.max(row + 1);
        width = width.max(col + 1);
    }
    let start = (0, 0);
    let end = (height - 1, width - 1);

    let mut l: usize = 0;
    let mut r: usize = obstacles.len() - 1;
    let mut obstacle_set: HashSet<(u8, u8)> = HashSet::new();
    while l < r {
        let m = (l + r) / 2;
        for i in l..=m {
            obstacle_set.insert(obstacles[i]);
        }
        for j in (m + 1)..=r {
            obstacle_set.remove(&obstacles[j]);
        }
        if does_path_exist(start, end, &obstacle_set, width, height) {
            l = m + 1;
        } else {
            r = m;
        }
    }

    let obstacle = obstacles[l];
    println!(
        "(Part 2) First byte preventing exit: {},{}",
        obstacle.1, obstacle.0
    );
}

pub fn day18() {
    println!("---- DAY 18 ----");
    part1();
    part2();
    println!("");
}
