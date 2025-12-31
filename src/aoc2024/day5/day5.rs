use std::collections::{HashMap, HashSet};

fn merge_sort(nums: &Vec<u8>, l: usize, r: usize, nodes: &HashMap<u8, HashSet<u8>>) -> Vec<u8> {
    if r - l < 1 {
        return vec![nums[l]];
    }

    let m = (l + r) / 2;
    let l_arr = merge_sort(nums, l, m, nodes);
    let r_arr = merge_sort(nums, m + 1, r, nodes);

    let mut l_ptr: usize = 0;
    let mut r_ptr: usize = 0;
    let mut res_ptr: usize = 0;
    let mut res = vec![0; l_arr.len() + r_arr.len()];

    while l_ptr < l_arr.len() && r_ptr < r_arr.len() {
        let l_val = l_arr[l_ptr];
        let r_val = r_arr[r_ptr];

        // L < R
        if nodes.get(&l_val).unwrap().contains(&r_val) {
            res[res_ptr] = l_val;
            l_ptr += 1;
        // L >= R
        } else {
            res[res_ptr] = r_val;
            r_ptr += 1;
        }

        res_ptr += 1;
    }

    while l_ptr < l_arr.len() {
        res[res_ptr] = l_arr[l_ptr];
        l_ptr += 1;
        res_ptr += 1;
    }

    while r_ptr < r_arr.len() {
        res[res_ptr] = r_arr[r_ptr];
        r_ptr += 1;
        res_ptr += 1;
    }

    res
}

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
                nodes.entry(to).or_insert_with(HashSet::new);
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
    let input = include_str!("input.txt");
    let mut nodes: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut sum: u64 = 0;

    for line in input.lines() {
        if line.contains("|") {
            if let Some((from_str, to_str)) = line.split_once("|") {
                let from = from_str.parse::<u8>().unwrap();
                let to = to_str.parse::<u8>().unwrap();
                nodes.entry(from).or_insert_with(HashSet::new).insert(to);
                nodes.entry(to).or_insert_with(HashSet::new);
            }
        } else if line.contains(",") {
            let nums: Vec<&str> = line.split(",").collect();
            let nums: Vec<u8> = nums.iter().map(|n| n.parse::<u8>().unwrap()).collect();
            if !in_order(&nums, &nodes) {
                let sorted = merge_sort(&nums, 0, nums.len() - 1, &nodes);
                sum += sorted[sorted.len() / 2] as u64;
            }
        }
    }
    println!("(Part 2) Sum of middle pages of corrected updates: {}", sum);
}

pub fn day5() {
    println!("---- DAY 5 ----");
    part1();
    part2();
    println!("");
}
