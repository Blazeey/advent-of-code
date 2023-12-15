use std::collections::HashMap;

fn hash(text: &str) -> u32 {
    text.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn add(list: &mut Vec<(String, u32)>, label: &str, focal: u32) {
    let found = list.iter().enumerate().find(|(_, f)| f.0 == label);
    if found.is_none() {
        list.push((label.to_owned(), focal));
    } else {
        let idx = found.unwrap().0;
        list.get_mut(idx).unwrap().1 = focal;
        // found.unwrap().1 = focal;
    }
}

fn remove(list: &mut Vec<(String, u32)>, label: &str) {
    let found = list.iter().enumerate().find(|(_, f)| f.0 == label);
    if let Some((idx, _)) = found {
        list.remove(idx);
    }
}

fn calculate(list: &[(String, u32)], b: u32) -> u32 {
    list.iter()
        .enumerate()
        .map(|(i, (_, f))| (i as u32 + 1) * *f * b)
        .sum::<u32>()
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let mut hash_map: HashMap<u32, Vec<(String, u32)>> = HashMap::new();
    (0..256).for_each(|i| {
        hash_map.insert(i, Vec::new());
    });
    file_content.split(',').for_each(|f| {
        let op_idx = *f.find('-').get_or_insert_with(|| f.find('=').unwrap());
        let label = &f[..op_idx];
        let h = hash(label);
        let op = &f[op_idx..op_idx + 1];
        match op {
            "=" => {
                let focal = &f[op_idx + 1_usize..].parse::<u32>().unwrap();
                let list = hash_map.get_mut(&h).unwrap();
                add(list, label, *focal);
            }
            "-" => {
                remove(hash_map.get_mut(&h).unwrap(), label);
            }
            _ => (),
        }
    });

    let ans: u32 = hash_map.iter().map(|(k, v)| calculate(v, *k + 1)).sum();
    println!("Ans: {:?}", ans);
}
