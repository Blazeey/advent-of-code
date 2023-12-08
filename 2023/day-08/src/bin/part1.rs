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

fn main() {
    let file_content = include_str!("../../input1.txt");
    let (path, map) = parse_line(file_content);
    let mut length = 0;
    //DVA MPA TDA AAA FJA XPA
    let mut cur_node = "AAA".to_owned();
    loop {
        let c = path.chars().nth(length % path.len()).unwrap();
        let l = map.get(&cur_node).unwrap();
        match c {
            'L' => cur_node = l.0.to_string(),
            'R' => cur_node = l.1.to_string(),
            _ => (),
        }

        length += 1;
        if cur_node.ends_with('Z') {
            break;
        }
    }

    println!("{:?}", length);
}
