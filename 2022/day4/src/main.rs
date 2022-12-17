use std::io::BufRead;

fn main() {
    let data = std::fs::read("input").unwrap();
    let mut res1 = 0;
    let mut res2 = 0;
    for pair in data.lines() {
        let ps = pair
            .unwrap()
            .split(',')
            .map(|p| {
                p.split('-')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for i in 0..2 {
            if ps[i][0] <= ps[(i + 1) % 2][0] && ps[i][1] >= ps[(i + 1) % 2][1] {
                res1 += 1;
                break;
            }
        }
        let mut non_overlap = false;
        for i in 0..2 {
            if ps[i][0] > ps[(i + 1) % 2][1] {
                non_overlap = true;
            }
        }
        if !non_overlap {
            res2 += 1;
        }
    }
    println!("{res1} {res2}");
}
