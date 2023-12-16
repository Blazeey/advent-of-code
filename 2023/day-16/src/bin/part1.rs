use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    L,
    R,
    U,
    D,
}

fn solve(
    grid: &[Vec<char>],
    new_grid: &mut [Vec<bool>],
    visited: &mut HashSet<(i32, i32, Dir)>,
    row: i32,
    col: i32,
    dir: Dir,
) {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    if row < 0 || col < 0 || row >= m || col >= n || visited.contains(&(row, col, dir)) {
        return;
    }
    new_grid[row as usize][col as usize] = true;
    visited.insert((row, col, dir));
    match grid[row as usize][col as usize] {
        '|' => match dir {
            Dir::L | Dir::R => {
                solve(grid, new_grid, visited, row - 1, col, Dir::D);
                solve(grid, new_grid, visited, row + 1, col, Dir::U);
            }
            Dir::U => solve(grid, new_grid, visited, row + 1, col, Dir::U),
            Dir::D => solve(grid, new_grid, visited, row - 1, col, Dir::D),
        },
        '-' => match dir {
            Dir::U | Dir::D => {
                solve(grid, new_grid, visited, row, col - 1, Dir::R);
                solve(grid, new_grid, visited, row, col + 1, Dir::L);
            }
            Dir::L => solve(grid, new_grid, visited, row, col + 1, Dir::L),
            Dir::R => solve(grid, new_grid, visited, row, col - 1, Dir::R),
        },
        '\\' => match dir {
            Dir::L => solve(grid, new_grid, visited, row + 1, col, Dir::U),
            Dir::R => solve(grid, new_grid, visited, row - 1, col, Dir::D),
            Dir::U => solve(grid, new_grid, visited, row, col + 1, Dir::L),
            Dir::D => solve(grid, new_grid, visited, row, col - 1, Dir::R),
        },
        '/' => match dir {
            Dir::L => solve(grid, new_grid, visited, row - 1, col, Dir::D),
            Dir::R => solve(grid, new_grid, visited, row + 1, col, Dir::U),
            Dir::U => solve(grid, new_grid, visited, row, col - 1, Dir::R),
            Dir::D => solve(grid, new_grid, visited, row, col + 1, Dir::L),
        },
        '.' => match dir {
            Dir::L => solve(grid, new_grid, visited, row, col + 1, Dir::L),
            Dir::R => solve(grid, new_grid, visited, row, col - 1, Dir::R),
            Dir::U => solve(grid, new_grid, visited, row + 1, col, Dir::U),
            Dir::D => solve(grid, new_grid, visited, row - 1, col, Dir::D),
        },
        _ => (),
    }
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let grid: Vec<Vec<char>> = file_content.lines().map(|l| l.chars().collect()).collect();
    let m = grid.len();
    let n = grid[0].len();
    let mut new_grid = vec![vec![false; n]; m];
    let mut visited: HashSet<_> = HashSet::new();
    solve(&grid, &mut new_grid, &mut visited, 0, 0, Dir::L);
    let ans = new_grid.iter().flatten().filter(|f| **f).count();
    for i in new_grid {
        for j in i {
            print!("{:?} ", j);
        }
        println!("");
    }
    println!("Ans: {:?}", ans);
}
