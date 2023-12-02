const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

fn check_validity(subset: (i32, i32, i32)) -> bool {
    subset.0 <= RED_MAX && subset.1<= BLUE_MAX && subset.2 <= GREEN_MAX
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
    let game_id = game_split[0][5..].parse::<i32>().unwrap();
    let is_invalid_game = game_split[1].split(';').map(|s| check_validity(parse_subset(s.trim()))).any(|f| !f);
    if is_invalid_game {
        0
    } else {
        game_id
    }
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let sum = file_content.lines().map(parse_game).sum::<i32>();
    println!("{:?}", sum);
}
