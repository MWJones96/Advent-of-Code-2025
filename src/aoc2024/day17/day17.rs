fn combo_operand(operand: u8, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand as u64,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid!"),
    }
}

fn execute_instruction(
    opcode: u8,
    operand: u8,
    a: &mut u64,
    b: &mut u64,
    c: &mut u64,
    ip: &mut usize,
) -> Option<u64> {
    match opcode {
        0 => {
            let numerator = *a as f64;
            let denominator = 2u64.pow(combo_operand(operand, *a, *b, *c) as u32) as f64;
            let val = (numerator / denominator) as u64;
            *a = val;
            *ip += 2;
            None
        }
        1 => {
            let val = *b ^ (operand as u64);
            *b = val;
            *ip += 2;
            None
        }
        2 => {
            *b = (combo_operand(operand, *a, *b, *c) & 7) as u64;
            *ip += 2;
            None
        }
        3 => {
            if *a != 0 {
                *ip = operand as usize;
            } else {
                *ip += 2;
            }
            None
        }
        4 => {
            let val = *b ^ *c;
            *b = val;
            *ip += 2;
            None
        }
        5 => {
            let val = combo_operand(operand, *a, *b, *c) & 7;
            *ip += 2;
            Some(val)
        }
        6 => {
            let numerator = *a as f64;
            let denominator = 2u64.pow(combo_operand(operand, *a, *b, *c) as u32) as f64;
            let val = (numerator / denominator) as u64;
            *b = val;
            *ip += 2;
            None
        }
        7 => {
            let numerator = *a as f64;
            let denominator = 2u64.pow(combo_operand(operand, *a, *b, *c) as u32) as f64;
            let val = (numerator / denominator) as u64;
            *c = val;
            *ip += 2;
            None
        }

        _ => panic!("Invalid opcode"),
    }
}

fn part1() {
    let input = include_str!("input.txt");
    let input = input.lines().collect::<Vec<&str>>();

    let mut a = input[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .parse::<u64>()
        .unwrap();
    let mut b = input[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .parse::<u64>()
        .unwrap();
    let mut c = input[2]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .parse::<u64>()
        .unwrap();

    let program: Vec<&str> = input[4]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(",")
        .collect();

    let mut print: Vec<String> = Vec::new();
    let mut ip: usize = 0;
    while ip < program.len() {
        let opcode = program[ip].parse::<u8>().unwrap();
        let operand = program[ip + 1].parse::<u8>().unwrap();

        if let Some(val) = execute_instruction(opcode, operand, &mut a, &mut b, &mut c, &mut ip) {
            print.push(val.to_string());
        }
    }

    println!("(Part 1) Program output: {}", print.join(","));
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 17) Part 2 not implemented yet");
}

pub fn day17() {
    println!("---- DAY 17 ----");
    part1();
    part2();
    println!("");
}
