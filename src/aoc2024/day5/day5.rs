use std::collections::{HashMap, HashSet};

fn in_order(nums: &Vec<u8>, nodes: &HashMap<u8, HashSet<u8>>) -> bool {
    let empty_set = HashSet::new();
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            continue;
        }
        let can_be_before = nodes.get(&nums[i - 1]).unwrap_or(&empty_set);
        if !can_be_before.contains(&nums[i]) {
            return false;
        }
    }
    return true;
}

fn part1() {
    let input = include_str!("input.txt");
    let mut nodes: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut sum: u64 = 0;

    for line in input.lines() {
        if line.contains("|") {
            if let Some((from_str, to_str)) = line.split_once("|") {
                let from = from_str.parse::<u8>().unwrap();
                let to = to_str.parse::<u8>().unwrap();
                nodes.entry(from).or_insert_with(HashSet::new).insert(to);
            }
        } else if line.contains(",") {
            let nums: Vec<&str> = line.split(",").collect();
            let nums: Vec<u8> = nums.iter().map(|n| n.parse::<u8>().unwrap()).collect();
            if in_order(&nums, &nodes) {
                sum += nums[nums.len() / 2] as u64;
            }
        }
    }
    println!(
        "(Part 1) Sum of middle pages of correctly-ordered updates: {}",
        sum
    );
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 5) Part 2 not implemented yet");
}

pub fn day5() {
    println!("---- DAY 5 ----");
    part1();
    part2();
    println!("");
}
