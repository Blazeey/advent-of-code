fn parse_numbers(content: &str) -> i64 {
    content
        .split(' ')
        .filter_map(|s| {
            let val = s.trim();
            match val {
                "" => None,
                _ => Some(val.to_string()),
            }
        })
        .fold("".to_string(), |acc, val| {
            let mut s = acc.to_string();
            s.push_str(&val);
            s
        })
        .parse::<i64>()
        .unwrap()
}

fn parse_lines(content: &str) -> (i64, i64) {
    let lines = content.lines().collect::<Vec<_>>();
    let time = lines[0].split("Time:").nth(1).map(parse_numbers).unwrap();
    let distance = lines[1]
        .split("Distance:")
        .nth(1)
        .map(parse_numbers)
        .unwrap();
    (time, distance)
}

fn compute_race_score(race: (i64, i64)) -> i64 {
    println!("{:?}", race);
    (1..race.0)
        .map(|i| {
            // println!("{:?}", (race.0 - i) * i);
            (race.0 - i) * i
        })
        .filter(|score| score > &race.1)
        .count() as i64
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let races = parse_lines(file_content);
    let scores: i64 = compute_race_score(races);
    println!("{:?}, {:?}", races, scores);
}
