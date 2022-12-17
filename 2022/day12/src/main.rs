use std::{
    collections::{BTreeMap, VecDeque},
    io::BufRead,
};

fn main() {
    let data = std::fs::read("input.txt").unwrap();
    let mut grid = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut starters = Vec::new();

    for (y, line) in data.lines().enumerate() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for (x, c) in line.trim().as_bytes().iter().enumerate() {
            match c {
                b'S' => {
                    start = (x, y);
                    starters.push((x, y));
                    row.push(b'a');
                }
                b'E' => {
                    end = (x, y);
                    row.push(b'z');
                }
                &c => {
                    if c == b'a' {
                        starters.push((x, y));
                    }
                    row.push(c)
                }
            }
        }
        grid.push(row);
    }

    let res1 = bfs(&grid, start, end);
    println!("{:?}", res1);

    let res2 = starters.into_iter().flat_map(|s| bfs(&grid, s, end)).min();
    println!("{:?}", res2);
}

fn bfs(grid: &[Vec<u8>], start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_front(start);
    let mut distance: BTreeMap<(usize, usize), usize> = BTreeMap::new();

    while let Some(pos) = queue.pop_back() {
        if pos == goal {
            return distance.get(&goal).cloned();
        }
        let cur_val = grid[pos.1][pos.0];
        let cur_dist = distance.get(&pos).cloned().unwrap_or_default();
        for (i, j) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nb = ((pos.0 as i64 + i) as usize, (pos.1 as i64 + j) as usize);
            if !distance.contains_key(&nb) {
                let Some(v) = grid.get(nb.1).and_then(|row| row.get(nb.0)).cloned() else { continue; };
                if v <= cur_val + 1 {
                    let dist = distance.entry(nb).or_insert(usize::MAX);
                    if cur_dist + 1 < *dist {
                        *dist = cur_dist + 1;
                    }
                    queue.push_front(nb);
                }
            }
        }
    }

    None
}
