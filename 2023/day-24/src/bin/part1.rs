#[derive(Debug, Clone, Copy)]
struct Line {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Line {
    fn from(values: &[&str]) -> Line {
        let points: Vec<_> = values
            .iter()
            .map(|v| v.trim().parse::<f64>().unwrap())
            .collect();
        Line {
            x: points[0],
            y: points[1],
            z: points[2],
            vx: points[3],
            vy: points[4],
            vz: points[5],
        }
    }
}

fn intersect(a: &Line, b: &Line) -> bool {
    let x1 = a.x;
    let y1 = a.y;
    let x2 = b.x;
    let y2 = b.y;
    let x1v = a.vx;
    let y1v = a.vy;
    let x2v = b.vx;
    let y2v = b.vy;

    let rangemin = 200000000000000f64;
    let rangemax = 400000000000000f64;

    // let rangemin = 7;
    // let rangemax = 27;

    let t1num = x2v * y1 - x2v * y2 - x1 * y2v + x2 * y2v;
    let t2num = x1v * y1 - x1 * y1v + x2 * y1v - x1v * y2;
    let mut det = x1v * y2v - x2v * y1v;
    if det > 0.0 && (t1num <= 0.0 || t2num <= 0.0) {
        return false;
    }
    if det < 0.0 && (t1num >= 0.0 || t2num >= 0.0) {
        return false;
    }
    if det == 0.0 {
        return false;
    }
    let mut xnum = -(x1 * x2v * y1v) + x1v * x2v * (y1 - y2) + x1v * x2 * y2v;
    let mut ynum = -(x2v * y1v * y2) + x1v * y1 * y2v + (-x1 + x2) * y1v * y2v;
    if det < 0.0 {
        det = -det;
        xnum = -xnum;
        ynum = -ynum;
    }
    (rangemin * det <= xnum)
        && (xnum <= rangemax * det)
        && (rangemin * det <= ynum)
        && (ynum <= rangemax * det)
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let lines: Vec<Line> = file_content
        .lines()
        .map(|l| {
            let values: Vec<_> = l.split('@').collect();
            let l: Vec<&str> = [
                values[0].split(',').collect::<Vec<_>>(),
                values[1].split(',').collect::<Vec<_>>(),
            ]
            .concat();
            Line::from(&l)
        })
        .collect();

    let mut count = 0;
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if intersect(&lines[i], &lines[j]) {
                count += 1;
            }
        }
    }
    println!("{count}");
}
