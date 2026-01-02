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
    println!("(Part 1) Number of stones: {}", stones.len());
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 11) Part 2 not implemented yet");
}

pub fn day11() {
    println!("---- DAY 11 ----");
    part1();
    part2();
    println!("");
}
