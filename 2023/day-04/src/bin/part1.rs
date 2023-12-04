use std::collections::HashSet;

fn get_points_line(line: &str) -> u32 {
    let numbers: Vec<&str> = line
        .split(':')
        .enumerate()
        .nth(1)
        .unwrap()
        .1
        .split('|')
        .map(str::trim)
        .collect();
    println!("{:?}", numbers);
    let winning_numbers: HashSet<u32> =
        HashSet::from_iter(numbers.first().copied().unwrap().split(' ').filter_map(
            |num| match num {
                "" => None,
                _ => Some(num.parse::<u32>().unwrap()),
            },
        ));
    let my_numbers = numbers
        .last()
        .unwrap()
        .split(' ')
        .filter_map(|num| match num {
            "" => None,
            _ => Some(num.parse::<u32>().unwrap()),
        })
        .filter(|number| winning_numbers.contains(number))
        .count() as u32;
    let base: u32 = 2;
    println!("{:?}, {:?}", winning_numbers, my_numbers);
    if my_numbers == 0 {
        0
    } else {
        base.pow(my_numbers - 1)
    }
}

fn parse_input(content: &str) -> u32 {
    content.lines().map(get_points_line).sum()
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let total = parse_input(file_content);
    println!("{:?}", total);
}
