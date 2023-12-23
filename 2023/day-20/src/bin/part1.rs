use std::{
    borrow::{Borrow, BorrowMut},
    collections::{HashMap, VecDeque},
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

fn solve_iter(module_map: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut low_total = 0;
    let mut high_total = 0;

    let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();
    queue.push_back(("broadcaster".to_owned(), "broadcaster".to_owned(), false));
    while !queue.is_empty() {
        println!("{:?}", queue.front());
        let (cur, prev, pulse) = queue.pop_front().unwrap();
        if pulse {
            high_total += 1;
        } else {
            low_total += 1;
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
                        queue.push_back((next.to_owned(), cur.to_owned(), module.state));
                    }
                }
            }
            ModuleType::B => {
                for next in &module.to {
                    queue.push_back((next.to_owned(), cur.to_owned(), false));
                }
            }
            ModuleType::C => {
                module.input.insert(prev.to_owned(), pulse);
                let op = module.input.values().all(|f| *f);

                for next in &module.to {
                    queue.push_back((next.to_owned(), cur.to_owned(), !op));
                }
            }
            ModuleType::O => {}
        }
    }
    (low_total, high_total)
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
    // module_map.values().into_iter().for_each(|m| {
    //     for next in m.to.iter() {
    //         let mut module = module_map.get_mut(next).unwrap();

    //         module_map[next].input.insert(m.name, false);
    //     }
    // });

    let all = (0..1000)
        .map(|_| solve_iter(&mut module_map))
        .fold((0, 0), |acc, cur| (acc.0 + cur.0, acc.1 + cur.1));
    // let ans = solve_iter(&mut module_map);
    println!("{:?}", all.0 * all.1);
}
