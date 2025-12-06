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
    let _input = include_str!("input.txt");
    println!("(Part 2) not implemented yet");
}

pub fn day6() {
    println!("---- DAY 6 ----");
    part1();
    part2();
    println!("");
}
