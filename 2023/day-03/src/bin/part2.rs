use std::collections::HashMap;
use std::collections::HashSet;
use std::vec;

fn parse_input(content: &str) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>> = content.lines().map(|s| s.chars().collect()).collect();
    input
}

fn check_nearby_symbol(schematic: &[Vec<char>], row: i32, col: i32) -> Vec<(usize, usize)> {
    let neighbors: Vec<Vec<i32>> = vec![
        vec![-1, -1],
        vec![-1, 0],
        vec![-1, 1],
        vec![0, -1],
        vec![0, 1],
        vec![1, -1],
        vec![1, 0],
        vec![1, 1],
    ];
    let m = schematic.len() as i32;
    let n = schematic[0].len() as i32;

    let mut gears: Vec<(usize, usize)> = Vec::new();

    for next in neighbors.iter() {
        let x = row + next[0];
        let y = col + next[1];
        if x >= 0 && x < m && y >= 0 && y < n && schematic[x as usize][y as usize] == '*' {
            gears.push((x as usize, y as usize));
        }
    }
    // println!("{:?}, {:?}, {:?}", row, col, gears);
    gears
}

fn update_gears_neighbors(
    gears_map: &mut HashMap<(usize, usize), Vec<u64>>,
    schematic: &[Vec<char>],
    row: usize,
) {
    let mut cur_number: u64 = 0;
    let mut cur_num_gears: HashSet<(usize, usize)> = HashSet::new();
    for (i, x) in schematic[row].iter().enumerate() {
        match x {
            '0'..='9' => {
                cur_number = cur_number * 10 + x.to_digit(10).unwrap() as u64;
                let gears = check_nearby_symbol(schematic, row as i32, i as i32);
                for x in gears {
                    cur_num_gears.insert(x);
                }
            }
            _ => {
                let k: Vec<_> = cur_num_gears.iter().collect();
                for gear in k {
                    let list = gears_map.entry(*gear).or_default();
                    list.push(cur_number);
                }
                cur_number = 0;
                cur_num_gears.clear()
            }
        }
    }
    let k: Vec<_> = cur_num_gears.iter().collect();
    for gear in k {
        let list = gears_map.entry(*gear).or_default();
        list.push(cur_number);
    }
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let schematic = parse_input(file_content);
    let mut gears_map: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    for i in 0..=schematic.len() - 1 {
        update_gears_neighbors(&mut gears_map, &schematic, i);
    }

    let mut total: u64 = 0;
    for v in gears_map.values() {
        if v.len() == 2 {
            let cur = v.iter().product::<u64>();
            total += cur;
            // println!("{:?}, {:?}, {:?}", cur, total, v);
        }
    }

    // println!("{:?}", gears_map);
    println!("{:?}", total);
}
