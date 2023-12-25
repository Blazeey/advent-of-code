use std::cmp::max;
use std::collections::HashSet;

const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn dfs(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    visited: &mut HashSet<(usize, usize)>,
    len: i64,
) -> i64 {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    if visited.contains(&(row, col)) {
        return i64::MIN;
    }
    if row == m as usize - 1 {
        println!("{len}");
        return len;
    }
    visited.insert((row, col));
    let mut maxx = i64::MIN;
    match grid[row][col] {
        '.' => {
            for d in DIR {
                let x = row as i32 + d.0;
                let y = col as i32 + d.1;
                if x < 0 || y < 0 || x >= m || y >= n {
                    continue;
                }

                maxx = max(maxx, dfs(grid, x as usize, y as usize, visited, len + 1));
            }
        }
        '>' => {
            let x = row as i32;
            let y = col as i32 + 1;
            if x >= 0 && y >= 0 && x < m && y < n {
                maxx = max(maxx, dfs(grid, x as usize, y as usize, visited, len + 1));
            }
        }
        '<' => {
            let x = row as i32;
            let y = col as i32 - 1;
            if x >= 0 && y >= 0 && x < m && y < n {
                maxx = max(maxx, dfs(grid, x as usize, y as usize, visited, len + 1));
            }
        }
        'v' => {
            let x = row as i32 + 1;
            let y = col as i32;
            if x >= 0 && y >= 0 && x < m && y < n {
                maxx = max(maxx, dfs(grid, x as usize, y as usize, visited, len + 1));
            }
        }
        '^' => {
            let x = row as i32 - 1;
            let y = col as i32;
            if x >= 0 && y >= 0 && x < m && y < n {
                maxx = max(maxx, dfs(grid, x as usize, y as usize, visited, len + 1));
            }
        }
        _ => (),
    }

    visited.remove(&(row, col));

    maxx
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let grid: Vec<Vec<char>> = file_content.lines().map(|l| l.chars().collect()).collect();
    let ans = (0..grid[0].len())
        .map(|i| {
            let mut s = HashSet::new();
            dfs(&grid, 0, i, &mut s, 0)
        })
        .max()
        .unwrap();
    println!("{:?}", ans);
}
