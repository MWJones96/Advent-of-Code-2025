use std::arch::x86_64;

fn part1() {
    let input = include_str!("input.txt");
    let input = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;

    for row in 0..input.len() {
        for col in 0..input[row].chars().count() {
            let c = input[row].chars().nth(col).unwrap();
            if c == '@' {
                let mut neighbour_count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let x = col as isize + dx;
                        let y = row as isize + dy;

                        if x < 0
                            || y < 0
                            || x >= input.len() as isize
                            || y >= input[0].len() as isize
                        {
                            continue;
                        }

                        if input[y as usize].chars().nth(x as usize).unwrap() == '@' {
                            neighbour_count += 1;
                        }
                    }
                }
                if neighbour_count < 4 {
                    sum += 1;
                }
            }
        }
    }
    println!("(Part 1) Rolls of paper accessible: {}", sum);
}

fn part2() {
    let input = include_str!("input.txt");
    let input = input.lines().collect::<Vec<&str>>();
    println!("(Part 2) not implemented yet");
}

pub fn day4() {
    println!("---- DAY 4 ----");
    part1();
    part2();
    println!("");
}
