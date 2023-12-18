use self::Dir::*;
use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashSet};
use std::slice::Iter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    L,
    R,
    U,
    D,
}

impl Dir {
    pub fn iterator() -> Iter<'static, Dir> {
        static DIR: [Dir; 4] = [L, R, U, D];
        DIR.iter()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    row: usize,
    col: usize,
    dir: Dir,
    path: Vec<Dir>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(grid: &[Vec<u32>], start_dir: Dir) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let start = State {
        cost: 0,
        row: 0,
        col: 0,
        dir: start_dir,
        path: vec![],
    };
    let mut set: HashSet<(usize, usize, Dir)> = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(start);
    while let Some(State {
        cost,
        row,
        col,
        dir,
        path,
    }) = heap.pop()
    {
        if row == m - 1 && col == n - 1 {
            println!("{:?}", path);
            return cost;
        }
        if set.contains(&(row, col, dir)) {
            continue;
        }

        println!("{:?} {:?} {:?} {:?}", row, col, dir, cost);
        set.insert((row, col, dir));

        for dist in 4..11 {
            for d in Dir::iterator() {
                if *d == dir
                    || (dir == L && *d == R)
                    || (dir == R && *d == L)
                    || (dir == U && *d == D)
                    || (dir == D && *d == U)
                {
                    continue;
                };
                match d {
                    L => {
                        let next_col = col as i32 - dist;
                        if next_col >= 0 {
                            let mut next_dist = 0u32;
                            for j in next_col as usize..col {
                                next_dist += grid[row][j];
                            }
                            heap.push(State {
                                cost: cost + next_dist,
                                row,
                                col: next_col as usize,
                                dir: Dir::L,
                                path: [path.clone(), vec![Dir::L; dist as usize]].concat(),
                            });
                            println!(
                                "({:?}, {:?}) ({:?}, {:?}) - {:?}",
                                row,
                                col,
                                row,
                                next_col,
                                cost + next_dist
                            );
                        }
                        // if col as i32 - 1 > 0 {
                        //     heap.push(State {
                        //         cost: cost + grid[row][col - 1],
                        //         row,
                        //         col: col - 1,
                        //         dir: Dir::L,
                        //         path: [path.clone(), vec![Dir::L]].concat(),
                        //     })
                        // }
                        // if col as i32 - 2 >= 0 {
                        //     heap.push(State {
                        //         cost: cost + grid[row][col - 1] + grid[row][col - 2],
                        //         row,
                        //         col: col - 2,
                        //         dir: Dir::L,
                        //         path: [path.clone(), vec![Dir::L, Dir::L]].concat(),
                        //     })
                        // }
                        // if col as i32 - 3 >= 0 {
                        //     heap.push(State {
                        //         cost: cost
                        //             + grid[row][col - 1]
                        //             + grid[row][col - 2]
                        //             + grid[row][col - 3],
                        //         row,
                        //         col: col - 3,
                        //         dir: Dir::L,
                        //         path: [path.clone(), vec![Dir::L, Dir::L, Dir::L]].concat(),
                        //     })
                        // }
                    }
                    R => {
                        let next_col = col + dist as usize;
                        if next_col < n {
                            let mut next_dist = 0u32;
                            for j in col + 1..next_col + 1 {
                                next_dist += grid[row][j];
                            }
                            heap.push(State {
                                cost: cost + next_dist,
                                row,
                                col: next_col,
                                dir: Dir::R,
                                path: [path.clone(), vec![Dir::R; dist as usize]].concat(),
                            });
                            println!(
                                "({:?}, {:?}) ({:?}, {:?}) - {:?}",
                                row,
                                col,
                                row,
                                next_col,
                                cost + next_dist
                            );
                        }
                        // if col + 1 < n {
                        //     heap.push(State {
                        //         cost: cost + grid[row][col + 1],
                        //         row,
                        //         col: col + 1,
                        //         dir: Dir::R,
                        //         path: [path.clone(), vec![Dir::R]].concat(),
                        //     })
                        // }
                        // if col + 2 < n {
                        //     heap.push(State {
                        //         cost: cost + grid[row][col + 1] + grid[row][col + 2],
                        //         row,
                        //         col: col + 2,
                        //         dir: Dir::R,
                        //         path: [path.clone(), vec![Dir::R, Dir::R]].concat(),
                        //     })
                        // }
                        // if col + 3 < n {
                        //     heap.push(State {
                        //         cost: cost
                        //             + grid[row][col + 1]
                        //             + grid[row][col + 2]
                        //             + grid[row][col + 3],
                        //         row,
                        //         col: col + 3,
                        //         dir: Dir::R,
                        //         path: [path.clone(), vec![Dir::R, Dir::R, Dir::R]].concat(),
                        //     })
                        // }
                    }
                    U => {
                        let next_row = row as i32 - dist;
                        if next_row >= 0 {
                            let mut next_dist = 0u32;
                            for j in next_row as usize..row {
                                next_dist += grid[j][col];
                            }
                            heap.push(State {
                                cost: cost + next_dist,
                                row: next_row as usize,
                                col,
                                dir: Dir::U,
                                path: [path.clone(), vec![Dir::U; dist as usize]].concat(),
                            });
                            println!(
                                "({:?}, {:?}) ({:?}, {:?}) - {:?}",
                                row,
                                col,
                                next_row,
                                col,
                                cost + next_dist
                            );
                        }
                        // if row as i32 > 0 {
                        //     heap.push(State {
                        //         cost: cost + grid[row - 1][col],
                        //         row: row - 1,
                        //         col,
                        //         dir: Dir::U,
                        //         path: [path.clone(), vec![Dir::U]].concat(),
                        //     })
                        // }
                        // if row as i32 - 2 >= 0 {
                        //     heap.push(State {
                        //         cost: cost + grid[row - 1][col] + grid[row - 2][col],
                        //         row: row - 2,
                        //         col,
                        //         dir: Dir::U,
                        //         path: [path.clone(), vec![Dir::U, Dir::U]].concat(),
                        //     })
                        // }
                        // if row as i32 - 3 >= 0 {
                        //     heap.push(State {
                        //         cost: cost
                        //             + grid[row - 1][col]
                        //             + grid[row - 2][col]
                        //             + grid[row - 3][col],
                        //         row: row - 3,
                        //         col,
                        //         dir: Dir::U,
                        //         path: [path.clone(), vec![Dir::U, Dir::U, Dir::U]].concat(),
                        //     })
                        // }
                    }
                    D => {
                        let next_row = row + dist as usize;
                        if next_row < m {
                            let mut next_dist = 0u32;
                            for j in row + 1..next_row + 1 {
                                next_dist += grid[j][col];
                            }
                            heap.push(State {
                                cost: cost + next_dist,
                                row: next_row,
                                col,
                                dir: Dir::D,
                                path: [path.clone(), vec![Dir::D; dist as usize]].concat(),
                            });
                            println!(
                                "({:?}, {:?}) ({:?}, {:?}) - {:?}",
                                row,
                                col,
                                next_row,
                                col,
                                cost + next_dist
                            );
                        }
                    }
                }
            }
        }
    }
    0
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let grid: Vec<Vec<u32>> = file_content
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let ans = min(solve(&grid, L), solve(&grid, U));
    println!("Ans: {:?}", ans);
}
