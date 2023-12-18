use self::Dir::*;

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
    let hex = values[2]
        .split('#')
        .nth(1)
        .unwrap()
        .split(')')
        .next()
        .unwrap()
        .to_string();
    let dir = match hex.chars().nth(5).unwrap() {
        '0' => R,
        '1' => D,
        '2' => L,
        '3' => U,
        _ => L,
    };
    let num = u32::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();
    Plan { dir, num }
}

fn get_coordinates(plans: &[Plan]) -> Vec<(i64, i64)> {
    let mut r: i64 = 0;
    let mut c: i64 = 0;
    let mut coordinates = vec![];
    plans.iter().for_each(|plan| {
        match plan.dir {
            L => c -= plan.num as i64,
            R => c += plan.num as i64,
            U => r -= plan.num as i64,
            D => r += plan.num as i64,
        };
        coordinates.push((r, c));
    });
    coordinates.reverse();
    coordinates.push((0, 0));
    coordinates
}

fn area(coord: &[(i64, i64)]) -> u64 {
    let mut res = 0i64;
    for i in 0..coord.len() {
        let (x1, y1) = coord[i];
        let (x2, y2) = coord[(i + 1) % coord.len()];
        res += (x1 * y2) - (x2 * y1);
    }
    let perimeter = coord
        .iter()
        .zip(coord.iter().skip(1))
        .fold(0, |acc, (a, b)| acc + (a.0 - b.0).abs() + (a.1 - b.1).abs());
    println!("{:?} {:?}", res.abs(), perimeter);
    ((res.abs() + perimeter) / 2) as u64 + 1
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let plans: Vec<_> = file_content.lines().map(parse_line).collect();

    let coordinates = get_coordinates(&plans);
    println!("{:?}", coordinates);
    let res = area(&coordinates);

    println!("Ans {:?}", res);
}
