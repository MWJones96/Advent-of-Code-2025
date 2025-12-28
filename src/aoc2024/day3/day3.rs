use regex::Regex;

fn part1() {
    let input = include_str!("input.txt");
    let re = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let mut sum: u64 = 0;
    for line in input.lines() {
        for capture in re.find_iter(line) {
            let str = &capture.as_str()[4..capture.len() - 1];
            let (x, y) = str.split_once(",").unwrap();

            sum += x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap();
        }
    }

    println!("(Part 1) Sum of multiplications: {}", sum);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(Part 2) Part 2 not implemented yet");
}

pub fn day3() {
    println!("---- DAY 3 ----");
    part1();
    part2();
    println!("");
}
