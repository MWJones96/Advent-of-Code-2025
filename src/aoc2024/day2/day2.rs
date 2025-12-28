#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Inc,
    Dec,
}

fn is_valid_report(report: &Vec<u32>, skips: i32, prev: u32, idx: usize, dir: Dir) -> bool {
    if skips < 0 {
        return false;
    }

    if idx == report.len() {
        return true;
    }

    let cont = match dir {
        Dir::Inc => report[idx] > prev && report[idx] - prev <= 3,
        Dir::Dec => report[idx] < prev && prev - report[idx] <= 3,
    };

    if !cont {
        return is_valid_report(report, skips - 1, prev, idx + 1, dir);
    }

    let mut res = is_valid_report(report, skips, report[idx], idx + 1, dir);

    if idx == 0 {
        res = res
            || match dir {
                Dir::Inc => is_valid_report(report, skips - 1, report[idx + 1] - 1, idx + 1, dir),
                Dir::Dec => is_valid_report(report, skips - 1, report[idx + 1] + 1, idx + 1, dir),
            };
    } else {
        res = res || is_valid_report(report, skips - 1, prev, idx + 1, dir);
    }

    return res;
}

fn part1() {
    let input = include_str!("input.txt");
    let mut safe = 0;

    for line in input.lines() {
        let report: Vec<_> = line.split(" ").collect();
        let report: Vec<u32> = report.iter().map(|n| n.parse::<u32>().unwrap()).collect();
        if is_valid_report(&report, 0, report[0] - 1, 0, Dir::Inc)
            || is_valid_report(&report, 0, report[0] + 1, 0, Dir::Dec)
        {
            safe += 1;
        }
    }
    println!("(Part 1) Number of safe reports: {}", safe);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut safe = 0;
    for line in input.lines() {
        let report: Vec<_> = line.split(" ").collect();
        let report: Vec<u32> = report.iter().map(|n| n.parse::<u32>().unwrap()).collect();
        if is_valid_report(&report, 1, report[0] - 1, 0, Dir::Inc)
            || is_valid_report(&report, 1, report[0] + 1, 0, Dir::Dec)
        {
            safe += 1;
        }
    }
    println!("(Part 2) Number of safe reports: {}", safe);
}

pub fn day2() {
    println!("---- DAY 2 ----");
    part1();
    part2();
    println!("");
}
