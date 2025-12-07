fn part1() {
    let input = include_str!("input.txt");
    let input: Vec<&str> = input.split("\n").collect();
    let ops: Vec<char> = input
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();
    let mut vals: Vec<u64> = input
        .first()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for line in input[1..input.len() - 1].iter() {
        let line_vals: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for (idx, num) in line_vals.iter().enumerate() {
            match ops[idx] {
                '+' => {
                    vals[idx] += *num;
                }
                '*' => {
                    vals[idx] *= *num;
                }
                _ => {
                    panic!("Unexpected value in input")
                }
            }
        }
    }

    let sum: u64 = vals.iter().sum();
    println!("(Part 1) Sum of answers: {}", sum);
}

fn part2() {
    let input = include_str!("input.txt");
    let input: Vec<&str> = input.split("\n").collect();
    let input: Vec<Vec<char>> = input.iter().map(|s| (*s).chars().collect()).collect();

    let mut current_op: Option<(char, u64)> = Option::None;
    let mut sum = 0;
    for col in 0..input[0].len() {
        let op = input[input.len() - 1][col];
        if op != ' ' {
            current_op = match op {
                '+' => Some(('+', 0)),
                '*' => Some(('*', 1)),
                _ => {
                    panic!("Invalid symbol")
                }
            }
        }

        let mut num: u64 = 0;
        for row in 0..input.len() - 1 {
            let c = input[row][col];
            if c.is_digit(10) {
                num = num * 10 + (c as u8 - b'0') as u64;
            }
        }

        if num == 0 {
            sum += current_op.unwrap().1;
            continue;
        }

        match &mut current_op {
            Some(('+', curr)) => {
                *curr += num;
            }
            Some(('*', curr)) => {
                *curr *= num;
            }
            _ => {
                panic!("Invalid operator")
            }
        }
    }

    sum += current_op.unwrap().1;

    println!("(Part 2) Sum of answers: {}", sum);
}

pub fn day6() {
    println!("---- DAY 6 ----");
    part1();
    part2();
    println!("");
}
