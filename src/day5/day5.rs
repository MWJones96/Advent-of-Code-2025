fn _merge_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged: Vec<(u64, u64)> = vec![];
    for range in ranges.iter() {
        if merged.len() == 0 {
            merged.push(*range);
            continue;
        }
        let last = merged.last_mut().unwrap();
        if range.0 <= last.1 + 1 {
            if range.1 > last.1 {
                last.1 = range.1;
            }
        } else {
            merged.push(*range);
        }
    }
    merged
}

fn part1() {
    #[derive(Debug)]
    enum ReadType {
        RANGE,
        ID,
    }
    let input = include_str!("input.txt");
    let mut read_type = ReadType::RANGE;
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ids: Vec<u64> = vec![];
    let mut count = 0;

    for line in input.lines() {
        if line.len() == 0 {
            read_type = ReadType::ID;
            continue;
        };
        match read_type {
            ReadType::RANGE => {
                let (start, end) = line.split_once("-").unwrap();
                let start = start.parse::<u64>().unwrap();
                let end = end.parse::<u64>().unwrap();
                ranges.push((start, end));
            }
            ReadType::ID => {
                let _id = line.parse::<u64>().unwrap();
                ids.push(_id);
            }
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let ranges = _merge_ranges(&ranges);
    for id in ids {
        let mut l: i32 = 0;
        let mut r: i32 = ranges.len() as i32 - 1;
        while l <= r {
            let m = (l + r) / 2;
            if id >= ranges[m as usize].0 && id <= ranges[m as usize].1 {
                count += 1;
                break;
            } else if id < ranges[m as usize].0 {
                r = m - 1;
            } else if id > ranges[m as usize].1 {
                l = m + 1;
            }
        }
    }

    println!("(Part 1) Fresh ingredient count: {}", count);
}

fn part2() {
    let _input = include_str!("input.txt");
    // TODO: implement Part 2
    println!("(Part 2) not implemented yet");
}

pub fn day5() {
    println!("---- DAY 5 ----");
    part1();
    part2();
    println!("");
}
