use std::collections::{HashMap, HashSet};

fn part1() {
    fn dfs(line: &str, start: usize, options: &HashSet<&str>) -> bool {
        if start == line.len() {
            return true;
        }

        for end in start..line.len() {
            if options.contains(&line[start..=end]) && dfs(line, end + 1, options) {
                return true;
            }
        }

        false
    }

    let input = include_str!("input.txt");

    let lines = input.lines().collect::<Vec<&str>>();
    let mut options = lines[0]
        .split(",")
        .map(|w| w.trim())
        .collect::<HashSet<&str>>();
    let mut cnt = 0;
    for line in &lines[2..] {
        if dfs(*line, 0, &mut options) {
            cnt += 1;
        }
    }
    println!("(Part 1) Possible designs: {}", cnt);
}

fn part2() {
    fn dfs<'a>(
        line: &'a str,
        start: usize,
        options: &HashSet<&'a str>,
        seen: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if start == line.len() {
            return 1;
        }
        if seen.contains_key(&line[start..]) {
            return *seen.get(&line[start..]).unwrap();
        }
        let mut sum: u64 = 0;
        for end in start..line.len() {
            if options.contains(&line[start..=end]) {
                sum += dfs(line, end + 1, options, seen);
            }
        }

        seen.insert(&line[start..], sum);
        sum
    }

    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut options = lines[0]
        .split(",")
        .map(|w| w.trim())
        .collect::<HashSet<&str>>();
    let mut seen: HashMap<&str, u64> = HashMap::new();
    let mut cnt: u64 = 0;
    for line in &lines[2..] {
        cnt += dfs(line, 0, &options, &mut seen);
    }
    println!("(Part 2) Sum of different design counts: {}", cnt);
}

pub fn day19() {
    println!("---- DAY 19 ----");
    part1();
    part2();
    println!("");
}
