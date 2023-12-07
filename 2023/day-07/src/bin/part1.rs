use std::{cmp::Ordering, collections::HashMap};

const CHAR_ORDER: &str = "23456789TJQKA";

fn parse_lines(content: &str) -> Vec<(String, u64)> {
    content
        .lines()
        .map(|line| {
            let hand = line.split(' ').next().unwrap().to_string();
            let bid = line.split(' ').nth(1).unwrap().parse::<u64>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<(String, u64)>>()
}

fn compare_string(a: &str, b: &str) -> Ordering {
    for i in 0..a.len() {
        let a_idx = CHAR_ORDER.find(a.chars().nth(i).unwrap()).unwrap();
        let b_idx = CHAR_ORDER.find(b.chars().nth(i).unwrap()).unwrap();
        if a_idx != b_idx {
            return a_idx.cmp(&b_idx);
        }
    }
    Ordering::Less
}

fn get_chars_with_count(hand_counts: &HashMap<char, u32>, count: u32) -> u32 {
    let mut total_counts = 0;
    for c in hand_counts.values() {
        if *c == count {
            total_counts += 1;
        }
    }
    total_counts
}

fn get_rank(hand: String) -> u32 {
    let letter_counts: HashMap<char, u32> = hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    let five_count = get_chars_with_count(&letter_counts, 5);
    let four_count = get_chars_with_count(&letter_counts, 4);
    let three_count = get_chars_with_count(&letter_counts, 3);
    let two_count = get_chars_with_count(&letter_counts, 2);
    let one_count = get_chars_with_count(&letter_counts, 1);

    if five_count > 0 {
        return 7;
    } else if four_count > 0 {
        return 6;
    } else if three_count > 0 {
        if two_count > 0 {
            return 5;
        }
        return 4;
    } else if two_count == 2 {
        return 3;
    } else if two_count == 1 {
        return 2;
    } else if one_count == 5 {
        return 1;
    }
    0
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let mut lines = parse_lines(file_content);

    lines.sort_by(|(a, _), (b, _)| {
        let rank_a = get_rank(a.to_owned());
        let rank_b = get_rank(b.to_owned());
        if rank_a == rank_b {
            return compare_string(a.as_str(), b.as_str());
        }
        rank_a.cmp(&rank_b)
    });

    let total = lines
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) as u64 * bid)
        .sum::<u64>();
    println!("{:?}, {:?}", lines, total);
}
