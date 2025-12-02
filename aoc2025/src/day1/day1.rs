use std::fs::read_to_string;

fn part1() {
    let mut state: i32 = 50;
    let mut password: u32 = 0;

    for line in read_to_string("src/day1/input.txt").unwrap().lines() {
        let (side, num) = line.split_at(1);
        let num: i32 = num.parse::<i32>().ok().expect("Expected a number");
        match side {
            "L" => {
                state = (state - num) % 100;
            }
            "R" => {
                state = (state + num) % 100;
            }
            _ => panic!("Invalid input"),
        }
        if state == 0 {
            password += 1;
        }
    }

    println!("(Part 1) Password: {}", password);
}

fn part2() {
    let mut state: i32 = 50;
    let mut password: u32 = 0;

    for line in read_to_string("src/day1/input.txt").unwrap().lines() {
        let (side, num) = line.split_at(1);
        let num: u32 = num.parse::<u32>().ok().expect("Expected a number");
        match side {
            "L" => {
                for _ in 0..num {
                    state = (state - 1) % 100;
                    if state == 0 {
                        password += 1
                    }
                }
            }
            "R" => {
                for _ in 0..num {
                    state = (state + 1) % 100;
                    if state == 0 {
                        password += 1
                    }
                }
            }
            _ => panic!("Invalid input"),
        }
    }

    println!("(Part 2) Password: {}", password);
}

pub fn day1() {
    part1();
    part2();
}
