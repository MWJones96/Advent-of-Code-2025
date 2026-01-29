use std::collections::HashSet;

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
    let _input = include_str!("input.txt");
    println!("(2024 Day 19) Part 2 not implemented yet");
}

pub fn day19() {
    println!("---- DAY 19 ----");
    part1();
    part2();
    println!("");
}
