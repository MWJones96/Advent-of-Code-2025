use regex::Regex;

fn part1() {
    let input = include_str!("input.txt");
    let mut stars = 0;
    let mut star_counts = vec![];

    let re = Regex::new(r"^(\d+):$").unwrap();
    let re2 = Regex::new(r"^[.#]+$").unwrap();
    let re3 = Regex::new(r"^(\d+)x(\d+):\s*(\d+(?:\s+\d+)*)$").unwrap();

    let (mut yes, mut maybe) = (0, 0);

    for line in input.lines() {
        if re.is_match(line) {
            stars = 0;
        } else if re2.is_match(line) {
            stars += line.chars().filter(|c| *c == '#').collect::<Vec<_>>().len();
        } else if line == "" {
            star_counts.push(stars);
        } else if let Some(caps) = re3.captures(line) {
            let width: u32 = caps[1].parse().unwrap();
            let height: u32 = caps[2].parse().unwrap();
            let nums: Vec<_> = caps[3]
                .split(" ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            let star_iter: Vec<_> = nums
                .iter()
                .enumerate()
                .map(|(i, &x)| (x as usize) * star_counts[i])
                .collect();
            let total_stars: usize = star_iter.iter().sum();
            let num_presents: u32 = nums.iter().sum();
            let regions = (width / 3) * (height / 3);
            if num_presents <= regions {
                yes += 1;
            } else if total_stars > (width * height) as usize {
            } else {
                maybe += 1;
            }
        }
    }

    println!(
        "(Part 1) Number of regions that can fit presents: {}<=X<={}",
        yes,
        yes + maybe
    );
}

pub fn day12() {
    println!("---- DAY 12 ----");
    part1();
}
