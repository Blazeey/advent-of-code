
fn parse_input(content: &str) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>> = content.lines().map(|s| s.chars().collect()).collect();
    input
}

fn check_nearby_symbol(schematic: &[Vec<char>], row: i32, col: i32) -> bool {
    let neighbors: Vec<Vec<i32>> = vec![vec![-1,-1],vec![-1,0],vec![-1,1],vec![0,-1],vec![0,1],vec![1,-1],vec![1,0],vec![1,1]];
    let m = schematic.len() as i32;
    let n = schematic[0].len() as i32;
    
    for next in neighbors.iter() {
        let x = row + next[0];
        let y = col + next[1];
        if x >= 0 && x < m && y >= 0 && y < n {
            match schematic[x as usize][y as usize] {
                '0'..='9' | '.' => (),
                _ => return true
            }
        }
    }
    false
}

fn compute_line_sum(schematic: &[Vec<char>], row: usize) -> u32 {
    let mut total = 0;
    let mut cur_number = 0;
    let mut near_char = false;
    for (i, x) in schematic[row].iter().enumerate() {
        match x {
            '0'..='9' => {
                cur_number = cur_number * 10 + x.to_digit(10).unwrap();
                if !near_char {
                    near_char |= check_nearby_symbol(schematic, row as i32, i as i32);
                }
            },
            _ => {
                if near_char {
                    total += cur_number;
                }
                cur_number = 0;
                near_char = false;
            }
        }
    }
    if near_char {
        total += cur_number;
    }
    total
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let schematic = parse_input(file_content);
    let mut total = 0;
    for i in 0..=schematic.len()-1 {
        total += compute_line_sum(&schematic, i);
    }

    println!("{:?}", total);
}
