#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
}

fn parse_grid(content: &str) -> Grid {
    Grid {
        grid: content.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn parse(content: &str) -> Vec<Grid> {
    content.split("\n\n").map(parse_grid).collect()
}

fn transform_rows(line: &Vec<char>) -> u64 {
    line.iter().enumerate().fold(
        0u64,
        |acc, (i, c)| {
            if *c == '#' {
                acc | 1 << i
            } else {
                acc
            }
        },
    )
}

fn transform_cols(grid: &Grid, col: usize) -> u64 {
    grid.grid
        .iter()
        .map(|f| f[col])
        .enumerate()
        .fold(
            0u64,
            |acc, (i, c)| {
                if c == '#' {
                    acc | 1 << i
                } else {
                    acc
                }
            },
        )
}

fn check_reverse(line: &Vec<u64>, idx: usize) -> (u32, bool) {
    let mut count = 0;
    for i in 0..line.len() {
        // println!("check {:?} {:?} {:?}", i, line[idx - i], line[idx + i + 1]);
        if idx as i32 - i as i32 >= 0
            && idx + i + 1 < line.len()
            && line[idx - i] != line[idx + i + 1]
        {
            // println!("false");
            count += 1;
            if (line[idx - i] ^ line[idx + i + 1]).count_ones() > 1 {
                return (count, false);
            }
        }
    }
    (count, true)
}

fn solve_grid(grid: &Grid) -> u64 {
    let rows: Vec<_> = grid.grid.iter().map(transform_rows).collect();
    let mut cols = vec![0u64; grid.grid[0].len()];

    cols = cols
        .iter()
        .enumerate()
        .map(|(col, _)| transform_cols(grid, col))
        .collect();
    println!("{:?}", rows);
    println!("{:?}", cols);
    for i in 0..rows.len() - 1 {
        let count = check_reverse(&rows, i);
        println!("rows {:?} {:?}", i, count);
        if count.1 && count.0 == 1 {
            return 100 * (i as u64 + 1);
        }
    }
    for i in 0..grid.grid[0].len() - 1 {
        let count = check_reverse(&cols, i);
        if count.1 && count.0 == 1 {
            println!("rows {:?} {:?}", i, true);
            return i as u64 + 1;
        }
    }
    0
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let grids = parse(file_content);

    println!("{:?}", grids);

    let ans: u64 = grids.iter().map(solve_grid).sum();
    println!("{:?}", ans);
}
