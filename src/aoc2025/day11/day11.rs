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

fn dp2<'a>(
    start: &'a str,
    need_fft: bool,
    need_dac: bool,
    seen: &mut HashMap<(&'a str, bool, bool), u64>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
) -> u64 {
    if start == "out" {
        if !need_fft && !need_dac {
            return 1;
        }
        return 0;
    }
    if seen.contains_key(&(start, need_fft, need_dac)) {
        return *seen.get(&(start, need_fft, need_dac)).unwrap();
    }
    let sum: Vec<u64> = graph
        .get(start)
        .unwrap()
        .iter()
        .map(|n| {
            dp2(
                *n,
                if start == "fft" { false } else { need_fft },
                if start == "dac" { false } else { need_dac },
                seen,
                graph,
            )
        })
        .collect();
    let sum: u64 = sum.iter().sum();
    seen.insert((start, need_fft, need_dac), sum);

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
    let input = include_str!("input.txt");
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (node, next) = line.split_once(':').unwrap();
        let next = next.trim();
        let next: Vec<_> = next.split_whitespace().collect();
        graph.insert(node, next);
    }
    let mut seen: HashMap<(&str, bool, bool), u64> = HashMap::new();

    println!(
        "(Part 2) FFT and DAC paths: {}",
        dp2("svr", true, true, &mut seen, &graph)
    );
}

pub fn day11() {
    println!("---- DAY 11 ----");
    part1();
    part2();
    println!("");
}
