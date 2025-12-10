use std::cmp::max;
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
    let _input = include_str!("input.txt");
    println!("(Part 2) not implemented yet");
}

pub fn day9() {
    println!("---- DAY 9 ----");
    part1();
    part2();
    println!("");
}
