use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
    U,
}
#[derive(Debug, Clone, Copy)]
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
    value: i64,
    // next: Option<Workflow>,
    next: String,
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    cond: Vec<Condition>,
    // default_cond: Option<Workflow>,
    default: String,
}

#[derive(Debug, Clone)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

fn create_condition(condition: &str) -> Condition {
    let values: Vec<_> = condition.split(':').collect();
    let cat = values[0].chars().next().unwrap();
    let op = values[0].chars().nth(1).unwrap();
    let num = values[0][2..].parse::<i64>().unwrap();
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

fn create_parts(content: &str) -> Vec<Part> {
    content
        .lines()
        .map(|l| {
            let values: Vec<_> = l[1..l.len() - 1]
                .split(',')
                .map(|p| p[2..].to_string().parse::<i64>().unwrap())
                .collect();
            Part {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3],
            }
        })
        .collect()
}

fn check_operator(op: Operator, a: i64, b: i64) -> bool {
    match op {
        Operator::EQ => a == b,
        Operator::LT => a < b,
        Operator::GT => a > b,
        Operator::U => true,
    }
}

fn check_condition(cond: &Condition, part: &Part) -> bool {
    match cond.cat {
        Category::X => check_operator(cond.op, part.x, cond.value),
        Category::M => check_operator(cond.op, part.m, cond.value),
        Category::A => check_operator(cond.op, part.a, cond.value),
        Category::S => check_operator(cond.op, part.s, cond.value),
        Category::U => check_operator(cond.op, part.a, cond.value), // Not possible
    }
}

fn check_part(map: &HashMap<String, Workflow>, part: &Part) -> i64 {
    let mut cur = map.get("in").unwrap();
    while cur.name != "A" && cur.name != "R" {
        let mut use_default = true;
        for cond in cur.cond.iter() {
            // println!("{:?} {:?} {:?}", cur.name, cond, part);
            if check_condition(&cond, part) {
                cur = &map[&cond.next];
                use_default = false;
                break;
            }
        }
        if use_default {
            cur = &map[&cur.default];
        }
    }

    if cur.name == "A" {
        return part.a + part.x + part.m + part.s;
    }
    0
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let values: Vec<_> = file_content.split("\n\n").collect();
    let workflows_string = values[0];
    let parts_string = values[1];
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

    let parts = create_parts(parts_string);
    let ans = parts
        .iter()
        .map(|p| check_part(&workflows_map, p))
        .sum::<i64>();
    // println!("{:?} {:?}", workflows_map, parts);
    println!("Ans: {:?}", ans);
}
