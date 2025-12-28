fn part1() {
    enum Dir {
        Inc,
        Dec,
    }
    let input = include_str!("input.txt");
    let mut safe = 0;

    'lines: for line in input.lines() {
        let v: Vec<_> = line.split(" ").collect();
        let v: Vec<u32> = v.iter().map(|n| n.parse::<u32>().unwrap()).collect();
        let dir = if v[1] > v[0] { Dir::Inc } else { Dir::Dec };
        for i in 1..v.len() {
            match dir {
                Dir::Inc => {
                    if v[i] <= v[i - 1] {
                        continue 'lines;
                    }
                }
                Dir::Dec => {
                    if v[i] >= v[i - 1] {
                        continue 'lines;
                    }
                }
            }
            if v[i].abs_diff(v[i - 1]) > 3 {
                continue 'lines;
            }
        }
        safe += 1;
    }
    println!("(Part 1) Number of safe reports: {}", safe);
}

fn part2() {
    let input = include_str!("input.txt");
    println!("(Part 2) Part 2 not implemented yet");
}

pub fn day2() {
    println!("---- DAY 2 ----");
    part1();
    part2();
    println!("");
}
