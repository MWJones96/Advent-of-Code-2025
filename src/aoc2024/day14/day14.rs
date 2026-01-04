use std::collections::HashMap;

struct Robot {
    row: i16,
    col: i16,

    d_row: i16,
    d_col: i16,
}

fn contains_no_robot_overlap(width: i16, height: i16, robots: &Vec<Robot>) -> bool {
    let mut grid: Vec<Vec<_>> = vec![vec!['.'; width as usize]; height as usize];

    for r in robots {
        if grid[r.row as usize][r.col as usize] == '*' {
            return false;
        }
        grid[r.row as usize][r.col as usize] = '*';
    }

    return true;
}

fn part1() {
    let input = include_str!("input.txt");
    let mut width = 0;
    let mut height = 0;

    let mut robots: HashMap<(u8, u8), u8> = HashMap::new();
    for line in input.lines() {
        let robot = line.split_once(" ").unwrap();
        let pos = robot.0.split("=").collect::<Vec<_>>()[1];
        let pos: Vec<_> = pos.split(",").map(|c| c.parse::<i16>().unwrap()).collect();
        let (col, row) = (pos[0], pos[1]);
        height = height.max(row + 1);
        width = width.max(col + 1);
    }

    for line in input.lines() {
        let robot = line.split_once(" ").unwrap();
        let pos = robot.0.split("=").collect::<Vec<_>>()[1];
        let pos: Vec<_> = pos.split(",").map(|c| c.parse::<i16>().unwrap()).collect();
        let (mut col, mut row) = (pos[0], pos[1]);

        let v = robot.1.split("=").collect::<Vec<_>>()[1];
        let v: Vec<_> = v.split(",").map(|c| c.parse::<i16>().unwrap()).collect();
        let (d_col, d_row) = (v[0], v[1]);

        for _ in 0..100 {
            row += d_row;
            row = row.rem_euclid(height);

            col += d_col;
            col = col.rem_euclid(width);
        }

        *robots.entry((row as u8, col as u8)).or_default() += 1;
    }

    let mut quad_tl = 0;
    let mut quad_tr = 0;
    let mut quad_bl = 0;
    let mut quad_br = 0;

    for row in 0..height / 2 {
        for col in 0..width / 2 {
            quad_tl += *robots.entry((row as u8, col as u8)).or_default() as u64;
        }
    }

    for row in 0..height / 2 {
        for col in (width / 2 + 1)..width {
            quad_tr += *robots.entry((row as u8, col as u8)).or_default() as u64;
        }
    }

    for row in (height / 2 + 1)..height {
        for col in 0..width / 2 {
            quad_bl += *robots.entry((row as u8, col as u8)).or_default() as u64;
        }
    }

    for row in (height / 2 + 1)..height {
        for col in (width / 2 + 1)..width {
            quad_br += *robots.entry((row as u8, col as u8)).or_default() as u64;
        }
    }

    println!(
        "(Part 1) Safety factor: {}",
        quad_tl * quad_tr * quad_bl * quad_br
    );
}

fn part2() {
    let input = include_str!("input.txt");
    let mut width = 0;
    let mut height = 0;

    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let robot = line.split_once(" ").unwrap();
        let pos = robot.0.split("=").collect::<Vec<_>>()[1];
        let pos: Vec<_> = pos.split(",").map(|c| c.parse::<i16>().unwrap()).collect();
        let (col, row) = (pos[0], pos[1]);

        let v = robot.1.split("=").collect::<Vec<_>>()[1];
        let v: Vec<_> = v.split(",").map(|c| c.parse::<i16>().unwrap()).collect();
        let (d_col, d_row) = (v[0], v[1]);

        robots.push(Robot {
            row,
            col,
            d_row,
            d_col,
        });

        height = height.max(row + 1);
        width = width.max(col + 1);
    }

    let mut seconds = 0;

    while !contains_no_robot_overlap(width, height, &robots) {
        for r in &mut robots {
            r.row += r.d_row;
            r.row = r.row.rem_euclid(height);

            r.col += r.d_col;
            r.col = r.col.rem_euclid(width);
        }
        seconds += 1;
    }
    println!(
        "(Part 2): Minimum seconds elapsed for Easter Egg: {}",
        seconds
    );
}

pub fn day14() {
    println!("---- DAY 14 ----");
    part1();
    part2();
    println!("");
}
