fn solve_line(line: Vec<i64>) -> i64 {
    let mut next_line: Vec<i64> = vec![];
    for (i, x) in line.iter().enumerate() {
        if i > 0 {
            next_line.push(x - line[i - 1]);
        }
    }
    println!("{:?}", next_line);
    let first = next_line[0];
    let complete = next_line.iter().all(|val| *val == first);
    if complete {
        return line.first().unwrap() - first;
    }
    let sub_value = solve_line(next_line);
    println!("{:?} {:?}", *line.last().unwrap(), sub_value);
    *line.first().unwrap() - sub_value
}

fn parse_line(content: &str) -> i64 {
    let line: Vec<i64> = content
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    solve_line(line)
}

fn parse_lines(content: &str) -> i64 {
    content.lines().map(parse_line).sum()
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let sum = parse_lines(file_content);
    println!("{:?}", sum);
}
