use std::u64;

use z3::Solver;
use z3::ast::Int;

fn parse_prize(line: &str) -> (u64, u64) {
    // "Button A: X+94, Y+34"
    let (_, rest) = line.split_once(": ").unwrap();
    let (x, y) = rest.split_once(", ").unwrap();

    let x = x.strip_prefix("X=").unwrap().parse().unwrap();
    let y = y.strip_prefix("Y=").unwrap().parse().unwrap();

    (x, y)
}

fn parse_button(line: &str) -> (u64, u64) {
    // "Button A: X+94, Y+34"
    let (_, rest) = line.split_once(": ").unwrap();
    let (x, y) = rest.split_once(", ").unwrap();

    let x = x.strip_prefix("X+").unwrap().parse().unwrap();
    let y = y.strip_prefix("Y+").unwrap().parse().unwrap();

    (x, y)
}

fn part1() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut tokens: u64 = 0;

    loop {
        let chunk: Vec<&str> = lines.by_ref().take(4).collect();
        if chunk.is_empty() {
            break;
        }

        let (a_x, a_y) = parse_button(chunk[0]);
        let (b_x, b_y) = parse_button(chunk[1]);
        let (prize_x, prize_y) = parse_prize(chunk[2]);

        let a_presses = Int::fresh_const("a");
        let b_presses = Int::fresh_const("b");

        let solver = Solver::new();
        let mut min: u64 = u64::MAX;

        solver.assert(a_presses.ge(0));
        solver.assert(b_presses.ge(0));

        solver.assert((a_x * &a_presses + b_x * &b_presses).eq(prize_x));
        solver.assert((a_y * &a_presses + b_y * &b_presses).eq(prize_y));
        solver.assert((Int::from(3) * &a_presses + &b_presses).lt(min));

        let mut solution: Vec<_> = solver
            .solutions([&a_presses, &b_presses], false)
            .take(1)
            .collect();
        if solution.len() > 0 {
            while solution.len() >= 1 {
                let ab = solution[0].clone();
                let a = ab[0].as_u64().unwrap();
                let b = ab[1].as_u64().unwrap();

                min = min.min(3 * a + b);
                solver.assert((Int::from(3) * &a_presses + &b_presses).lt(min));

                solution = solver
                    .solutions([&a_presses, &b_presses], false)
                    .take(1)
                    .collect();
            }
            tokens += min;
        }
    }
    println!("(Part 1) Minimum tokens required: {}", tokens);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut tokens: u64 = 0;

    loop {
        let chunk: Vec<&str> = lines.by_ref().take(4).collect();
        if chunk.is_empty() {
            break;
        }

        let (a_x, a_y) = parse_button(chunk[0]);
        let (b_x, b_y) = parse_button(chunk[1]);
        let (mut prize_x, mut prize_y) = parse_prize(chunk[2]);
        prize_x += 10000000000000;
        prize_y += 10000000000000;

        let a_presses = Int::fresh_const("a");
        let b_presses = Int::fresh_const("b");

        let solver = Solver::new();
        let mut min: u64 = u64::MAX;

        solver.assert(a_presses.ge(0));
        solver.assert(b_presses.ge(0));

        solver.assert((a_x * &a_presses + b_x * &b_presses).eq(prize_x));
        solver.assert((a_y * &a_presses + b_y * &b_presses).eq(prize_y));

        let mut solution: Vec<_> = solver
            .solutions([&a_presses, &b_presses], false)
            .take(1)
            .collect();
        if solution.len() > 0 {
            while solution.len() >= 1 {
                let ab = solution[0].clone();
                let a = ab[0].as_u64().unwrap();
                let b = ab[1].as_u64().unwrap();

                min = min.min(3 * a + b);
                solver.assert((Int::from(3) * &a_presses + &b_presses).lt(min));

                solution = solver
                    .solutions([&a_presses, &b_presses], false)
                    .take(1)
                    .collect();
            }
            tokens += min;
        }
    }
    println!("(Part 2) Minimum tokens required: {}", tokens);
}

pub fn day13() {
    println!("---- DAY 13 ----");
    part1();
    part2();
    println!("");
}
