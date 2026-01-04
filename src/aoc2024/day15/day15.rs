use std::fmt;

#[derive(Debug, PartialEq, Eq)]
enum GridElement {
    Wall,
    Space,
    Box,
}

impl fmt::Display for GridElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridElement::Wall => write!(f, "#"),
            GridElement::Space => write!(f, "."),
            GridElement::Box => write!(f, "O"),
        }
    }
}

fn part1() {
    let input = include_str!("input.txt");
    let mut is_reading_grid = true;
    let mut grid: Vec<Vec<GridElement>> = vec![];
    let mut pos: (isize, isize) = (0, 0);
    for (row, line) in input.lines().enumerate() {
        if line == "" {
            is_reading_grid = false;
        }

        if is_reading_grid {
            let mut v: Vec<GridElement> = vec![];
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => v.push(GridElement::Wall),
                    'O' => v.push(GridElement::Box),
                    '.' => v.push(GridElement::Space),
                    '@' => {
                        pos = (row as isize, col as isize);
                        v.push(GridElement::Space);
                    }
                    _ => panic!("Invalid character"),
                }
            }
            grid.push(v);
        } else {
            for ch in line.chars() {
                let dir = match ch {
                    '^' => (-1, 0),
                    '<' => (0, -1),
                    '>' => (0, 1),
                    'v' => (1, 0),
                    _ => panic!("Invalid character"),
                };

                match grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] {
                    GridElement::Wall => {}
                    GridElement::Space => pos = (pos.0 + dir.0, pos.1 + dir.1),
                    GridElement::Box => {
                        let mut lookahead = (pos.0 + dir.0, pos.1 + dir.1);

                        while grid[lookahead.0 as usize][lookahead.1 as usize] == GridElement::Box {
                            lookahead = (lookahead.0 + dir.0, lookahead.1 + dir.1);
                        }
                        match grid[lookahead.0 as usize][lookahead.1 as usize] {
                            GridElement::Wall => {}
                            GridElement::Space => {
                                pos = (pos.0 + dir.0, pos.1 + dir.1);
                                grid[pos.0 as usize][pos.1 as usize] = GridElement::Space;
                                grid[lookahead.0 as usize][lookahead.1 as usize] = GridElement::Box;
                            }
                            GridElement::Box => panic!("Error"),
                        }
                    }
                }
            }
        }
    }
    let mut sum: u64 = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, element) in line.iter().enumerate() {
            if *element == GridElement::Box {
                sum += 100 * row as u64 + col as u64;
            }
        }
    }
    println!("(Part 1) Sum of GPS co-ordinates: {}", sum);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 15) Part 2 not implemented yet");
}

pub fn day15() {
    println!("---- DAY 15 ----");
    part1();
    part2();
    println!("");
}
