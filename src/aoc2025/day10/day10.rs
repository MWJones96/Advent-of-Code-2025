use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet, VecDeque};
use z3::Solver;
use z3::ast::Int;

use regex::Regex;
static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\[[^\]]+\])\s+((?:\([^)]+\)\s*)+)\s+(\{[^}]+\})$").unwrap());

fn part1() {
    let input = include_str!("input.txt");
    let mut sum = 0;

    for line in input.split("\n") {
        let line = RE.captures(line).unwrap();
        let target_state: &str = &line[1][1..line[1].len() - 1];
        let target_state: Vec<bool> = target_state.chars().map(|c| c == '#').collect();

        let moves: &str = &line[2];
        let moves: Vec<&str> = moves.split(" ").collect();
        let moves: Vec<Vec<usize>> = moves
            .iter()
            .map(|s| {
                s.trim_matches(&['(', ')'][..])
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let initial_state = vec![false; target_state.len()];
        let mut seen: HashSet<Vec<bool>> = HashSet::new();
        let mut queue: VecDeque<Vec<bool>> = VecDeque::new();
        let mut depth = 0;

        queue.push_back(initial_state);
        'outer: while !queue.is_empty() {
            let queue_size = queue.len();
            for _ in 0..queue_size {
                let node: Vec<bool> = queue.pop_front().unwrap();
                if node == target_state {
                    break 'outer;
                }
                for _move in &moves {
                    let mut state = node.clone();
                    for &idx in _move {
                        state[idx] = !state[idx];
                    }
                    if !seen.contains(&state) {
                        seen.insert(state.clone());
                        queue.push_back(state.clone());
                    }
                }
            }
            depth += 1;
        }
        sum += depth;
    }
    println!("(Part 1) Fewest button presses: {}", sum);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut sum: u64 = 0;
    for line in input.lines() {
        let line = RE.captures(line).unwrap();

        let moves: &str = &line[2];
        let moves: Vec<&str> = moves.split(" ").collect();
        let moves: Vec<Vec<usize>> = moves
            .iter()
            .map(|s| {
                s.trim_matches(&['(', ')'][..])
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let target_joltage: &str = &line[3];
        let target_joltage: &str = &target_joltage[1..target_joltage.len() - 1];
        let target_joltage: Vec<u32> = target_joltage
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        let mut count: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..moves.len() {
            for num in &moves[i] {
                let v = count.entry(*num).or_insert(vec![]);
                v.push(i);
            }
        }

        let mut count: Vec<_> = count.iter().collect();
        count.sort_by_key(|(k, _)| *k);
        let count: Vec<_> = count.iter().map(|(_, v)| *v).collect();

        let solver = Solver::new();
        let mut x: Vec<Int> = vec![];
        for i in 0..moves.len() {
            let xi = Int::fresh_const("x");
            solver.assert(xi.ge(0));
            let v: Vec<_> = moves[i].iter().map(|mv| target_joltage[*mv]).collect();
            let vm = *v.iter().min().unwrap();
            solver.assert(xi.le(vm));
            x.push(xi);
        }

        let z: Vec<_> = count.into_iter().zip(target_joltage).collect();
        for (buttons, target) in z {
            let cv: Vec<_> = buttons.iter().map(|c| &x[*c]).collect();
            let cv2 = Int::add(&cv);
            solver.assert(cv2.eq(target));
        }

        let solutions: Vec<_> = solver
            .solutions(x, true)
            // we use take to ensure that this loop terminates in case there are very many solutions
            .take(2000)
            .collect();

        let solutions: Vec<u64> = solutions
            .iter()
            .map(|solution| solution.iter().map(|v| v.as_i64().unwrap() as u64).sum())
            .collect();

        sum += solutions.iter().min().unwrap();
    }
    println!("(Part 2) Fewest button presses: {}", sum);
}

pub fn day10() {
    println!("---- DAY 10 ----");
    part1();
    part2();
    println!("");
}
