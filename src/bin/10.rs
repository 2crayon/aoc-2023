use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = include_str!("../../input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let s = find_s(&grid);

    let mut pos = (s.0, s.1 + 1);
    let mut steps = 1;
    let mut dir = Dir::Right;

    while grid[pos.0][pos.1] != 'S' {
        match dir {
            Dir::Down => match &grid[pos.0][pos.1] {
                'J' => turn(Dir::Left, &mut dir, &mut pos, &mut steps),
                'L' => turn(Dir::Right, &mut dir, &mut pos, &mut steps),
                '|' => turn(Dir::Down, &mut dir, &mut pos, &mut steps),
                _ => panic!(),
            },
            Dir::Up => match &grid[pos.0][pos.1] {
                '7' => turn(Dir::Left, &mut dir, &mut pos, &mut steps),
                'F' => turn(Dir::Right, &mut dir, &mut pos, &mut steps),
                '|' => turn(Dir::Up, &mut dir, &mut pos, &mut steps),
                _ => panic!(),
            },
            Dir::Left => match &grid[pos.0][pos.1] {
                'L' => turn(Dir::Up, &mut dir, &mut pos, &mut steps),
                'F' => turn(Dir::Down, &mut dir, &mut pos, &mut steps),
                '-' => turn(Dir::Left, &mut dir, &mut pos, &mut steps),
                _ => panic!(),
            },
            Dir::Right => match &grid[pos.0][pos.1] {
                'J' => turn(Dir::Up, &mut dir, &mut pos, &mut steps),
                '7' => turn(Dir::Down, &mut dir, &mut pos, &mut steps),
                '-' => turn(Dir::Right, &mut dir, &mut pos, &mut steps),
                _ => panic!(),
            },
        }
    }

    println!("{:#?}", steps / 2);
}

fn turn(dir: Dir, current_dir: &mut Dir, pos: &mut (usize, usize), steps: &mut usize) {
    *current_dir = dir;
    *steps += 1;
    match dir {
        Dir::Down => pos.0 += 1,
        Dir::Up => pos.0 -= 1,
        Dir::Left => pos.1 -= 1,
        Dir::Right => pos.1 += 1,
    }
}

fn find_s(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!()
}
