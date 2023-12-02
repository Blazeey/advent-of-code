
fn parse_line(line: &str) -> u32 {
    let mut digits: Vec<u32> = vec![];
    (0..line.len()).for_each(|i| {
        let c = line.chars().nth(i).unwrap();
        let cur_str = &line[i..];
        match c {
            '0' => digits.push(0),
            '1' => digits.push(1),
            '2' => digits.push(2),
            '3' => digits.push(3),
            '4' => digits.push(4),
            '5' => digits.push(5),
            '6' => digits.push(6),
            '7' => digits.push(7),
            '8' => digits.push(8),
            '9' => digits.push(9),
            _ => (),
        }
        if cur_str.starts_with("zero") { digits.push(0) }
        else if cur_str.starts_with("one") { digits.push(1) }
        else if cur_str.starts_with("two") { digits.push(2) }
        else if cur_str.starts_with("three") { digits.push(3) }
        else if cur_str.starts_with("four") { digits.push(4) }
        else if cur_str.starts_with("five") { digits.push(5) }
        else if cur_str.starts_with("six") { digits.push(6) }
        else if cur_str.starts_with("seven") { digits.push(7) }
        else if cur_str.starts_with("eight") { digits.push(8) }
        else if cur_str.starts_with("nine") { digits.push(9) }
    });
    digits[0] * 10 + digits[digits.len()-1]
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let sum = file_content.lines().map(parse_line).sum::<u32>();
    println!("{:?}", sum);
}
