use self::Dir::*;
use std::cmp::{max, min};

#[derive(Debug)]
enum Dir {
    L,
    R,
    U,
    D,
}

#[derive(Debug)]
struct Plan {
    dir: Dir,
    num: u32,
}

fn parse_line(line: &str) -> Plan {
    let values: Vec<_> = line.split(' ').collect();
    let dir = match values[0] {
        "R" => R,
        "L" => L,
        "U" => U,
        "D" => D,
        _ => L,
    };
    let num = values[1].parse::<u32>().unwrap();

    Plan { dir, num }
}

fn get_max_size(plans: &[Plan]) -> (usize, usize, usize, usize) {
    let mut r_max: i32 = 0;
    let mut c_max: i32 = 0;
    let mut r_min: i32 = 0;
    let mut c_min: i32 = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    plans.iter().for_each(|plan| {
        match plan.dir {
            L => c -= plan.num as i32,
            R => c += plan.num as i32,
            U => r -= plan.num as i32,
            D => r += plan.num as i32,
        };
        r_max = max(r_max, r);
        c_max = max(c_max, c);
        r_min = min(r_min, r);
        c_min = min(c_min, c);
    });
    (
        (r_max - r_min + 1) as usize,
        (c_max - c_min + 1) as usize,
        (-r_min) as usize,
        (-c_min) as usize,
    )
}

fn mark_cells(grid: &mut Vec<Vec<char>>, i: i32, j: i32, to_char: char) {
    if i < 0
        || i >= grid.len() as i32
        || j < 0
        || j >= grid[0].len() as i32
        || grid[i as usize][j as usize] != '.'
    {
        return;
    }
    grid[i as usize][j as usize] = to_char;
    mark_cells(grid, i + 1, j, to_char);
    mark_cells(grid, i - 1, j, to_char);
    mark_cells(grid, i, j + 1, to_char);
    mark_cells(grid, i, j - 1, to_char);
}

fn mark_trenches(grid: &mut [Vec<char>], plans: &[Plan], row: usize, col: usize) {
    let mut cur_row = row;
    let mut cur_col = col;
    grid[row][col] = '#';
    plans.iter().for_each(|plan| {
        for _ in 0..plan.num {
            match plan.dir {
                L => cur_col -= 1,
                R => cur_col += 1,
                U => cur_row -= 1,
                D => cur_row += 1,
            }
            grid[cur_row][cur_col] = '#';
        }
    });

    grid.iter().for_each(|f| {
        f.iter().for_each(|x| {
            print!("{x}");
        });
        println!();
    });
}

fn check_new_grid(old_grid: &Vec<Vec<char>>) -> i32 {
    let m = old_grid.len();
    let n = old_grid[0].len();
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            let a = old_grid[i][j];
            if a == 'O' {
                count += 1;
            }
        }
    }
    (m * n - count) as i32
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let plans: Vec<_> = file_content.lines().map(parse_line).collect();
    let (m, n, r, c) = get_max_size(&plans);
    let mut grid = vec![vec!['.'; n]; m];

    println!("{:?} {:?} {:?} {:?}", m, n, r, c);

    mark_trenches(&mut grid, &plans, r, c);
    for j in 0..n as i32 {
        mark_cells(&mut grid, 0, j, 'O');
        mark_cells(&mut grid, m as i32 - 1, j, 'O');
    }

    for i in 0..m as i32 {
        mark_cells(&mut grid, i, 0, 'O');
        mark_cells(&mut grid, i, n as i32 - 1, 'O');
    }

    grid.iter().for_each(|f| {
        f.iter().for_each(|x| {
            print!("{x}");
        });
        println!();
    });
    let ans = check_new_grid(&grid);
    println!("Ans {:?}", ans);
}
