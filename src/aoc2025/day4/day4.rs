fn _get_accessible_rolls(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut acc = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c == '@' {
                let mut neighbour_count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let x = col as isize + dx;
                        let y = row as isize + dy;

                        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize
                        {
                            continue;
                        }

                        if grid[y as usize][x as usize] == '@' {
                            neighbour_count += 1;
                        }
                    }
                }
                if neighbour_count < 4 {
                    acc.push((row, col));
                }
            }
        }
    }
    acc
}

fn part1() {
    let input = include_str!("input.txt");
    let input = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let acc = _get_accessible_rolls(&input);
    println!("(Part 1) Rolls of paper accessible: {}", acc.len());
}

fn part2() {
    let input = include_str!("input.txt");
    let mut input = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    loop {
        let acc = _get_accessible_rolls(&input);
        if acc.len() == 0 {
            break;
        }
        sum += acc.len();
        for (row, col) in acc {
            input[row][col] = '.';
        }
    }

    println!("(Part 2) Rolls of paper removed: {}", sum);
}

pub fn day4() {
    println!("---- DAY 4 ----");
    part1();
    part2();
    println!("");
}
