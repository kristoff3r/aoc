use std::{collections::BTreeSet, io::BufRead};

fn main() {
    let data = std::fs::read("input.txt").unwrap();
    let mut x = 1;
    let mut pc = 0;

    let mut skip = 0;
    let mut instr = String::new();
    let mut lines = data.lines();

    let interesting: BTreeSet<_> = (20..=220).step_by(40).collect();
    let mut res = 0;

    let mut display = [[0x41u8; 40]; 10];

    loop {
        if skip == 0 {
            if let Some(n) = instr.strip_prefix("addx ") {
                let n = n.parse::<i64>().unwrap();
                x += n;
            }
            let tmp = lines.next();
            if tmp.is_none() {
                break;
            }
            instr = tmp.unwrap().unwrap();
            skip = match &instr[..4] {
                "noop" => 1,
                "addx" => 2,
                _ => 1,
            }
        }

        let pc_x = pc % 40;
        let pc_y = pc / 40;
        let pixel = if (pc_x - 1..=pc_x + 1).contains(&x) {
            b'#'
        } else {
            b'.'
        };
        display[pc_y as usize][pc_x as usize] = pixel;

        pc += 1;
        skip -= 1;

        if interesting.contains(&pc) {
            println!("{pc} {x} {}", pc * x);
            res += x * pc;
        }
    }

    println!("{res}");

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", display[y][x] as char);
        }
        println!()
    }
}
