use std::cmp::min;
use std::collections::HashSet;

fn get_points_line(line: &str, frequency: &mut Vec<u32>, line_num: usize, max_lines: usize) -> u32 {
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
    let start_idx = min(line_num + 1, max_lines);
    let end_idx = min(line_num + my_numbers as usize + 1, max_lines);
    println!("{:?}, {:?}, {:?}", start_idx, end_idx, my_numbers);
    (start_idx..end_idx).for_each(|i| {
        frequency[i] += frequency[line_num];
    });
    println!("{:?}", frequency);
    frequency[line_num]
}

fn parse_input(content: &str) -> u32 {
    let lines = content.lines();
    let num_lines = lines.clone().count();
    let mut frequency_vec: Vec<u32> = vec![1; num_lines];
    lines
        .enumerate()
        .map(|(line_num, line)| get_points_line(line, &mut frequency_vec, line_num, num_lines))
        .sum()
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let total = parse_input(file_content);
    println!("{:?}", total);
}
