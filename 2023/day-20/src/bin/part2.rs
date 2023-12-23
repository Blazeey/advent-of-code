use std::{
    borrow::{Borrow, BorrowMut},
    collections::{HashMap, HashSet, VecDeque},
};

#[derive(Debug, Clone)]
enum ModuleType {
    FF,
    B,
    C,
    O,
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    name: String,
    state: bool,
    to: Vec<String>,
    input: HashMap<String, bool>,
}

fn parse_line(line: &str) -> Module {
    let idx = line.find('-').unwrap();
    let mut name = "";
    let mut state = true;
    let mut next = vec![];
    let mut module_type = ModuleType::B;
    match line.chars().next().unwrap() {
        'b' => {
            name = "broadcaster";
            module_type = ModuleType::B;
        }
        '%' => {
            name = &line[1..idx - 1];
            state = false;
            module_type = ModuleType::FF;
        }
        '&' => {
            name = &line[1..idx - 1];
            module_type = ModuleType::C;
        }
        _ => (),
    }

    next = line[idx + 3..]
        .split(',')
        .map(|n| n.trim().to_string())
        .collect();
    Module {
        module_type,
        name: name.to_string(),
        state,
        to: next,
        input: HashMap::new(),
    }
}

fn solve_iter(module_map: &mut HashMap<String, Module>) -> bool {
    let mut low_total = 0;
    let mut high_total = 0;

    let mut queue: VecDeque<(String, String, bool, u64)> = VecDeque::new();
    queue.push_back(("broadcaster".to_owned(), "broadcaster".to_owned(), false, 0));
    let mut l = 1;
    let mut cycle_set = HashSet::new();
    while !queue.is_empty() {
        // println!("{:?}", queue.front());
        let (cur, prev, pulse, len) = queue.pop_front().unwrap();
        if cur == "rx" && !pulse {
            return true;
        }
        let module = module_map.get_mut(&cur).unwrap();
        match module.module_type {
            ModuleType::FF => {
                if !pulse {
                    match module.state {
                        true => {
                            module.state = false;
                        }
                        false => {
                            module.state = true;
                        }
                    }

                    for next in &module.to {
                        queue.push_back((next.to_owned(), cur.to_owned(), module.state, len + 1));
                    }
                }
            }
            ModuleType::B => {
                for next in &module.to {
                    queue.push_back((next.to_owned(), cur.to_owned(), false, len + 1));
                }
            }
            ModuleType::C => {
                module.input.insert(prev.to_owned(), pulse);
                let op = module.input.values().all(|f| *f);

                // vec!["ft", "qr", "lk", "lz"])
                // if op && (cur == "ft" || cur == "qr" || cur == "lk" || cur == "lz") {
                //     if !cycle_map.contains_key(&cur) {
                //         cycle_map.insert(cur.clone(), vec![0]);
                //     }
                //     let x = cycle_map.get_mut(&cur).unwrap();
                //     x.push(len - x.iter().next_back().unwrap());
                //     println!("{:?} {:?} {:?} {:?}", cur, prev, l, len);
                //     if x.len() % 1000 == 0 {
                //         println!("{:?}", x);
                //     }
                // }
                if !cycle_set.contains(&cur) && !pulse {
                    println!("{cur} {l}");
                    cycle_set.insert(cur.to_string());
                }
                for next in &module.to {
                    queue.push_back((next.to_owned(), cur.to_owned(), !op, len + 1));
                }
            }
            ModuleType::O => {}
        }
        if queue.is_empty() {
            queue.push_back((
                "broadcaster".to_owned(),
                "broadcaster".to_owned(),
                false,
                len + 1,
            ));
        }
        l += 1;
    }
    false
}

fn main() {
    let file_content = include_str!("../../input3.txt");

    let mut module_map: HashMap<String, Module> = file_content
        .lines()
        .map(|l| {
            let module = parse_line(l);
            (module.name.to_owned(), module)
        })
        .collect();

    for module in module_map.clone().values() {
        for next in &module.to {
            // println!("next {:?}", next);
            let mut m = module_map.get_mut(next);
            if m.is_none() {
                module_map.insert(
                    next.to_string(),
                    Module {
                        module_type: ModuleType::O,
                        name: next.to_string(),
                        state: true,
                        to: vec![],
                        input: HashMap::new(),
                    },
                );
            }
            m = module_map.get_mut(next);
            m.unwrap().input.insert(module.name.to_owned(), false);
        }
    }
    println!("{:?}", module_map);

    let mut presses: u64 = 1;
    // loop {
    // if presses % 100000 == 0 {
    //     println!("{:?}", presses);
    // }
    let visited = solve_iter(&mut module_map);
    //     if visited {
    //         break;
    //     }
    //     presses += 1;
    // }

    println!("{:?}", presses);
}

/*
28104 - 30460
58564 - 29421
87985 - 30583
118568

1146381
1178273
*/
