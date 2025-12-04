use core::num;
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

    println!("(Part 1) Total output joltage: {}", sum);
}

fn part2() {
    let input = include_str!("input.txt").split("\n");
    let mut sum = 0;
    for line in input {
        let digits: Vec<u64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let mut dp: Vec<Vec<u64>> = vec![vec![0; 12 + 1]; digits.len()];

        for c in 1..dp[0].len() {
            dp[0][c] = digits[0] as u64;
        }
        for r in 1..dp.len() {
            dp[r][1] = max(digits[r], dp[r - 1][1]);
        }
        for num_idx in 1..dp.len() {
            for bgt in 2..dp[0].len() {
                let inc = 10 * dp[num_idx - 1][bgt - 1] + digits[num_idx];
                let not_inc = dp[num_idx - 1][bgt];

                dp[num_idx][bgt] = max(inc, not_inc);
            }
        }

        sum += dp[dp.len() - 1][dp[0].len() - 1];
    }
    println!("(Part 2) Total output joltage: {}", sum);
}

pub fn day3() {
    println!("---- DAY 3 ----");
    part1();
    part2();
    println!("")
}
