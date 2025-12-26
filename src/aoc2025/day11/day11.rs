use std::collections::HashMap;

fn dp<'a>(
    start: &'a str,
    seen: &mut HashMap<&'a str, u32>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
) -> u32 {
    if start == "out" {
        return 1;
    }

    if seen.contains_key(start) {
        return *seen.get(start).unwrap();
    }

    let sum: Vec<u32> = graph
        .get(start)
        .unwrap()
        .iter()
        .map(|n| dp(*n, seen, graph))
        .collect();
    let sum: u32 = sum.iter().sum();
    seen.insert(start, sum);

    sum
}

fn part1() {
    let input = include_str!("input.txt");
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (node, next) = line.split_once(':').unwrap();
        let next = next.trim();
        let next: Vec<_> = next.split_whitespace().collect();
        graph.insert(node, next);
    }
    let mut seen = HashMap::new();
    println!(
        "(Part 1) Number of paths to output: {}",
        dp("you", &mut seen, &graph)
    );
}

fn part2() {
    let _input = include_str!("input.txt");
    // TODO: implement Part 2
    println!("(Part 2) not implemented yet");
}

pub fn day11() {
    println!("---- DAY 11 ----");
    part1();
    part2();
    println!("");
}
