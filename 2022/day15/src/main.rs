use std::collections::BTreeSet;

fn main() {
    let file = "input.txt";
    let data = std::fs::read_to_string(file).unwrap();
    let target_row: i64;
    let bound: i64;
    if file == "input.txt" {
        target_row = 2000000;
        bound = 4000000;
    } else {
        target_row = 10;
        bound = 20;
    }

    let mut sensors = Vec::new();
    for l in data.lines() {
        let coords = l
            .split('=')
            .map(|p| p.trim_matches(|c| c != '-' && !('0'..='9').contains(&c)))
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        sensors.push(coords);
    }

    let mut row_coords = BTreeSet::new();
    let mut sensor_areas = Vec::new();
    let mut beacons = BTreeSet::new();
    for coords in sensors {
        let [sx, sy, bx, by] = coords[..] else { panic!("{coords:?}"); };
        beacons.insert((bx, by));
        let radius = sx.abs_diff(bx) + sy.abs_diff(by);
        sensor_areas.push((sx, sy, radius));
        let height = target_row.abs_diff(sy);
        if height > radius {
            continue;
        }
        let range = (radius - height) as i64;
        let (mut x_min, mut x_max) = (sx - range, sx + range);
        if by == target_row {
            if bx == x_min {
                x_min += 1;
            }
            if bx == x_max {
                x_max -= 1;
            }
        }
        for x in x_min..=x_max {
            row_coords.insert(x);
        }
        println!("{sx},{sy} {bx},{by} {range:?}");
    }

    println!("{}", row_coords.len());

    for x in 0..=bound {
        if x % 1000 == 0 {
            println!("{x}");
        }
        for y in 0..=bound {
            if beacons.contains(&(x, y)) {
                continue;
            }
            let mut possible = true;
            for (sx, sy, radius) in &sensor_areas {
                if x.abs_diff(*sx) + y.abs_diff(*sy) <= *radius {
                    possible = false;
                    break;
                }
            }
            if possible {
                println!("{x} {y} {}", x * 4000000 + y);
                return;
            }
        }
    }
}
