use std::{cmp::Ordering, collections::HashMap};

const CHAR_ORDER: &str = "J23456789TQKA";

fn parse_lines(content: &str) -> Vec<(String, u64, u64)> {
    content
        .lines()
        .map(|line| {
            let hand = line.split(' ').next().unwrap().to_string();
            let bid = line.split(' ').nth(1).unwrap().parse::<u64>().unwrap();
            let rank: u64 = get_rank(&hand) as u64;
            (hand, bid, rank)
        })
        .collect::<Vec<(String, u64, u64)>>()
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
    for (k, c) in hand_counts {
        if *c == count && *k != 'J' {
            total_counts += 1;
        }
    }
    total_counts
}

fn old_ranking(
    five_count: u32,
    four_count: u32,
    three_count: u32,
    two_count: u32,
    one_count: u32,
) -> u32 {
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

fn get_rank(hand: &str) -> u32 {
    let letter_counts: HashMap<char, u32> = hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let j_counts = letter_counts.get(&'J');

    let five_count = get_chars_with_count(&letter_counts, 5);
    let four_count = get_chars_with_count(&letter_counts, 4);
    let three_count = get_chars_with_count(&letter_counts, 3);
    let two_count = get_chars_with_count(&letter_counts, 2);
    let one_count = get_chars_with_count(&letter_counts, 1);
    if j_counts.is_none() || *j_counts.unwrap() == 0 {
        return old_ranking(five_count, four_count, three_count, two_count, one_count);
    }
    let total_j = *j_counts.unwrap();

    if total_j == 5 {
        return 7;
    }
    if five_count > 0 {
        println!("THIS SHOULDN'T HAPPEN");
    } else if four_count > 0 {
        return 7;
    } else if three_count > 0 {
        if total_j == 2u32 {
            return 7;
        } else {
            return 6;
        }
    } else if two_count > 0 {
        match total_j {
            1 => {
                if two_count == 2 {
                    return 5;
                }
                return 4;
            }
            2 => return 6,
            3 => return 7,
            _ => (),
        }
    } else if one_count > 0 {
        match total_j {
            1 => return 2,
            2 => return 4,
            3 => return 6,
            4 => return 7,
            _ => (),
        }
    }
    7
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let mut lines = parse_lines(file_content);

    lines.sort_by(|(a, _, rank_a), (b, _, rank_b)| {
        if rank_a == rank_b {
            return compare_string(a.as_str(), b.as_str());
        }
        rank_a.cmp(rank_b)
    });

    let total = lines
        .iter()
        .enumerate()
        .map(|(idx, (_, bid, _))| (idx + 1) as u64 * bid)
        .sum::<u64>();
    println!("{:?}", total);
}
