fn get_load(grid: &[Vec<char>], col: usize) -> usize {
    let v = grid
        .iter()
        .map(|l| l[col])
        .enumerate()
        .fold((0, 0), |(total, cur), (i, c)| match c {
            '#' => {
                println!("{:?}, {:?}, {:?}, {:?}", total, cur, i, c);
                if cur == 0 {
                    return (total, 0);
                }
                let t = (i * (i + 1)) / 2 - (((i - cur) * (i - cur + 1)) / 2);
                (total + t, 0)
            }
            'O' => (total, cur + 1),
            _ => (total, cur),
        });
    println!("{:?}", v);
    v.0
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let mut grids: Vec<Vec<char>> = file_content.lines().map(|l| l.chars().collect()).collect();
    grids.reverse();
    grids.push(vec!['#'; grids[0].len()]);
    let ans: usize = (0..grids[0].len()).map(|col| get_load(&grids, col)).sum();
    println!("Ans: {:?}", ans);
}
