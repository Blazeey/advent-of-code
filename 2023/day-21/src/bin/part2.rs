fn dfs(
    grid: &[Vec<char>],
    visited_even: &mut [Vec<bool>],
    visited_odd: &mut [Vec<bool>],
    mut row: i32,
    mut col: i32,
    len: i32,
) {
    if row < 0 {
        row = (row + grid.len() as i32) % grid.len() as i32;
    }

    if col < 0 {
        col = (col + grid[0].len() as i32) % grid[0].len() as i32;
    }
    if len < 0
        || grid[row as usize][col as usize] == '#'
        || (len % 2 == 0 && visited_even[row as usize][col as usize])
        || (len % 2 == 1 && visited_odd[row as usize][col as usize])
    {
        return;
    }
    println!("{row} {col} {len}");
    if len % 2 == 0 {
        visited_even[row as usize][col as usize] = true;
    } else {
        visited_odd[row as usize][col as usize] = true;
    }
    // if row > 0 {
    dfs(grid, visited_even, visited_odd, row - 1, col, len - 1);
    // }
    // if col > 0 {
    dfs(grid, visited_even, visited_odd, row, col - 1, len - 1);
    // }
    // if row < grid.len() as i32 - 1 {
    dfs(grid, visited_even, visited_odd, row + 1, col, len - 1);
    // }
    // if col < grid[0].len() as i32 - 1 {
    dfs(grid, visited_even, visited_odd, row, col + 1, len - 1);
    // }
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let grid: Vec<Vec<char>> = file_content.lines().map(|l| l.chars().collect()).collect();
    let mut start = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start = (i, j);
                break;
            }
        }
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut visited_even = vec![vec![false; n]; m];
    let mut visited_odd = vec![vec![false; n]; m];

    let len = 64;
    dfs(
        &grid,
        &mut visited_even,
        &mut visited_odd,
        start.0 as i32,
        start.1 as i32,
        len,
    );

    let mut dp_1 = vec![vec![false; n]; m];
    dp_1[start.0][start.1] = true;
    let mut dp_2 = vec![vec![false; n]; m];
    let dir = [[0, -1], [0, 1], [1, 0], [-1, 0]];
    for _ in 0..len {
        dp_2 = vec![vec![false; n]; m];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != '#' {
                    for d in dir {
                        let x = i as i32 + d[0];
                        let y = j as i32 + d[1];
                        if x >= 0
                            && y >= 0
                            && x < m as i32
                            && y < n as i32
                            && dp_1[x as usize][y as usize]
                        {
                            // println!("asdasd {:?} {:?}", x, y);
                            dp_2[i][j] = true;
                        }
                    }
                }
            }
        }
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if dp_1[i][j] {
                    print!("1 ");
                    count += 1;
                } else if grid[i][j] == '#' {
                    print!("# ");
                } else {
                    print!(". ");
                }
            }
            println!("");
        }
        // println!("{:?}", dp_2);
        dp_1 = dp_2;
    }

    // println!("{:?}", count);
}
