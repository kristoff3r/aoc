use std::collections::HashSet;

fn priority(c: char) -> u64 {
    let res = if c.is_uppercase() {
        c as u8 - b'A' + 27
    } else {
        c as u8 - b'a' + 1
    };
    res.into()
}

fn main() {
    let data = std::fs::read_to_string("input").unwrap();
    let mut res = 0;
    for line in data.split_ascii_whitespace() {
        let sack_len = line.len() / 2;
        let (s1, s2) = line.split_at(sack_len);

        let mut found = HashSet::new();
        for c in s1.chars() {
            if s2.contains(c) {
                found.insert(c);
            }
        }
        for c in found {
            res += priority(c);
        }
    }
    println!("{res}");

    let groups = data.split_ascii_whitespace().collect::<Vec<_>>();

    let mut res = 0;
    for group in groups.chunks(3) {
        let first = group[0];
        for c in first.chars() {
            if group[1].contains(c) && group[2].contains(c) {
                res += priority(c);
                break;
            }
        }
    }
    println!("{res}");
}
