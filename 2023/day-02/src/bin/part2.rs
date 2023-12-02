use std::cmp::max;

fn get_max(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (max(a.0, b.0), max(a.1, b.1), max(a.2, b.2))
}

fn parse_subset(subset_string: &str) -> (i32, i32, i32) {
    let mut count = (0,0,0);
    subset_string.split(',').map(str::trim).for_each(|s| {
        let found_cubes = s.find(' ');
        let mut cube_count = 0;
        if let Some(x) = found_cubes {
            println!("Index {:?}, {:?}", x, s);
            cube_count = s[0..x].parse::<i32>().unwrap()
        }
        let red_idx = s.find("red");
        let blue_idx= s.find("blue");
        let green_idx = s.find("green");
        println!("{:?} {:?} {:?}", red_idx, blue_idx, green_idx);
        if red_idx.is_some() { count.0 = cube_count }
        if blue_idx.is_some() { count.1 = cube_count }
        if green_idx.is_some() { count.2 = cube_count }
    });
    println!("{:?}", count);
    count
}

fn parse_game(game_string: &str) -> i32 {
    let game_split: Vec<String> = game_string.split(':').map(String::from).collect();
    let game_score = game_split[1].split(';')
        .map(|s| parse_subset(s.trim()))
        .fold((0,0,0), get_max);
    game_score.0 * game_score.1 * game_score.2
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let sum = file_content.lines().map(parse_game).sum::<i32>();
    println!("{:?}", sum);
}
