use std::collections::BTreeSet;

fn calc_tail(head: &(i64, i64), tail: &mut (i64, i64)) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dx.abs() + dy.abs() > 2 {
        tail.0 += dx.signum();
        tail.1 += dy.signum();
    } else {
        if dx.abs() > 1 {
            tail.0 += dx.signum();
        }
        if dy.abs() > 1 {
            tail.1 += dy.signum();
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut head = (0i64, 0i64);
    let mut tails = vec![(0, 0); 9];
    let mut positions = vec![BTreeSet::new(); 9];
    for (dir, n) in data.lines().map(|l| {
        let (dir, n) = l.split_once(" ").unwrap();
        let n = n.parse::<i64>().unwrap();
        (dir, n)
    }) {
        for _ in 0..n {
            match dir {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => panic!(),
            }

            calc_tail(&head, &mut tails[0]);
            positions[0].insert(tails[0]);
            for i in 1..tails.len() {
                let (first, last) = tails.split_at_mut(i);
                let head = first.last().unwrap();
                let tail = last.first_mut().unwrap();
                calc_tail(&head, tail);
                positions[i].insert(tails[i]);
            }
        }
    }

    println!("{}", positions[0].len());
    println!("{}", positions[7].len());
}
