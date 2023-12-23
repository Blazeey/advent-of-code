use std::{
    clone,
    cmp::max,
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn from(x: &[&str]) -> Point {
        Point {
            x: x[0].parse::<u64>().unwrap(),
            y: x[1].parse::<u64>().unwrap(),
            z: x[2].parse::<u64>().unwrap(),
        }
    }
}

fn solve(grid: &mut [Vec<Vec<i64>>], blocks: &[(Point, Point)]) -> u64 {
    let m = grid.len();
    let mut count = 0;
    let mut b_bricks: HashSet<i64> = HashSet::new();
    let mut bricks_map: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut base = vec![];
    let mut in_deg: HashMap<i64, usize> = HashMap::new();
    for (k, (point_a, point_b)) in blocks.iter().enumerate() {
        let mut idx = 1;
        let mut overlapping_bricks: HashSet<i64> = HashSet::new();
        for z in (1..point_a.z + 1).rev() {
            overlapping_bricks.clear();
            let mut possible = false;
            for x in min(point_a.x, point_b.x)..max(point_a.x, point_b.x) + 1 {
                for y in min(point_a.y, point_b.y)..max(point_a.y, point_b.y) + 1 {
                    // println!(
                    //     "Getting {x} {y} {z} = {:?}",
                    //     grid[x as usize][y as usize][z as usize]
                    // );
                    if grid[x as usize][y as usize][z as usize - 1] != -1 {
                        possible = true;
                    }
                }
            }
            if possible {
                println!("{:?} {:?} {idx} {z}", point_a, point_b);
                idx = z;
                break;
            }
        }
        let minz = min(point_a.z, point_b.z);
        for x in min(point_a.x, point_b.x)..max(point_a.x, point_b.x) + 1 {
            for y in min(point_a.y, point_b.y)..max(point_a.y, point_b.y) + 1 {
                for z in min(point_a.z, point_b.z)..max(point_a.z, point_b.z) + 1 {
                    let new_z = (idx + z - minz) as usize;
                    println!("Setting {x} {y} {new_z} as {:?}", k + 1);
                    grid[x as usize][y as usize][new_z] = k as i64 + 1;
                    overlapping_bricks.insert(grid[x as usize][y as usize][new_z - 1]);
                }
            }
        }
        overlapping_bricks.remove(&0);
        overlapping_bricks.remove(&-1);
        overlapping_bricks.remove(&(k as i64 + 1));
        println!("Bottom bricks {:?}", overlapping_bricks);
        in_deg.insert(k as i64 + 1, overlapping_bricks.len());
        for b in overlapping_bricks.clone() {
            bricks_map.entry(b).or_default();

            println!("Pushing {:?} to {:?} map", k + 1, &b);
            bricks_map.get_mut(&b).unwrap().push(k as i64 + 1);
        }
        // overlapping_bricks.remove(&0);
        // count += overlapping_bricks.len();
        let l = overlapping_bricks.len();
        if l == 1 {
            for p in overlapping_bricks {
                b_bricks.insert(p);
            }
        }
        if idx == 1 {
            base.push(k as i64 + 1);
        }
    }

    let mut total = 0;
    for b in b_bricks.clone() {
        total += topo_sort(&bricks_map, &in_deg, b) - 1;

        // println!("STARTING REC {:?} {:?}", b, x.len());
        // total += x.len() as u64;
    }
    total
}

fn topo_sort(m: &HashMap<i64, Vec<i64>>, in_deg: &HashMap<i64, usize>, cur: i64) -> u64 {
    let mut total = 0;

    let mut q: VecDeque<i64> = VecDeque::new();
    q.push_back(cur);

    let mut in_d = in_deg.clone();
    while !q.is_empty() {
        let top = q.pop_front().unwrap();
        total += 1;
        let x = m.get(&top);
        if x.is_none() {
            continue;
        }
        for n in x.unwrap() {
            *in_d.get_mut(n).unwrap() -= 1;
            if *in_d.get_mut(n).unwrap() == 0 {
                q.push_back(*n);
            }
        }
    }

    total
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let mut bricks: Vec<_> = file_content
        .lines()
        .map(|l| {
            let points: Vec<_> = l.split('~').collect();
            let a: Vec<_> = points[0].split(',').collect();
            let b: Vec<_> = points[1].split(',').collect();
            let mut point_a = Point::from(&a);
            let mut point_b = Point::from(&b);
            if point_b.z < point_a.z {
                (point_a, point_b) = (point_b, point_a);
            }
            (point_a, point_b)
        })
        .collect();

    let mut grid = vec![vec![vec![-1; 500]; 500]; 500];
    for x in 0..500 {
        for y in 0..500 {
            grid[x][y][0] = 0;
        }
    }
    bricks.sort_by(|(ax, _), (bx, _)| ax.z.cmp(&bx.z));

    let ans = solve(&mut grid, &bricks);

    println!("Ans {:?}", ans);
}
