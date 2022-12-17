use std::collections::HashSet;

fn main() {
    let stream = std::fs::read_to_string("input.txt").unwrap();

    for size in [4, 14] {
        let res = stream
            .as_bytes()
            .windows(size)
            .take_while(|w| w.iter().collect::<HashSet<_>>().len() != size)
            .count()
            + size;
        println!("{res}");
    }
}
