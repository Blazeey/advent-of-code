struct Record {
    line: Vec<char>,
    groups: Vec<i32>,
    ps: Vec<bool>,
}

fn solve(record: &Record, i: i32, j: i32) -> i32 {
    println!("{:?} {:?}", i, j);
    if i < 0 || j < 0 {
        return 0;
    }
    let iu = i as usize;
    let ju = j as usize;
    let size = record.groups[iu] as usize;
    if ju < iu * 2 + size - 1 {
        return 0;
    }
    // println!("here {:?} {:?}", iu, size);
    if record.line[ju] == '.' {
        return solve(record, i, j - 1);
    }
    // let mut count = 0;
    for p in ju + 1 - size..ju + 1 {
        println!("p {:?}", p);
        if record.line[p] == '.' {
            return solve(record, i, p as i32);
        }
    }
    if ju >= size && record.line[ju - size] == '#' {
        return solve(record, i, j - 1);
    }
    if i == 0 {
        if j - 1 >= 0 && record.ps[j as usize - 1] {
            return solve(record, i, j - 1);
        } else {
            return solve(record, i, j - 1) + 1;
        }
    }
    solve(record, i, j - 1) + solve(record, i - 1, j - size as i32 - 1)
}

// fn get_count(record: &Record) -> i32 {
//     let n = record.line.len();
//     let m = record.groups.len();
//     let dp = vec![vec![0; n]; m];
//     let ps = vec![0i32; n];
//     let count = 0i32;
//     for i in 0..n {
//         if record.line[i] == '#' && record.line[i - 1] != '#' {}
//     }

//     // for i in 0..m {
//     //     let mut nondot = 0;
//     //     let size = record.groups[i] as i32;
//     //     for j in 0..n  {
//     //         // check j-1
//     //         dp[i][j] = dp[i][j-1];
//     //         if record.line[j] != '.' {
//     //             nondot += 1;
//     //         }
//     //         if j as i32 - size >= 0{
//     //             if record.line[j - size as usize ] != '.' {
//     //                 nondot -= 1;
//     //             }
//     //             if record.line[j - size as usize]  != '#' {
//     //                 //check j-size-1
//     //                 if i == 0 {
//     //                     dp[i][j] +=
//     //                 } else {

//     //                     dp[i][j] += dp[i-1][j-size as usize - 1];
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }

//     // for i in 0..n as i32 {
//     //     let mut nondot = 0;
//     //     for j in i - 1..m as i32 {
//     //         if record.line[j as usize] != '.' {
//     //             nondot += 1;
//     //         } else {
//     //             nondot = 0;
//     //         }
//     //         if j - record.groups[i as usize] as i32 >= 0 && record.line[j as usize] == '.' {
//     //             nondot -= 1;
//     //         }
//     //         if nondot == record.groups[i as usize] && {

//     //         }
//     //         if record.line[j as usize - i as usize] != '#' && record.line[j as usize + 1] != '#' {}
//     //     }
//     // }
//     0
// }

fn calcuate_prefix_sums(line: &Vec<char>) -> Vec<bool> {
    let n = line.len();
    let mut ps = vec![true; n];
    let mut found_h = false;
    let mut found = false;
    for i in 0..n {
        if line[i] == '#' {
            found_h = true;
        }
        if found_h && line[i] == '.' {
            found = true;
        }
        if found {
            ps[i] = false;
        }
    }

    // let mut i = 0;
    // let mut found_h = false;
    // for i in 0..n {
    //     if line[i] == '.' && found_h {
    //         ps[i] = count;
    //         continue;
    //     }
    //     if line[i] == '#' {
    //         found_h = true;
    //     }

    // }

    ps
}

fn parse_line(content: &str) -> Vec<Record> {
    content
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let arrangements = parts.next().unwrap().chars().collect();
            let groups: Vec<i32> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|i| i.parse::<i32>().unwrap())
                .collect();
            Record {
                ps: calcuate_prefix_sums(&arrangements),
                line: arrangements,
                groups,
            }
        })
        .collect()
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let lines = parse_line(file_content);

    // let v: Vec<_> = lines
    //     .iter()
    //     .map(|f| solve(f, f.groups.len() as i32 - 1, f.line.len() as i32 - 1))
    //     .collect();

    // println!("{:?}", v);
    // let ans: i32 = v.iter().sum();
    // let ans = solve(
    //     &lines[0],
    //     lines[0].groups.len() as i32 - 1,
    //     lines[0].line.len() as i32 - 1,
    // );
    println!(
        "{:?}",
        solve(
            &lines[1],
            lines[1].groups.len() as i32 - 1,
            lines[1].line.len() as i32 - 1,
        )
    );

    // println!("ans {:?}", ans);
}
