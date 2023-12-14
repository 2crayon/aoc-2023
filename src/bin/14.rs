#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Round,
    Solid,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            'O' => Cell::Round,
            '#' => Cell::Solid,
            _ => panic!("Invalid cell char: {}", c),
        }
    }
}

fn display(cells: &[Vec<Cell>]) {
    for row in cells {
        for cell in row {
            match cell {
                Cell::Empty => print!("."),
                Cell::Round => print!("O"),
                Cell::Solid => print!("#"),
            }
        }
        println!();
    }
}

fn move_up(grid: &mut [Vec<Cell>], i: usize, j: usize) {
    if i == 0 {
        return;
    }

    if grid[i - 1][j] == Cell::Empty {
        grid[i - 1][j] = Cell::Round;
        grid[i][j] = Cell::Empty;
        move_up(grid, i - 1, j);
    }
}

fn score(grid: &[Vec<Cell>]) -> usize {
    let mut score = 0;

    for (i, row) in grid.iter().enumerate() {
        for cell in row {
            if *cell == Cell::Round {
                score += 1 + i;
            }
        }
    }

    score
}

fn main() {
    let input = include_str!("../../input.txt");

    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| Cell::from_char(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Cell::Round {
                move_up(&mut grid, i, j);
            }
        }
    }

    grid.reverse();

    let sln = score(&grid);

    display(&grid);

    println!("Solution: {}", sln);
}
