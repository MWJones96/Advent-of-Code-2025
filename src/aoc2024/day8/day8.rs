use std::collections::{HashMap, HashSet};

use regex::Regex;

fn part1() {
    let input = include_str!("input.txt");
    let re = Regex::new(r"[0-9a-zA-Z]").unwrap();
    let mut nodes: HashMap<char, Vec<(u8, u8)>> = HashMap::new();
    let mut antinodes: HashSet<(u8, u8)> = HashSet::new();

    let row_count = input.lines().collect::<Vec<_>>().len() as i32;
    let col_count = input.lines().collect::<Vec<_>>()[0].len() as i32;

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if re.is_match(&c.to_string()) {
                nodes
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((row as u8, col as u8));
            }
        }
    }
    for key in nodes.keys() {
        let vals = nodes.get(key).unwrap();
        for i in 0..vals.len() {
            for j in (i + 1)..vals.len() {
                let (row_i, col_i) = vals[i];
                let (row_j, col_j) = vals[j];

                let diff_row: i32 = row_i.abs_diff(row_j) as i32;
                let diff_col: i32 = col_i.abs_diff(col_j) as i32;

                let candidates = if row_j > row_i && col_j > col_i {
                    //TL
                    [
                        (row_i as i32 - diff_row, col_i as i32 - diff_col),
                        (row_j as i32 + diff_row, col_j as i32 + diff_col),
                    ]
                } else if row_j > row_i && col_j < col_i {
                    //TR
                    [
                        (row_i as i32 - diff_row, col_i as i32 + diff_col),
                        (row_j as i32 + diff_row, col_j as i32 - diff_col),
                    ]
                } else if row_j < row_i && col_j > col_i {
                    //BL
                    [
                        (row_i as i32 + diff_row, col_i as i32 - diff_col),
                        (row_j as i32 - diff_row, col_j as i32 + diff_col),
                    ]
                } else {
                    //BR
                    [
                        (row_i as i32 + diff_row, col_i as i32 + diff_col),
                        (row_i as i32 - diff_row, col_i as i32 - diff_col),
                    ]
                };

                for (row, col) in candidates {
                    if row < 0 || row >= row_count || col < 0 || col >= col_count {
                        continue;
                    }
                    antinodes.insert((row as u8, col as u8));
                }
            }
        }
    }
    println!("(Part 1) Unique antinode locations: {}", antinodes.len());
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 8) Part 2 not implemented yet");
}

pub fn day8() {
    println!("---- 2024 DAY 8 ----");
    part1();
    part2();
    println!("");
}
