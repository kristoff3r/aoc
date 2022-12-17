use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    One(u32),
    Many(Vec<Packet>),
}

impl Default for Packet {
    fn default() -> Self {
        Self::Many(Vec::new())
    }
}

impl Packet {
    fn push(&mut self, packet: Packet) {
        if let Self::Many(inner) = self {
            inner.push(packet);
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Packet::*;

        match (self, other) {
            (One(x), One(y)) => x.partial_cmp(y),
            (Many(xs), Many(ys)) => {
                let mut i = 0;
                loop {
                    if xs.len() == i && ys.len() == i {
                        return Some(Ordering::Equal);
                    }
                    let Some(x) = xs.get(i) else { return Some(Ordering::Less); };
                    let Some(y) = ys.get(i) else { return Some(Ordering::Greater); };
                    match x.partial_cmp(y) {
                        Some(Ordering::Equal) => (),
                        res => return res,
                    }
                    i += 1;
                }
            }
            (One(x), ys) => Many(vec![One(*x)]).partial_cmp(ys),
            (xs, One(y)) => xs.partial_cmp(&Many(vec![One(*y)])),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut res1 = 0;
    let decoders = vec![
        Packet::Many(vec![Packet::Many(vec![Packet::One(2)])]),
        Packet::Many(vec![Packet::Many(vec![Packet::One(6)])]),
    ];
    let mut packets = decoders.clone();
    for (idx, case) in data.split("\n\n").enumerate() {
        let (left, right) = case.split_once('\n').unwrap();

        let [left, right] = [left, right].map(|s| {
            let mut s = s.trim().as_bytes();
            let mut stack = Vec::new();
            let mut cur: Packet = Default::default();
            let mut digits = String::new();
            while !s.is_empty() {
                match s[0] {
                    b'[' => {
                        stack.push(std::mem::take(&mut cur));
                    }
                    b']' => {
                        if !digits.is_empty() {
                            let tmp = digits.parse().unwrap();
                            digits.clear();
                            cur.push(Packet::One(tmp));
                        }
                        let mut tmp = stack.pop().unwrap();
                        tmp.push(cur);
                        cur = tmp;
                    }
                    c @ (b'0'..=b'9') => {
                        digits.push(c as char);
                    }
                    b',' => {
                        if !digits.is_empty() {
                            let tmp = digits.parse().unwrap();
                            cur.push(Packet::One(tmp));
                            digits.clear();
                        }
                    }
                    c => panic!("invalid character: {}", c as char),
                }
                s = &s[1..];
            }
            cur
        });
        let cmp = left.partial_cmp(&right);
        packets.push(left);
        packets.push(right);
        if let Some(Ordering::Less) = cmp {
            res1 += idx + 1;
        }
    }
    packets.sort();

    let mut res2 = 1;
    for d in decoders {
        res2 *= packets.iter().position(|p| p == &d).unwrap() + 1;
    }

    println!("{}", res1);
    println!("{}", res2);
}
