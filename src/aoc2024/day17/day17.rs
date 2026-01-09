use std::collections::HashSet;
use std::u64;

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
            *a >>= combo_operand(operand, *a, *b, *c);
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
            *b = *a >> combo_operand(operand, *a, *b, *c);
            *ip += 2;
            None
        }
        7 => {
            *c = *a >> combo_operand(operand, *a, *b, *c);
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

#[derive(Debug)]
struct Matches {
    prefixes: HashSet<u8>,
    matches: HashSet<u16>,
    suffixes: HashSet<u8>,
}

impl Matches {
    fn new() -> Self {
        Self {
            prefixes: HashSet::new(),
            matches: HashSet::new(),
            suffixes: HashSet::new(),
        }
    }
}

fn prune(tree: &mut Vec<Matches>) {
    let tree_len = tree.len();

    let mut new_tree: Vec<Matches> = vec![];

    {
        let curr = &tree[0];
        let next = &tree[1];

        let mut new_matches: HashSet<u16> = HashSet::new();
        let mut new_prefixes: HashSet<u8> = HashSet::new();

        for node in &curr.matches {
            let prefix = ((node & 0b1_111_111_000) >> 3) as u8;
            if next.suffixes.contains(&prefix) {
                new_matches.insert(*node);
                new_prefixes.insert(prefix);
            }
        }

        new_tree.push(Matches {
            prefixes: new_prefixes,
            matches: new_matches,
            suffixes: HashSet::new(),
        });
    }

    for i in 1..(tree_len - 1) {
        let curr = &tree[i];
        let prev = &tree[i - 1];
        let next = &tree[i + 1];

        let mut new_matches: HashSet<u16> = HashSet::new();
        let mut new_prefixes: HashSet<u8> = HashSet::new();
        let mut new_suffixes: HashSet<u8> = HashSet::new();

        for node in &curr.matches {
            let prefix = ((node & 0b1_111_111_000) >> 3) as u8;
            let suffix = (node & 0b0_001_111_111) as u8;
            if prev.prefixes.contains(&suffix) && next.suffixes.contains(&prefix) {
                new_matches.insert(*node);
                new_prefixes.insert(prefix);
                new_suffixes.insert(suffix);
            }
        }

        new_tree.push(Matches {
            prefixes: new_prefixes,
            matches: new_matches,
            suffixes: new_suffixes,
        });
    }

    {
        let curr = &tree[tree_len - 1];
        let prev = &tree[tree_len - 2];

        let mut new_matches: HashSet<u16> = HashSet::new();
        let mut new_suffixes: HashSet<u8> = HashSet::new();

        for node in &curr.matches {
            let suffix = (node & 0b0_001_111_111) as u8;
            if prev.prefixes.contains(&suffix) {
                new_matches.insert(*node);
                new_suffixes.insert(suffix);
            }
        }

        new_tree.push(Matches {
            prefixes: HashSet::new(),
            matches: new_matches,
            suffixes: new_suffixes,
        });
    }

    *tree = new_tree;
}

fn dfs(tree: &Vec<Matches>, idx: usize, prev: u16, min_val: &mut u64) -> Option<u64> {
    if idx == tree.len() {
        return Some(0);
    }

    for node in &tree[idx].matches {
        let prev_prefix = (prev & 0b1_111_111_000) >> 3;
        let node_suffix = node & 0b0_001_111_111;

        if idx > 0 && prev_prefix != node_suffix {
            continue;
        }

        if let Some(val) = dfs(tree, idx + 1, *node, &mut u64::MAX) {
            let mut val = val << 3;
            val |= (node & 7) as u64;

            *min_val = val.min(*min_val);

            return Some(*min_val);
        }
    }

    None
}

fn part2() {
    /*
    BST 4, B = A & 111
    BXL 3, B = B ^ 3
    CDV 5, C = A >> B
    ADV 3, A = A >> 3
    BXL 5, B = B ^ 5
    BXC 4, B = B ^ C
    OUT 5, PRINT B
    JNZ 0 while A != 0

    do {
        b = a & 7;
        b = b ^ 3;
        c = a >> b;
        a = a >> 3;
        b = b ^ 5;
        b = b ^ c;
        print(b);
    } while (a != 0);

    */
    let input = include_str!("input.txt");
    let input = input.lines().collect::<Vec<&str>>();

    let program: Vec<&str> = input[4]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(",")
        .collect();
    let program: Vec<u8> = program.iter().map(|s| s.parse::<u8>().unwrap()).collect();

    let mut tree: Vec<Matches> = vec![];
    let mut lim = 0b1_111_111_111;
    for i in 0..program.clone().len() {
        let num = program[i];
        if i == program.len() - 1 {
            lim = 0b111;
        } else if i == program.len() - 2 {
            lim = 0b111_111;
        } else if i == program.len() - 3 {
            lim = 0b111_111_111;
        }
        let mut matches = Matches::new();
        for i in 0b0_000_000_000..=lim {
            let a: u16 = i;
            let mut b = a & 7;
            b = b ^ 3;
            let c = (a >> b) & 7;
            b = b ^ 5;
            b = b ^ c;
            if b as u8 == num {
                matches.matches.insert(i);
                matches.prefixes.insert(((i & 0b1_111_111_000) >> 3) as u8);
                matches.suffixes.insert((i & 0b0_001_111_111) as u8);
            }
        }
        tree.push(matches);
    }

    for _ in 0..100 {
        prune(&mut tree);
    }

    let mut min_val = u64::MAX;
    dfs(&tree, 0, 0, &mut min_val);
    println!(
        "(Part 2) Minimum value of A to output copy of itself: {}",
        min_val
    )
}

pub fn day17() {
    println!("---- DAY 17 ----");
    part1();
    part2();
    println!("");
}
