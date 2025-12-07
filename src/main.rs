mod aoc2024;
mod aoc2025;

#[allow(dead_code, unused_variables, unused_imports)]
fn print_usage() {
    println!("Usage: cargo run -- <year>");
    println!("  year: 2024 | 2025 | all");
}

fn main() {
    let arg = std::env::args().nth(1);
    match arg.as_deref() {
        Some("2024") => {
            aoc2024::run_all();
        }
        Some("2025") => {
            aoc2025::run_all();
        }
        Some("all") => {
            aoc2024::run_all();
            aoc2025::run_all();
        }
        _ => {
            print_usage();
        }
    }
}
