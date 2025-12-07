mod aoc2024;
mod aoc2025;

#[allow(dead_code, unused_variables, unused_imports)]
fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- <year>                  Run all days for the year");
    println!("  cargo run -- <year> <day>           Run specific day for the year");
    println!("  cargo run -- all                     Run all days for all years");
    println!();
    println!("Examples:");
    println!("  cargo run -- 2025                    Run all 2025 days");
    println!("  cargo run -- 2025 7                  Run 2025 day 7");
    println!("  cargo run -- 2024 15                 Run 2024 day 15");
    println!("  cargo run -- all                     Run all years");
}

fn main() {
    let mut args = std::env::args();
    args.next(); // Skip program name

    let year_arg = match args.next() {
        Some(y) => y,
        None => {
            print_usage();
            return;
        }
    };

    let day_arg = args.next();

    match year_arg.as_str() {
        "2024" => {
            if let Some(day_str) = day_arg {
                match day_str.parse::<u32>() {
                    Ok(day) => aoc2024::run_day(day),
                    Err(_) => {
                        eprintln!("Invalid day: '{}'", day_str);
                        print_usage();
                    }
                }
            } else {
                aoc2024::run_all();
            }
        }
        "2025" => {
            if let Some(day_str) = day_arg {
                match day_str.parse::<u32>() {
                    Ok(day) => aoc2025::run_day(day),
                    Err(_) => {
                        eprintln!("Invalid day: '{}'", day_str);
                        print_usage();
                    }
                }
            } else {
                aoc2025::run_all();
            }
        }
        "all" => {
            if day_arg.is_some() {
                eprintln!("Cannot specify day with 'all' year");
                print_usage();
            } else {
                aoc2024::run_all();
                aoc2025::run_all();
            }
        }
        _ => {
            eprintln!("Invalid year: '{}'", year_arg);
            print_usage();
        }
    }
}
