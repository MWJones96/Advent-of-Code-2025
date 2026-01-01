fn part1() {
    let input = include_str!("input.txt");
    let line = input.lines().collect::<Vec<&str>>()[0];
    let mut is_file: bool = true;
    let mut hard_drive: Vec<i16> = vec![];
    let mut id = 0;
    let mut idx = 0;
    let mut file_idx: Vec<usize> = vec![];
    let mut free_idx: Vec<usize> = vec![];

    for n in line.chars().map(|c| c.to_digit(10).unwrap()) {
        if is_file {
            hard_drive.extend(std::iter::repeat(id).take(n as usize));
            for i in idx..idx + n {
                file_idx.push(i as usize);
            }
            id += 1;
        } else {
            hard_drive.extend(std::iter::repeat(-1).take(n as usize));
            for i in idx..idx + n {
                free_idx.push(i as usize);
            }
        }
        idx += n;
        is_file = !is_file;
    }

    let mut l: usize = 0;
    let mut r: usize = file_idx.len() - 1;

    while free_idx[l] < file_idx[r] {
        let free_hd = free_idx[l];
        let file_hd = file_idx[r];

        hard_drive[free_hd] = hard_drive[file_hd];
        hard_drive[file_hd] = -1;

        l += 1;
        r -= 1;
    }

    let mut checksum: u64 = 0;
    for (idx, val) in hard_drive.iter().enumerate() {
        if *val == -1 {
            break;
        }
        checksum += *val as u64 * idx as u64;
    }

    println!("(Part 1) Checksum: {}", checksum);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 9) Part 2 not implemented yet");
}

pub fn day9() {
    println!("---- 2024 DAY 9 ----");
    part1();
    part2();
    println!("");
}
