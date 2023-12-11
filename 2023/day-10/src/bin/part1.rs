use std::cmp::max;

#[derive(Debug)]
struct Grid {
    area_grid: Vec<Vec<char>>,
    start: (usize, usize),
}

#[derive(Debug)]
enum Dir {
    L,
    R,
    U,
    D,
}

fn get_length(grid: &Grid, i: i32, j: i32, dir: Dir, len: u32) -> u32 {
    let m = grid.area_grid.len() as i32;
    let n = grid.area_grid[0].len() as i32;
    if i < 0 || i >= m || j < 0 || j >= n {
        return 0;
    }
    match grid.area_grid[i as usize][j as usize] {
        'L' => match dir {
            Dir::R => get_length(grid, i - 1, j, Dir::D, len + 1),
            Dir::U => get_length(grid, i, j + 1, Dir::L, len + 1),
            _ => 0,
        },
        '|' => match dir {
            Dir::U => get_length(grid, i + 1, j, Dir::U, len + 1),
            Dir::D => get_length(grid, i - 1, j, Dir::D, len + 1),
            _ => 0,
        },
        '-' => match dir {
            Dir::L => get_length(grid, i, j + 1, Dir::L, len + 1),
            Dir::R => get_length(grid, i, j - 1, Dir::R, len + 1),
            _ => 0,
        },
        'J' => match dir {
            Dir::U => get_length(grid, i, j - 1, Dir::R, len + 1),
            Dir::L => get_length(grid, i - 1, j, Dir::D, len + 1),
            _ => 0,
        },
        '7' => match dir {
            Dir::D => get_length(grid, i, j - 1, Dir::R, len + 1),
            Dir::L => get_length(grid, i + 1, j, Dir::U, len + 1),
            _ => 0,
        },
        'F' => match dir {
            Dir::R => get_length(grid, i + 1, j, Dir::U, len + 1),
            Dir::D => get_length(grid, i, j + 1, Dir::L, len + 1),
            _ => 0,
        },
        'S' => len,
        _ => 0,
    }
}

fn parse_grid(content: &str) -> Grid {
    let grid = content
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut start = (0, 0);
    for (i, g) in grid.iter().enumerate() {
        for (j, grid) in g.iter().enumerate() {
            if *grid == 'S' {
                start = (i, j);
            }
        }
    }
    Grid {
        area_grid: grid,
        start,
    }
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let grid = parse_grid(file_content);

    let m = grid.area_grid.len();
    let n = grid.area_grid[0].len();

    let dir_value = |dir: &Dir| -> (i32, i32) {
        match dir {
            Dir::L => (0, -1),
            Dir::R => (0, 1),
            Dir::U => (-1, 0),
            Dir::D => (1, 0),
        }
    };
    let neighbors = [Dir::L, Dir::R, Dir::U, Dir::D];
    let mut ans = 0;
    for next in neighbors.iter() {
        let x = grid.start.0 as i32 + dir_value(next).0;
        let y = grid.start.1 as i32 + dir_value(next).1;
        if x < 0
            || x >= m as i32
            || y < 0
            || y >= n as i32
            || grid.area_grid[x as usize][y as usize] == '.'
        {
            continue;
        }
        let c = grid.area_grid[x as usize][y as usize];
        match *next {
            Dir::L => match c {
                '-' | 'L' | 'F' => ans = max(ans, get_length(&grid, x, y, Dir::R, 1)),
                _ => (),
            },
            Dir::R => match c {
                '-' | 'J' | '7' => ans = max(ans, get_length(&grid, x, y, Dir::L, 1)),
                _ => (),
            },
            Dir::U => match c {
                '|' | 'L' | 'J' => ans = max(ans, get_length(&grid, x, y, Dir::D, 1)),
                _ => (),
            },
            Dir::D => match c {
                '|' | 'F' | '7' => ans = max(ans, get_length(&grid, x, y, Dir::U, 1)),
                _ => (),
            },
        }
    }

    if ans % 2 == 0 {
        ans /= 2;
    } else {
        ans /= 2 + 1;
    }
    println!("Ans: {:?}", ans);
}
