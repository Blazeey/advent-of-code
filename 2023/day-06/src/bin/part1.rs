fn parse_numbers(content: &str) -> Vec<i64> {
    content
        .split(' ')
        .filter_map(|s| {
            let val = s.trim();
            match val {
                "" => None,
                _ => Some(val.parse::<i64>().unwrap()),
            }
        })
        .collect::<Vec<_>>()
}

fn parse_lines(content: &str) -> Vec<(i64, i64)> {
    let lines = content.lines().collect::<Vec<_>>();
    let times = lines[0].split("Time:").nth(1).map(parse_numbers).unwrap();
    let distance = lines[1]
        .split("Distance:")
        .nth(1)
        .map(parse_numbers)
        .unwrap();
    (0..times.len())
        .map(|i| (times[i], distance[i]))
        .collect::<Vec<_>>()
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
    let file_content = include_str!("../../input1.txt");
    let races = parse_lines(file_content);
    let scores: i64 = races.iter().map(|r| compute_race_score(*r)).product();
    println!("{:?}, {:?}", races, scores);
}
