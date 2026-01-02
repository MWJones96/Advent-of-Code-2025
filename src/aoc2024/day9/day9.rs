use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn part1() {
    let input = include_str!("input.txt");
    let line = input.lines().collect::<Vec<&str>>()[0];
    let mut is_file: bool = true;
    let mut hard_drive: Vec<i16> = vec![];
    let mut id = 0;
    let mut idx = 0;
    let mut file_idx: Vec<usize> = vec![];
    let mut free_idx: Vec<usize> = vec![];

    for n in line.chars().map(|c| c.to_digit(10).unwrap()) {
        if is_file {
            hard_drive.extend(std::iter::repeat(id).take(n as usize));
            for i in idx..idx + n {
                file_idx.push(i as usize);
            }
            id += 1;
        } else {
            hard_drive.extend(std::iter::repeat(-1).take(n as usize));
            for i in idx..idx + n {
                free_idx.push(i as usize);
            }
        }
        idx += n;
        is_file = !is_file;
    }

    let mut l: usize = 0;
    let mut r: usize = file_idx.len() - 1;

    while free_idx[l] < file_idx[r] {
        let free_hd = free_idx[l];
        let file_hd = file_idx[r];

        hard_drive[free_hd] = hard_drive[file_hd];
        hard_drive[file_hd] = -1;

        l += 1;
        r -= 1;
    }

    let mut checksum: u64 = 0;
    for (idx, val) in hard_drive.iter().enumerate() {
        if *val == -1 {
            break;
        }
        checksum += *val as u64 * idx as u64;
    }

    println!("(Part 1) Checksum: {}", checksum);
}

fn part2() {
    let input = include_str!("input.txt");
    let line = input.lines().collect::<Vec<&str>>()[0];
    let mut is_file: bool = true;
    let mut hard_drive: Vec<i16> = vec![];
    let mut free_space: HashMap<u8, BinaryHeap<Reverse<usize>>> = HashMap::new();
    let mut file_space: Vec<(i16, usize, usize)> = vec![];
    let mut id = 0;
    let mut idx = 0;

    for n in line.chars().map(|c| c.to_digit(10).unwrap()) {
        if is_file {
            hard_drive.extend(std::iter::repeat(id).take(n as usize));
            if n > 0 {
                file_space.push((id, idx, idx + n as usize - 1));
            }
            id += 1;
        } else {
            hard_drive.extend(std::iter::repeat(-1).take(n as usize));
            if n > 0 {
                free_space.entry(n as u8).or_default().push(Reverse(idx));
            }
        }
        idx += n as usize;
        is_file = !is_file;
    }
    for (id, start, end) in file_space.iter_mut().rev() {
        let file_size_blocks = *end - *start + 1;
        let mut min_idx = (*start, 0);
        for gap_size in file_size_blocks..=9 {
            let v = free_space.entry(gap_size as u8).or_default();
            match v.peek() {
                Some(val) => {
                    if val.0 < min_idx.0 {
                        min_idx.0 = val.0;
                        min_idx.1 = gap_size;
                    }
                }
                None => {}
            }
        }
        if min_idx.0 < *start {
            let v = free_space
                .entry(min_idx.1 as u8)
                .or_default()
                .pop()
                .unwrap()
                .0;

            for i in v..v + file_size_blocks {
                hard_drive[i] = *id;
            }
            for j in *start..=*end {
                hard_drive[j] = -1;
            }

            let diff = min_idx.1 - file_size_blocks;
            if diff > 0 {
                free_space
                    .entry(diff as u8)
                    .or_default()
                    .push(Reverse(v + file_size_blocks));
            }
        }
    }
    let mut sum: u64 = 0;
    for (i, n) in hard_drive.iter().enumerate() {
        if *n == -1 {
            continue;
        }
        sum += i as u64 * *n as u64;
    }
    println!("(Part 2) Checksum: {}", sum);
}

pub fn day9() {
    println!("---- DAY 9 ----");
    part1();
    part2();
    println!("");
}
