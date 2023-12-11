fn expand_cols(grid: &mut [Vec<char>], cols: &[usize]) {
    for j in cols.iter() {
        for g in grid.iter_mut() {
            if cols.contains(j) {
                g.insert(*j, '.');
            }
        }
    }
}

fn expand_grid(grid: &mut Vec<Vec<char>>) {
    let n = grid[0].len();
    let mut expanded_rows = vec![];
    let mut expanded_cols = vec![];
    for (i, _) in grid.iter().enumerate() {
        let res = grid[i].iter().all(|c| *c == '.');
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

    expanded_cols.reverse();
    expand_cols(grid, &expanded_cols);
    let updated_n = grid[0].len();
    expanded_rows.reverse();
    expanded_rows.iter().for_each(|i| {
        let v = vec!['.'; updated_n];
        grid.insert(*i, v)
    });
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
    expand_grid(&mut grid);
    print_grid(&grid);

    let points = get_points(&grid);

    let mut total = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let xdiff = points[i].0 - points[j].0;
            let ydiff = points[i].1 - points[j].1;
            total += xdiff.abs() + ydiff.abs();
        }
    }

    println!("{:?}", total);
}
