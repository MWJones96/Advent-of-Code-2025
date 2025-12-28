use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
    println!("(Day 1) Total distance between lists: {}", total_distance);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(Day 1)");
}

pub fn day1() {
    println!("---- 2024 DAY 1 ----");
    part1();
    part2();
    println!("");
}
