use std::collections::HashSet;

fn part1() {
    let mut sum: u64 = 0;
    let input = include_str!("input.txt").split(",");
    for range in input {
        let (start, end) = range.split_once("-").unwrap();
        let mut start_u64 = 0;
        let mut end_u64 = 0;

        if start.len() % 2 == 0 {
            let start_outer = &start[0..start.len() / 2];
            let start_inner = &start[start.len() / 2..];

            let start_outer = start_outer.parse::<u64>().unwrap();
            let start_inner = start_inner.parse::<u64>().unwrap();

            start_u64 = if start_outer < start_inner {
                start_outer + 1
            } else {
                start_outer
            }
        } else {
            let start = format!("1{}", "0".repeat(start.len()));
            let start = &start[0..start.len() / 2];
            start_u64 = start.parse::<u64>().unwrap();
        }

        if end.len() % 2 == 0 {
            let end_outer = &end[0..end.len() / 2];
            let end_inner = &end[end.len() / 2..];

            let end_outer = end_outer.parse::<u64>().unwrap();
            let end_inner = end_inner.parse::<u64>().unwrap();

            end_u64 = if end_inner < end_outer {
                end_outer - 1
            } else {
                end_outer
            }
        } else {
            let end = format!("{}", "9".repeat(end.len() - 1));
            let end = &end[0..end.len() / 2];
            end_u64 = end.parse::<u64>().unwrap();
        }

        for i in start_u64..=end_u64 {
            let num = i * 10u64.pow(format!("{}", i).len() as u32) + i;
            sum += num;
        }
    }

    println!("(Day 2) (Part 1) Sum of invalid IDs: {}", sum);
}

fn part2() {
    fn factorise(n: u64) -> Vec<(u64, u64)> {
        let mut factors: Vec<(u64, u64)> = Vec::new();
        factors.push((1, n));
        for i in 2..n {
            if n % i == 0 {
                factors.push((i, n / i));
            }
        }

        factors
    }

    fn duplicate_digits(num: u64, n: usize) -> u64 {
        let s = num.to_string().repeat(n);
        s.parse().unwrap()
    }

    let mut set: HashSet<u64> = HashSet::new();
    let mut sum = 0;
    let input = include_str!("input.txt").split(",");
    for range in input {
        let (start, end) = range.split_once("-").unwrap();
        let start_digits = start.len();
        let end_digits = end.len();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        for digits in (start_digits..=end_digits) {
            if digits == 1 {
                continue;
            }

            let factors = factorise(digits as u64);
            for (pow, count) in factors {
                let lower_bound = 10u64.pow((pow - 1) as u32);
                let upper_bound = 10u64.pow(pow as u32);

                for i in lower_bound..upper_bound {
                    let num = duplicate_digits(i, count as usize);
                    if !set.contains(&num) && num >= start && num <= end {
                        set.insert(num);
                        sum += num;
                    }
                }
            }
        }
    }

    println!("(Day 2) (Part 2) Sum of invalid IDs: {}", sum);
}

pub fn day2() {
    part1();
    part2();
}
