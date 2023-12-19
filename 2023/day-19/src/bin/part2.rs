use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
    U,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    EQ,
    LT,
    GT,
    U,
}

#[derive(Debug, Clone)]
struct Condition {
    cat: Category,
    op: Operator,
    value: u64,
    next: String,
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    cond: Vec<Condition>,
    default: String,
}

fn create_condition(condition: &str) -> Condition {
    let values: Vec<_> = condition.split(':').collect();
    let cat = values[0].chars().next().unwrap();
    let op = values[0].chars().nth(1).unwrap();
    let num = values[0][2..].parse::<u64>().unwrap();
    let next = values[1].to_string();
    Condition {
        cat: match cat {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => Category::U,
        },
        op: match op {
            '<' => Operator::LT,
            '>' => Operator::GT,
            '=' => Operator::EQ,
            _ => Operator::U,
        },
        value: num,
        next,
    }
}

fn create_workflows(content: &str) -> Vec<Workflow> {
    let workflows: Vec<_> = content
        .lines()
        .map(|line| {
            let name_split: Vec<_> = line.split('{').collect();
            let name = name_split[0].to_string();
            let conditions: Vec<_> = name_split[1][..name_split[1].len() - 1]
                .split(',')
                .collect();
            let cond = conditions[..conditions.len() - 1]
                .iter()
                .map(|c| create_condition(c))
                .collect();
            let default = conditions.iter().next_back().unwrap().to_string();

            Workflow {
                name,
                cond,
                default,
            }
        })
        .collect();

    workflows
}

fn invert_condition(cond: &Condition) -> Condition {
    let op = if cond.op == Operator::LT {
        Operator::GT
    } else {
        Operator::LT
    };
    let value = if cond.op == Operator::LT {
        cond.value - 1
    } else {
        cond.value + 1
    };
    Condition {
        cat: cond.cat,
        op,
        value,
        next: cond.next.to_string(),
    }
}

fn merge_ranges(cond: &mut [&Condition]) -> u64 {
    if cond.is_empty() {
        return 4000;
    }
    cond.sort_by(|a, b| a.value.cmp(&b.value));
    let mut total = 0;
    let mut cur_lower = 0;
    let mut cur_op = Operator::GT;
    for c in cond.iter() {
        // println!("{:?} {:?} {:?} {:?}", total, cur_lower, cur_op, c);
        if c.op == Operator::LT && cur_op == Operator::GT {
            total += c.value - cur_lower - 1;
        }
        cur_op = c.op;
        cur_lower = c.value;
    }
    if cur_op == Operator::GT && cur_lower != 0 {
        total += 4000 - cur_lower;
    }
    total
}

fn dfs(work: &Workflow, map: &HashMap<String, Workflow>, all_cond: &[Condition]) -> u64 {
    // println!("{:?} {:?}", work, all_cond);
    if work.name == "A" {
        let mut x_v: Vec<_> = all_cond.iter().filter(|c| c.cat == Category::X).collect();
        let x = merge_ranges(&mut x_v);

        let mut m_v: Vec<_> = all_cond.iter().filter(|c| c.cat == Category::M).collect();
        let m = merge_ranges(&mut m_v);

        let mut a_v: Vec<_> = all_cond.iter().filter(|c| c.cat == Category::A).collect();
        let a = merge_ranges(&mut a_v);

        let mut s_v: Vec<_> = all_cond.iter().filter(|c| c.cat == Category::S).collect();
        let s = merge_ranges(&mut s_v);

        // println!("ADDING {:?} {:?} {:?} {:?} {:?}", x, m, a, s, x * m * a * s);
        return x * m * a * s;
    } else if work.name == "R" {
        return 0;
    }
    let mut cur_work_conds = vec![];
    let mut total = 0;
    for c in work.cond.iter() {
        cur_work_conds.push(c.to_owned());
        total += dfs(&map[&c.next], map, &[all_cond, &cur_work_conds].concat());
        cur_work_conds.pop();
        cur_work_conds.push(invert_condition(c));
    }
    total += dfs(
        &map[&work.default],
        map,
        &[all_cond, &cur_work_conds].concat(),
    );
    total
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let values: Vec<_> = file_content.split("\n\n").collect();
    let workflows_string = values[0];
    let workflows = create_workflows(workflows_string);

    let mut workflows_map = HashMap::new();
    for w in workflows.iter() {
        workflows_map.insert(w.name.to_owned(), w.to_owned());
    }

    let a_workflow = Workflow {
        name: "A".to_string(),
        cond: vec![],
        default: String::default(),
    };

    let r_workflow = Workflow {
        name: "R".to_string(),
        cond: vec![],
        default: String::default(),
    };

    workflows_map.insert("A".to_string(), a_workflow);
    workflows_map.insert("R".to_string(), r_workflow);

    let all_cond = vec![];
    let ans = dfs(workflows_map.get("in").unwrap(), &workflows_map, &all_cond);
    println!("Ans: {:?}", ans);
}
