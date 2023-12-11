use std::cmp::{max, min};
use std::collections::Bound::Excluded;

use std::collections::BTreeSet;

fn expand_grid(grid: &mut Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let m = grid.len();
    let n = grid[0].len();
    let mut expanded_rows = vec![];
    let mut expanded_cols = vec![];
    for (i, g) in grid.iter().enumerate() {
        let res = g.iter().all(|c| *c == '.');
        if res {
            expanded_rows.push(i);
        }
    }

    for j in 0..n {
        let res = grid.iter().map(|f| f[j]).all(|c| c == '.');
        if res {
            expanded_cols.push(j);
        }
    }

    (expanded_rows, expanded_cols)
}

fn parse_grid(content: &str) -> Vec<Vec<char>> {
    let grid = content
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    grid
}

fn print_grid(grid: &[Vec<char>]) {
    grid.iter().for_each(|f| {
        f.iter().for_each(|x| {
            print!("{x}");
        });
        println!();
    });
}

fn get_points(grid: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut points = vec![];
    let n = grid[0].len();
    for (i, _) in grid.iter().enumerate() {
        for j in 0..n {
            if grid[i][j] == '#' {
                points.push((i as i32, j as i32));
            }
        }
    }
    points
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let mut grid = parse_grid(file_content);
    let (rows, cols) = expand_grid(&mut grid);
    print_grid(&grid);

    let points = get_points(&grid);
    let expanded_col_nums: BTreeSet<usize> = cols.iter().cloned().collect();
    let expanded_row_nums: BTreeSet<usize> = rows.iter().cloned().collect();

    let mut total = 0;
    let multiplication_factor: u64 = 1_000_000;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let minj = min(points[i].1, points[j].1) as u64;
            let maxj = max(points[i].1, points[j].1) as u64;
            let mini = min(points[i].0, points[j].0) as u64;
            let maxi = max(points[i].0, points[j].0) as u64;

            let mut total_j_dist = 0;
            let mut total_i_dist = 0;
            if minj != maxj {
                let multiplied_cols = expanded_col_nums
                    .range((Excluded(minj as usize), Excluded(maxj as usize)))
                    .count() as u64;
                total_j_dist =
                    multiplied_cols * multiplication_factor + (maxj - minj - multiplied_cols);
            }

            if mini != maxi {
                let multiplied_rows = expanded_row_nums
                    .range((Excluded(mini as usize), Excluded(maxi as usize)))
                    .count() as u64;

                total_i_dist =
                    multiplied_rows * multiplication_factor + (maxi - mini - multiplied_rows);
            }

            // println!(
            //     "{:?} {:?} {:?} {:?}",
            //     points[i], points[j], total_i_dist, total_j_dist
            // );
            total += total_i_dist + total_j_dist;
        }
    }

    println!("{:?}", total);
}
