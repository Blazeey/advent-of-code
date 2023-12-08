use std::collections::HashMap;

fn split_edge(line: &String) -> (String, String, String) {
    let r_line = &line[..line.len() - 1];
    let start = r_line.split(" = ").next().unwrap().to_string();
    let child: Vec<_> = r_line.split(" = (").nth(1).unwrap().split(", ").collect();
    let left = child[0].to_string();
    let right = child[1].to_string();
    (start, left, right)
}

fn parse_line(content: &str) -> (String, HashMap<String, (String, String)>) {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let lines = content.lines().map(String::from).collect::<Vec<String>>();
    let path = lines[0].trim().to_owned();

    let edges = &lines[2..];
    edges.iter().for_each(|e| {
        let edge = split_edge(e);
        map.entry(edge.0).or_insert((edge.1, edge.2));
    });

    (path, map)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let (path, map) = parse_line(file_content);
    let mut length = 0;
    let mut cur_nodes = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(String::from)
        .collect::<Vec<_>>();
    // let mut cur_node = "AAA".to_owned();
    println!("{:?}", cur_nodes);
    loop {
        let c = path.chars().nth(length % path.len()).unwrap();
        cur_nodes = cur_nodes
            .iter()
            .map(|s| {
                let l = map.get(s).unwrap();
                match c {
                    'L' => l.0.to_string(),
                    'R' => l.1.to_string(),
                    _ => l.0.to_string(),
                }
            })
            .collect::<Vec<_>>();

        length += 1;
        let ends = cur_nodes.iter().all(|s| s.ends_with('Z'));
        if ends {
            break;
        }
    }

    // This is the answer
    println!("{:?}", lcm(&[21389, 23147, 19631, 12599, 17873, 20803]));
}
