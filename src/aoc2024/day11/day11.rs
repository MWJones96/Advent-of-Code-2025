use std::collections::HashMap;

fn part1() {
    let input = include_str!("input.txt");
    let mut stones: Vec<u64> = input
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    for _ in 0..25 {
        let mut new_stones = vec![];
        for stone in stones {
            let stone_str = stone.to_string();
            if stone == 0 {
                new_stones.push(1);
            } else if stone_str.len() % 2 == 0 {
                let first_half = &stone_str[0..stone_str.len() / 2];
                let second_half = &stone_str[stone_str.len() / 2..];
                new_stones.push(first_half.parse::<u64>().unwrap());
                new_stones.push(second_half.parse::<u64>().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }

        stones = new_stones;
    }
    println!(
        "(Part 1) Number of stones after 25 blinks: {}",
        stones.len()
    );
}

fn part2() {
    let input = include_str!("input.txt");
    let stones: Vec<u64> = input
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let mut freq: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        *freq.entry(stone).or_default() += 1;
    }
    for _ in 0..75 {
        let mut new_freq: HashMap<u64, u64> = HashMap::new();
        for (num, occurrences) in freq {
            let stone_str = num.to_string();
            if num == 0 {
                *new_freq.entry(1).or_default() += occurrences;
            } else if stone_str.len() % 2 == 0 {
                let first_half = &stone_str[0..stone_str.len() / 2];
                let second_half = &stone_str[stone_str.len() / 2..];
                *new_freq
                    .entry(first_half.parse::<u64>().unwrap())
                    .or_default() += occurrences;
                *new_freq
                    .entry(second_half.parse::<u64>().unwrap())
                    .or_default() += occurrences;
            } else {
                *new_freq.entry(num * 2024).or_default() += occurrences;
            }
        }
        freq = new_freq;
    }

    println!(
        "(Part 2) Number of stones after 75 blinks: {}",
        freq.values().sum::<u64>()
    );
}

pub fn day11() {
    println!("---- DAY 11 ----");
    part1();
    part2();
    println!("");
}
