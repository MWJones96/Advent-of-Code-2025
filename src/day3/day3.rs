use std::cmp::max;

fn part1() {
    let input = include_str!("input.txt").split("\n");
    let mut sum = 0;
    for line in input {
        let mut max_line_jolts = 0;
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut max_to_right = vec![0; line.len()];
        for i in (0..max_to_right.len() - 1).rev() {
            max_to_right[i] = max(max_to_right[i + 1], digits[i + 1]);
        }

        for j in 0..digits.len() - 1 {
            let n = 10 * digits[j] + max_to_right[j];
            max_line_jolts = max(max_line_jolts, n);
        }

        sum += max_line_jolts;
    }

    println!("(Day 3) (Part 1) Total output joltage: {}", sum);
}

fn part2() {
    println!("(Day 3) (Part 2) {}", 0);
}

pub fn day3() {
    part1();
    part2();
}
