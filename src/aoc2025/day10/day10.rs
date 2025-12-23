use std::collections::{HashSet, VecDeque};

use regex::Regex;

fn part1() {
    let input = include_str!("input.txt");
    let re = Regex::new(r"^(\[[^\]]+\])\s+((?:\([^)]+\)\s*)+)\s+(\{[^}]+\})$").unwrap();
    let mut sum = 0;

    for line in input.split("\n") {
        let line = re.captures(line).unwrap();
        let target_state: &str = &line[1][1..line[1].len() - 1];
        let target_state: Vec<bool> = target_state
            .chars()
            .map(|c| if c == '#' { true } else { false })
            .collect();

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
    let _input = include_str!("input.txt");
    // TODO: implement Part 2
    println!("(Part 2) not implemented yet");
}

pub fn day10() {
    println!("---- DAY 10 ----");
    part1();
    part2();
    println!("");
}
