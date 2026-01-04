use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum GridElement {
    Wall,
    Space,
    Box,
    LeftBox,
    RightBox,
}

impl fmt::Display for GridElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridElement::Wall => write!(f, "#"),
            GridElement::Space => write!(f, "."),
            GridElement::Box => write!(f, "O"),
            GridElement::LeftBox => write!(f, "["),
            GridElement::RightBox => write!(f, "]"),
        }
    }
}

fn push(grid: &mut Vec<Vec<GridElement>>, pos: (isize, isize), dir: (isize, isize)) -> bool {
    match grid[pos.0 as usize][pos.1 as usize] {
        GridElement::Wall => false,
        GridElement::Space => true,
        _ => {
            let can_push = push(grid, (pos.0 + dir.0, pos.1 + dir.1), dir);
            if can_push {
                grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] =
                    grid[pos.0 as usize][pos.1 as usize];
                grid[pos.0 as usize][pos.1 as usize] = GridElement::Space;
            }
            can_push
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

                if push(&mut grid, (pos.0 + dir.0, pos.1 + dir.1), dir) {
                    pos = (pos.0 + dir.0, pos.1 + dir.1)
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
                    '#' => {
                        v.push(GridElement::Wall);
                        v.push(GridElement::Wall);
                    }
                    'O' => {
                        v.push(GridElement::LeftBox);
                        v.push(GridElement::RightBox);
                    }
                    '.' => {
                        v.push(GridElement::Space);
                        v.push(GridElement::Space);
                    }
                    '@' => {
                        pos = (row as isize, col as isize);
                        v.push(GridElement::Space);
                        v.push(GridElement::Space);
                    }
                    _ => panic!("Invalid character"),
                }
            }
            grid.push(v);
        } else {
        }
    }

    for (row, line) in grid.iter().enumerate() {
        for (col, element) in line.iter().enumerate() {
            print!("{}", element);
        }
        println!();
    }

    println!("(Part 2) Part 2 not implemented yet");
}

pub fn day15() {
    println!("---- DAY 15 ----");
    part1();
    part2();
    println!("");
}
