use std::cmp::{max, min};
use std::collections::HashMap;
use std::u64;

fn part1() {
    #[derive(Debug)]
    struct Dir {
        dist: u64,
        x: u64,
        y: u64,
    }

    let _input = include_str!("input.txt");
    let input: Vec<&str> = _input.split("\n").collect();
    let input: Vec<(u64, u64)> = input
        .iter()
        .map(|s| {
            let (a, b) = s.split_once(",").unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect();

    let mut nw = Dir {
        dist: u64::MAX,
        x: 0,
        y: 0,
    };
    let mut sw = Dir {
        dist: u64::MAX,
        x: 0,
        y: 0,
    };
    let mut ne = Dir {
        dist: u64::MAX,
        x: 0,
        y: 0,
    };
    let mut se = Dir {
        dist: u64::MAX,
        x: 0,
        y: 0,
    };
    let mut width: u64 = 0;
    let mut height: u64 = 0;

    for (x, y) in &input {
        width = max(width, *x + 1);
        height = max(height, *y + 1);
    }

    for (x, y) in &input {
        let nw_dist = x + y;
        if nw_dist < nw.dist {
            nw.dist = nw_dist;
            nw.x = *x;
            nw.y = *y;
        }
        let ne_dist = (width - x - 1) + y;
        if ne_dist < ne.dist {
            ne.dist = ne_dist;
            ne.x = *x;
            ne.y = *y;
        }
        let sw_dist = x + (height - y - 1);
        if sw_dist < sw.dist {
            sw.dist = sw_dist;
            sw.x = *x;
            sw.y = *y;
        }
        let se_dist = (width - x - 1) + (height - y - 1);
        if se_dist < se.dist {
            se.dist = se_dist;
            se.x = *x;
            se.y = *y;
        }
    }

    let nw_to_se = (nw.x.abs_diff(se.x) + 1) * (nw.y.abs_diff(se.y) + 1);
    let ne_to_sw = (ne.x.abs_diff(sw.x) + 1) * (ne.y.abs_diff(sw.y) + 1);

    println!(
        "(Part 1) Largest rectangle area: {}",
        max(nw_to_se, ne_to_sw)
    );
}

fn part2() {
    fn flood_fill(start_x: u32, start_y: u32, grid: &mut HashMap<(u32, u32), char>) {
        let mut stack = Vec::new();
        stack.push((start_x, start_y));

        while let Some((x, y)) = stack.pop() {
            // Already filled or blocked
            if grid.contains_key(&(x, y)) {
                continue;
            }

            let mut count = 0;

            // Cardinal neighbors (safe checks)
            if x > 0 && matches!(grid.get(&(x - 1, y)), Some('#' | 'X')) {
                count += 1;
            }
            if matches!(grid.get(&(x + 1, y)), Some('#' | 'X')) {
                count += 1;
            }
            if y > 0 && matches!(grid.get(&(x, y - 1)), Some('#' | 'X')) {
                count += 1;
            }
            if matches!(grid.get(&(x, y + 1)), Some('#' | 'X')) {
                count += 1;
            }

            if count == 0 {
                continue;
            }

            // Fill
            grid.insert((x, y), '*');

            // Push 8 neighbors
            for (nx, ny) in [(x + 1, y), (x, y + 1), (x + 1, y + 1)] {
                stack.push((nx, ny));
            }

            if x > 0 {
                stack.push((x - 1, y));
                if y > 0 {
                    stack.push((x - 1, y - 1));
                }
                stack.push((x - 1, y + 1));
            }

            if y > 0 {
                stack.push((x, y - 1));
                stack.push((x + 1, y - 1));
            }
        }
    }

    fn get_area(p1: (u32, u32), p2: (u32, u32)) -> u64 {
        (p1.0.abs_diff(p2.0) + 1) as u64 * (p1.1.abs_diff(p2.1) + 1) as u64
    }

    let _input = include_str!("input.txt");
    let input: Vec<&str> = _input.split("\n").collect();
    let input: Vec<(u32, u32)> = input
        .iter()
        .map(|s| {
            let (a, b) = s.split_once(",").unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect();

    let max_x = input.iter().map(|(x, _)| x).max().copied().unwrap();
    let max_y = input.iter().map(|(_, y)| y).max().copied().unwrap();

    let mut grid_point = (0, 0);

    let mut grid: HashMap<(u32, u32), char> = HashMap::new();
    for i in 0..input.len() {
        let (x1, y1) = input[i];
        let (x2, y2) = input[(i + 1) % input.len()];

        if x1 == max_x {
            grid_point = (x1 + 1, y1)
        } else if y1 == max_y {
            grid_point = (x1, y1 + 1);
        }

        grid.insert((x1, y1), '#');
        if x1 == x2 {
            let s = min(y1, y2);
            let e = max(y1, y2);
            for y in s + 1..e {
                grid.insert((x1, y), 'X');
            }
        } else if y1 == y2 {
            let s = min(x1, x2);
            let e = max(x1, x2);
            for x in s + 1..e {
                grid.insert((x, y1), 'X');
            }
        }
    }

    flood_fill(grid_point.0, grid_point.1, &mut grid);
    let mut max_area: u64 = 0;
    for i in 0..input.len() {
        'inner: for j in (i + 1)..input.len() {
            let (x1, y1) = input[i];
            let (x2, y2) = input[j];

            let area = get_area(input[i], input[j]);
            if area <= max_area {
                continue;
            }

            //Top row
            for x in min(x1, x2)..=max(x1, x2) {
                if grid.get(&(x, min(y1, y2))) == Some(&'*') {
                    continue 'inner;
                }
            }

            //Bottom row
            for x in min(x1, x2)..=max(x1, x2) {
                if grid.get(&(x, max(y1, y2))) == Some(&'*') {
                    continue 'inner;
                }
            }

            //Left column
            for y in min(y1, y2)..=max(y1, y2) {
                if grid.get(&(min(x1, x2), y)) == Some(&'*') {
                    continue 'inner;
                }
            }

            //Right column
            for y in min(y1, y2)..=max(y1, y2) {
                if grid.get(&(max(x1, x2), y)) == Some(&'*') {
                    continue 'inner;
                }
            }

            max_area = max(max_area, area);
        }
    }

    println!("(Part 2) Largest rectangle area: {}", max_area);
}

pub fn day9() {
    println!("---- DAY 9 ----");
    part1();
    part2();
    println!("");
}
