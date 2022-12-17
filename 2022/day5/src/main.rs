//                         [R] [J] [W]
//             [R] [N]     [T] [T] [C]
// [R]         [P] [G]     [J] [P] [T]
// [Q]     [C] [M] [V]     [F] [F] [H]
// [G] [P] [M] [S] [Z]     [Z] [C] [Q]
// [P] [C] [P] [Q] [J] [J] [P] [H] [Z]
// [C] [T] [H] [T] [H] [P] [G] [L] [V]
// [F] [W] [B] [L] [P] [D] [L] [N] [G]
//  1   2   3   4   5   6   7   8   9

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut stacks = [
        vec!['F', 'C', 'P', 'G', 'Q', 'R'],
        vec!['W', 'T', 'C', 'P'],
        vec!['B', 'H', 'P', 'M', 'C'],
        vec!['L', 'T', 'Q', 'S', 'M', 'P', 'R'],
        vec!['P', 'H', 'J', 'Z', 'V', 'G', 'N'],
        vec!['D', 'P', 'J'],
        vec!['L', 'G', 'P', 'Z', 'F', 'J', 'T', 'R'],
        vec!['N', 'L', 'H', 'C', 'F', 'P', 'T', 'J'],
        vec!['G', 'V', 'Z', 'Q', 'H', 'T', 'C', 'W'],
    ];
    let mut stacks2 = stacks.clone();

    for mov in data.lines() {
        let (amount, rest) = mov
            .trim_start_matches("move ")
            .split_once(" from ")
            .unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let amount: usize = amount.parse().unwrap();
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();

        for _ in 0..amount {
            let tmp = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(tmp);
        }
        let mut tmp = Vec::new();
        for _ in 0..amount {
            tmp.push(stacks2[from - 1].pop().unwrap());
        }
        tmp.reverse();
        stacks2[to - 1].extend_from_slice(&tmp);
    }

    let mut res = String::new();
    let mut res2 = String::new();
    for s in stacks {
        if let Some(c) = s.last() {
            res.push(*c);
        }
    }
    for s in stacks2 {
        if let Some(c) = s.last() {
            res2.push(*c);
        }
    }
    println!("{res} {res2}");
}
