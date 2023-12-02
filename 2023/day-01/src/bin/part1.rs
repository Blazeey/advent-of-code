
fn parse_line(line: &str) -> u32 {
    let digits: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).map(|c| c.to_digit(10).unwrap()).collect();
    digits[0] * 10 + digits[digits.len()-1]
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let sum = file_content.lines().map(parse_line).sum::<u32>();
    println!("{:?}", sum);
}
