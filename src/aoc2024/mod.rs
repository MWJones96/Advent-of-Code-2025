// Auto-generated skeleton for Advent of Code 2024
// This file exposes day1..day25 modules. Implementations are placeholders.

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn run_day(day: u32) {
    match day {
        1 => day1::day1::day1(),
        2 => day2::day2::day2(),
        3 => day3::day3::day3(),
        4 => day4::day4::day4(),
        5 => day5::day5::day5(),
        6 => day6::day6::day6(),
        7 => day7::day7::day7(),
        8 => day8::day8::day8(),
        9 => day9::day9::day9(),
        10 => day10::day10::day10(),
        11 => day11::day11::day11(),
        12 => day12::day12::day12(),
        13 => day13::day13::day13(),
        14 => day14::day14::day14(),
        15 => day15::day15::day15(),
        16 => day16::day16::day16(),
        17 => day17::day17::day17(),
        18 => day18::day18::day18(),
        19 => day19::day19::day19(),
        20 => day20::day20::day20(),
        21 => day21::day21::day21(),
        22 => day22::day22::day22(),
        23 => day23::day23::day23(),
        24 => day24::day24::day24(),
        25 => day25::day25::day25(),
        _ => println!("Invalid day: {}. Expected 1-25.", day),
    }
}

pub fn run_all() {
    println!("**** Advent of Code 2024 ****");
    for day in 1..=25 {
        run_day(day);
    }
}
