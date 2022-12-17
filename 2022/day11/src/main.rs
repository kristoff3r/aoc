#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Vec<String>,
    test: i64,
    tcase: usize,
    fcase: usize,
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let monkey_data = data.split("\n\n");

    let mut monkeys = Vec::new();
    for m in monkey_data {
        let mut lines = m.lines();
        lines.next();
        let items = lines
            .next()
            .unwrap()
            .trim_start()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let operation = lines
            .next()
            .unwrap()
            .trim_start()
            .strip_prefix("Operation: new = ")
            .unwrap()
            .split(' ')
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        let test = lines
            .next()
            .unwrap()
            .trim_start()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let tcase = lines
            .next()
            .unwrap()
            .trim_start()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let fcase = lines
            .next()
            .unwrap()
            .trim_start()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            tcase,
            fcase,
        })
    }

    let n: i64 = monkeys.iter().map(|m| m.test).product();

    let mut thrown = Vec::new();
    let mut inspected = vec![0; monkeys.len()];
    for _r in 0..10000 {
        for i in 0..monkeys.len() {
            if let Some(m) = monkeys.get_mut(i) {
                inspected[i] += m.items.len();
                for item in m.items.drain(..) {
                    let [x, y] = [&m.operation[0], &m.operation[2]]
                        .map(|x| x.parse::<i64>().unwrap_or(item));
                    let item = match m.operation[1].as_str() {
                        "+" => x + y,
                        "*" => x * y,
                        _ => panic!("invalid op"),
                    };
                    let item = item % n;
                    let receiver = if item % m.test == 0 { m.tcase } else { m.fcase };
                    thrown.push((receiver, item));
                }
            }
            for (m, i) in thrown.drain(..) {
                monkeys[m].items.push(i);
            }
        }
    }
    inspected.sort();
    inspected.reverse();

    println!("{}", inspected[0] * inspected[1]);
}
