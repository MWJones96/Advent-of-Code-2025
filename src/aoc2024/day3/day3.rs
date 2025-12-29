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
    #[derive(Debug)]
    enum Event {
        Do,
        DoNot,
        Mul(u64),
    }

    let input = include_str!("input.txt");
    let re_mul = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don\'t\(\)").unwrap();

    let mut events: Vec<(usize, Event)> = vec![];
    let mut sum: u64 = 0;
    let mut line_off: usize = 0;
    for line in input.lines() {
        for capture in re_mul.find_iter(line) {
            let str = &capture.as_str()[4..capture.len() - 1];
            let (x, y) = str.split_once(",").unwrap();

            events.push((
                line_off + capture.start(),
                Event::Mul(x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap()),
            ));
        }
        for capture in re_dont.find_iter(line) {
            events.push((line_off + capture.start(), Event::DoNot));
        }
        for capture in re_do.find_iter(line) {
            events.push((line_off + capture.start(), Event::Do));
        }
        line_off += line.len();
    }

    events.sort_by_key(|key| key.0);
    let mut enabled = true;
    for (_, event) in events {
        match event {
            Event::Do => enabled = true,
            Event::DoNot => enabled = false,
            Event::Mul(mult) => {
                if enabled {
                    sum += mult;
                }
            }
        }
    }

    println!("(Part 2) Sum of enabled multiplications: {}", sum);
}

pub fn day3() {
    println!("---- DAY 3 ----");
    part1();
    part2();
    println!("");
}
