use std::collections::HashSet;

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

fn mark_cells(grid: &mut Vec<Vec<char>>, i: i32, j: i32, to_char: char) {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] != '.' {
        return;
    }
    grid[i as usize][j as usize] = to_char;
    mark_cells(grid, i + 1, j, to_char);
    mark_cells(grid, i - 1, j, to_char);
    mark_cells(grid, i, j + 1, to_char);
    mark_cells(grid, i, j - 1, to_char);
}

fn expand_grid(grid: &Grid) -> Vec<Vec<char>> {
    let m = grid.area_grid.len() as i32;
    let n = grid.area_grid[0].len() as i32;
    let mut new_grid: Vec<Vec<char>> = vec![];

    let horizontal_match = |a: char, b: char| -> bool {
        matches!(
            (a, b),
            ('L', '-')
                | ('L', 'J')
                | ('L', '7')
                | ('L', 'S')
                | ('-', 'J')
                | ('-', '7')
                | ('-', 'S')
                | ('-', '-')
                | ('F', '-')
                | ('F', 'J')
                | ('F', '7')
                | ('F', 'S')
                | ('S', '-')
                | ('S', 'J')
                | ('S', '7')
        )
    };

    let vertial_match = |a: char, b: char| -> bool {
        matches!(
            (a, b),
            ('|', '|')
                | ('|', 'L')
                | ('|', 'J')
                | ('|', 'S')
                | ('F', '|')
                | ('F', 'L')
                | ('F', 'J')
                | ('F', 'S')
                | ('7', '|')
                | ('7', 'L')
                | ('7', 'J')
                | ('7', 'S')
                | ('S', '|')
                | ('S', 'L')
                | ('S', 'J')
        )
    };

    grid.area_grid.iter().for_each(|line| {
        let mut v: Vec<char> = vec![];
        for i in 0..line.len() {
            v.push(line[i]);
            if i != line.len() - 1 && horizontal_match(line[i], line[i + 1]) {
                v.push('-');
            } else {
                v.push('.');
            }
        }
        new_grid.push(v);
    });
    for i in (1..m + 1).rev() {
        let v = vec!['.'; 2 * n as usize];
        new_grid.insert(i as usize, v);
    }
    let mut i: usize = 1;
    while i < 2 * m as usize - 1 {
        for j in 0usize..2 * n as usize {
            if vertial_match(new_grid[i - 1][j], new_grid[i + 1][j]) {
                new_grid[i][j] = '|';
            }
        }
        i += 2;
    }
    new_grid.iter().for_each(|f| {
        f.iter().for_each(|x| {
            print!("{x}");
        });
        println!();
    });
    new_grid
}

fn get_length(
    grid: &Grid,
    i: i32,
    j: i32,
    dir: Dir,
    len: u32,
    vertices: &mut HashSet<(usize, usize)>,
) -> u32 {
    let m = grid.area_grid.len() as i32;
    let n = grid.area_grid[0].len() as i32;
    if i < 0 || i >= m || j < 0 || j >= n {
        return 0;
    }
    vertices.insert((i as usize, j as usize));
    match grid.area_grid[i as usize][j as usize] {
        'L' => match dir {
            Dir::R => get_length(grid, i - 1, j, Dir::D, len + 1, vertices),
            Dir::U => get_length(grid, i, j + 1, Dir::L, len + 1, vertices),
            _ => 0,
        },
        '|' => match dir {
            Dir::U => get_length(grid, i + 1, j, Dir::U, len + 1, vertices),
            Dir::D => get_length(grid, i - 1, j, Dir::D, len + 1, vertices),
            _ => 0,
        },
        '-' => match dir {
            Dir::L => get_length(grid, i, j + 1, Dir::L, len + 1, vertices),
            Dir::R => get_length(grid, i, j - 1, Dir::R, len + 1, vertices),
            _ => 0,
        },
        'J' => match dir {
            Dir::U => get_length(grid, i, j - 1, Dir::R, len + 1, vertices),
            Dir::L => get_length(grid, i - 1, j, Dir::D, len + 1, vertices),
            _ => 0,
        },
        '7' => match dir {
            Dir::D => get_length(grid, i, j - 1, Dir::R, len + 1, vertices),
            Dir::L => get_length(grid, i + 1, j, Dir::U, len + 1, vertices),
            _ => 0,
        },
        'F' => match dir {
            Dir::R => get_length(grid, i + 1, j, Dir::U, len + 1, vertices),
            Dir::D => get_length(grid, i, j + 1, Dir::L, len + 1, vertices),
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

fn check_new_grid(old_grid: &Vec<Vec<char>>, new_grid: &[Vec<char>]) -> i32 {
    let m = old_grid.len();
    let n = old_grid[0].len();
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            let a = old_grid[i][j];
            let b = new_grid[i * 2][j * 2];
            if a == '.' && b == '.' {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let mut grid = parse_grid(file_content);

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
    let mut path: HashSet<(usize, usize)> = HashSet::new();
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
                '-' | 'L' | 'F' => {
                    let mut h = HashSet::new();
                    let len = get_length(&grid, x, y, Dir::R, 1, &mut h);
                    if len > ans {
                        ans = len;
                        path = h;
                    }
                }
                _ => (),
            },
            Dir::R => match c {
                '-' | 'J' | '7' => {
                    let mut h = HashSet::new();
                    let len = get_length(&grid, x, y, Dir::L, 1, &mut h);
                    if len > ans {
                        ans = len;
                        path = h;
                    }
                }
                _ => (),
            },
            Dir::U => match c {
                '|' | 'L' | 'J' => {
                    let mut h = HashSet::new();
                    let len = get_length(&grid, x, y, Dir::D, 1, &mut h);
                    if len > ans {
                        ans = len;
                        path = h;
                    }
                }
                _ => (),
            },
            Dir::D => match c {
                '|' | 'F' | '7' => {
                    let mut h = HashSet::new();
                    let len = get_length(&grid, x, y, Dir::U, 1, &mut h);
                    if len > ans {
                        ans = len;
                        path = h;
                    }
                }
                _ => (),
            },
        }
    }

    for i in 0..m {
        for j in 0..n {
            if !path.contains(&(i, j)) {
                grid.area_grid[i][j] = '.';
            }
        }
    }

    let mut new_grid = expand_grid(&grid);
    let nm = new_grid.len() as i32;
    let nn = new_grid[0].len() as i32;

    for j in 0..nn {
        mark_cells(&mut new_grid, 0, j, 'O');
        mark_cells(&mut new_grid, nm - 1, j, 'O');
    }

    for i in 0..nm {
        mark_cells(&mut new_grid, i, 0, 'O');
        mark_cells(&mut new_grid, i, nn - 1, 'O');
    }

    new_grid.iter().for_each(|f| {
        f.iter().for_each(|x| {
            print!("{x}");
        });
        println!();
    });
    let check = check_new_grid(&grid.area_grid, &new_grid);
    println!("Ans: {:?}", check);
}
