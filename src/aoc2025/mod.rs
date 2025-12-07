pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
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
        _ => println!("Invalid day: {}. Expected 1-12.", day),
    }
}

pub fn run_all() {
    println!("**** Advent of Code 2025 ****");
    for day in 1..=12 {
        run_day(day);
    }
}
