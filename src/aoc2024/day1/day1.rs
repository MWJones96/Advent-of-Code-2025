use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn part1() {
    let input = include_str!("input.txt");
    let mut total_distance: u64 = 0;
    let mut l_heap = BinaryHeap::new();
    let mut r_heap = BinaryHeap::new();
    for line in input.lines() {
        if let Some(v) = line.split_once(" ") {
            let l = v.0.trim().parse::<u32>().unwrap();
            let r = v.1.trim().parse::<u32>().unwrap();
            l_heap.push(Reverse(l));
            r_heap.push(Reverse(r));
        }
    }

    while let (Some(l), Some(r)) = (l_heap.pop(), r_heap.pop()) {
        total_distance += l.0.abs_diff(r.0) as u64;
    }
    println!("(Part 1) Total distance between lists: {}", total_distance);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut right_freq = HashMap::new();
    let mut sim_score: u64 = 0;
    let mut left_nums = vec![];
    for line in input.lines() {
        if let Some(v) = line.split_once(" ") {
            let l = v.0.trim().parse::<u32>().unwrap();
            left_nums.push(l);
            let r = v.1.trim().parse::<u32>().unwrap();
            let val = right_freq.entry(r).or_insert(0);
            *val += 1;
        }
    }

    for num in left_nums {
        sim_score += num as u64 * right_freq.get(&num).unwrap_or(&0);
    }

    println!("(Part 2) Similarity Score: {}", sim_score);
}

pub fn day1() {
    println!("---- DAY 1 ----");
    part1();
    part2();
    println!("");
}
