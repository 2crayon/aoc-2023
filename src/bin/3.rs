// doesn't work

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Digit(char),
    Symbol,
    Infected,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            c if c.is_digit(10) => Cell::Digit(c),
            '.' => Cell::Empty,
            _ => Cell::Symbol,
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| Cell::from_char(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                Cell::Symbol => {
                    infect(&mut grid, i, j + 1);
                    infect(&mut grid, i, j - 1);
                    infect(&mut grid, i + 1, j);
                    infect(&mut grid, i - 1, j);
                    infect(&mut grid, i + 1, j + 1);
                    infect(&mut grid, i + 1, j - 1);
                    infect(&mut grid, i - 1, j + 1);
                    infect(&mut grid, i - 1, j - 1);

                    grid[i][j] = Cell::Empty;
                }
                _ => continue,
            }
        }
    }

    let mut starting_points = vec![];
    let mut on_num = false;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                Cell::Digit(_) | Cell::Infected => {
                    if !on_num {
                        starting_points.push((i, j));
                        on_num = true;
                    }
                }
                _ => on_num = false,
            }
        }
    }

    let mut sum = 0;

    for (i, j) in starting_points.clone() {
        let row = &grid[i];
        let mut infected = false;
        let mut num = String::new();

        for k in j..row.len() {
            match row[k] {
                Cell::Digit(d) => {
                    num.push(d);

                    if k == row.len() - 1 && !infected {
                        sum += num.parse::<i32>().unwrap()
                    }
                }
                Cell::Infected => {
                    infected = true;
                    break;
                }
                Cell::Empty => sum += num.parse::<i32>().unwrap(),
                _ => panic!(),
            }
        }
    }

    println!("{:#?}", sum);
}

fn infect(grid: &mut Vec<Vec<Cell>>, i: usize, j: usize) {
    match grid.get_mut(i) {
        None => return,
        Some(row) => match row.get_mut(j) {
            None => return,
            Some(cell) => match cell {
                Cell::Digit(_) => *cell = Cell::Infected,
                _ => return,
            },
        },
    }
}
