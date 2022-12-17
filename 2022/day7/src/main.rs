use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Debug)]
struct Dir {
    size: u64,
    children: BTreeMap<String, Box<Dir>>,
}

fn add_size(node: &mut Dir, mut path: Vec<String>, size: u64) {
    if let Some(child) = path.pop() {
        let next = node.children.entry(child).or_default();
        add_size(next, path, size);
    } else {
        node.size = size;
    }
}

fn calculate_sizes(node: &mut Dir) -> u64 {
    for child in node.children.values_mut() {
        node.size += calculate_sizes(child);
    }
    return node.size;
}

fn solve1(node: &Dir, res: &mut u64) {
    for child in node.children.values() {
        solve1(child, res);
    }
    if node.size < 100000 {
        *res += node.size;
    }
}

fn solve2(node: &Dir, target: u64, res: &mut Option<u64>) {
    for child in node.children.values() {
        solve2(child, target, res);
    }
    if node.size >= target && node.size < res.unwrap_or(u64::MAX) {
        *res = Some(node.size);
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut sizes = BTreeMap::new();
    let mut pwd = Vec::new();
    let mut seen = BTreeSet::new();
    for line in data.trim().lines() {
        if let Some(path) = line.strip_prefix("$ cd ") {
            match path {
                ".." => {
                    pwd.pop();
                }
                "/" => {
                    pwd.clear();
                }
                child => {
                    pwd.push(child.to_string());
                }
            }
            sizes.entry(pwd.clone()).or_default();
        }
        if !line.starts_with("$") {
            let mut tmp = pwd.clone();
            if let Some(dir) = line.strip_prefix("dir ") {
                tmp.push(dir.to_string());
                sizes.entry(tmp).or_default();
            } else {
                let (size, name) = line.split_once(" ").unwrap();
                let size = size.parse::<u64>().unwrap();
                let mut tmp = pwd.clone();
                tmp.push(name.to_string());
                if seen.insert(tmp) {
                    let entry = sizes.get_mut(&pwd).unwrap();
                    *entry += size;
                }
            }
        }
    }

    let mut tree = Dir::default();
    for (mut path, size) in sizes {
        path.reverse();
        add_size(&mut tree, path, size);
    }
    calculate_sizes(&mut tree);
    let mut res = 0;
    solve1(&mut tree, &mut res);
    let mut res2 = None;
    let unused = 70000000 - tree.size;
    let target = 30000000 - unused;
    solve2(&mut tree, target, &mut res2);
    println!("{res} {res2:?}");
}
