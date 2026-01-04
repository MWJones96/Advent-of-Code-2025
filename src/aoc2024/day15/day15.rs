use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum GridElement {
    Wall,
    Space,
    Box,
    LeftBox,
    RightBox,
}

#[derive(Copy, Clone)]
enum Polarity {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone)]
struct Direction {
    row_diff: i8,
    col_diff: i8,
    polarity: Polarity,
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

fn can_push(grid: &mut Vec<Vec<GridElement>>, pos: (isize, isize), dir: Direction) -> bool {
    match grid[pos.0 as usize][pos.1 as usize] {
        GridElement::Wall => false,
        GridElement::Space => true,
        GridElement::LeftBox | GridElement::RightBox => {
            let (row, col) = pos;

            match dir.polarity {
                Polarity::Horizontal => {
                    //L-R
                    let next_c = col + dir.col_diff as isize;
                    let can_push = can_push(grid, (row, next_c), dir);
                    can_push
                }
                Polarity::Vertical => {
                    //T-B
                    let offset = if grid[row as usize][col as usize] == GridElement::LeftBox {
                        1
                    } else {
                        -1
                    };
                    let next_row = row + dir.row_diff as isize;
                    let can_push = can_push(grid, (next_row, col), dir)
                        && can_push(grid, (next_row, col + offset), dir);
                    can_push
                }
            }
        }
        GridElement::Box => panic!("Full Box not in Part 2"),
    }
}

fn push2(grid: &mut Vec<Vec<GridElement>>, pos: (isize, isize), dir: Direction) {
    match grid[pos.0 as usize][pos.1 as usize] {
        GridElement::Wall => {}
        GridElement::Space => {}
        GridElement::LeftBox | GridElement::RightBox => {
            let (row, col) = pos;

            match dir.polarity {
                Polarity::Horizontal => {
                    //L-R
                    let next_c = col + dir.col_diff as isize;
                    push2(grid, (row, next_c), dir);
                    grid[row as usize][next_c as usize] = grid[row as usize][col as usize];
                    grid[row as usize][col as usize] = GridElement::Space;
                }
                Polarity::Vertical => {
                    //T-B
                    let offset = if grid[row as usize][col as usize] == GridElement::LeftBox {
                        1
                    } else {
                        -1
                    };
                    let next_row = row + dir.row_diff as isize;
                    push2(grid, (next_row, col), dir);
                    push2(grid, (next_row, col + offset), dir);
                    grid[next_row as usize][col as usize] = grid[row as usize][col as usize];
                    grid[next_row as usize][(col + offset) as usize] =
                        grid[row as usize][(col + offset) as usize];

                    grid[row as usize][col as usize] = GridElement::Space;
                    grid[row as usize][(col + offset) as usize] = GridElement::Space;
                }
            }
        }
        GridElement::Box => panic!("Full Box not in Part 2"),
    }
}

fn push(grid: &mut Vec<Vec<GridElement>>, pos: (isize, isize), dir: (isize, isize)) -> bool {
    match grid[pos.0 as usize][pos.1 as usize] {
        GridElement::Wall => false,
        GridElement::Space => true,
        GridElement::Box => {
            let can_push = push(grid, (pos.0 + dir.0, pos.1 + dir.1), dir);
            if can_push {
                grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] =
                    grid[pos.0 as usize][pos.1 as usize];
                grid[pos.0 as usize][pos.1 as usize] = GridElement::Space;
            }
            can_push
        }
        _ => panic!("Left and Right Box not in Part 1"),
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
                        pos = (row as isize, (col * 2) as isize);
                        v.push(GridElement::Space);
                        v.push(GridElement::Space);
                    }
                    _ => panic!("Invalid character"),
                }
            }
            grid.push(v);
        } else {
            for ch in line.chars() {
                let (row, col) = pos;
                let dir = match ch {
                    '^' => Direction {
                        row_diff: -1,
                        col_diff: 0,
                        polarity: Polarity::Vertical,
                    },
                    '<' => Direction {
                        row_diff: 0,
                        col_diff: -1,
                        polarity: Polarity::Horizontal,
                    },
                    '>' => Direction {
                        row_diff: 0,
                        col_diff: 1,
                        polarity: Polarity::Horizontal,
                    },
                    'v' => Direction {
                        row_diff: 1,
                        col_diff: 0,
                        polarity: Polarity::Vertical,
                    },
                    _ => panic!("Invalid character"),
                };

                if can_push(
                    &mut grid,
                    (row + dir.row_diff as isize, col + dir.col_diff as isize),
                    dir,
                ) {
                    push2(
                        &mut grid,
                        (row + dir.row_diff as isize, col + dir.col_diff as isize),
                        dir,
                    );
                    pos = (row + dir.row_diff as isize, col + dir.col_diff as isize);
                }
            }
        }
    }

    let mut sum: u64 = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, element) in line.iter().enumerate() {
            if *element == GridElement::LeftBox {
                sum += 100 * row as u64 + col as u64;
            }
        }
    }

    println!("(Part 2) Sum of GPS co-ordinates: {}", sum);
}

pub fn day15() {
    println!("---- DAY 15 ----");
    part1();
    part2();
    println!("");
}
