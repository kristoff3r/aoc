fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut grid = [['.'; 700]; 180];
    let mut highest_y = 0;

    for line in data.split('\n') {
        for path in line.split(" -> ").collect::<Vec<_>>().windows(2) {
            let path = path
                .join(",")
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let &[ax, ay, bx, by] = &path[..] else { return; };
            if ay.max(by) > highest_y {
                highest_y = ay.max(by);
            }
            draw_line(&mut grid, (ax, ay), (bx, by));
        }
    }

    let mut iterations = 0;
    let mut cur_pos: Option<(usize, usize)> = None;
    let mut grid1 = grid.clone();
    while simulate(&mut grid1, &mut cur_pos) {
        iterations += 1;
    }
    let count1 = grid1.iter().flatten().filter(|&&c| c == 'o').count();
    println!("{count1} {iterations}");

    let mut iterations = 0;
    let mut cur_pos: Option<(usize, usize)> = None;
    for x in 0..grid[0].len() {
        grid[highest_y + 2][x] = '#';
    }
    while simulate(&mut grid, &mut cur_pos) {
        iterations += 1;
    }
    let count2 = grid.iter().flatten().filter(|&&c| c == 'o').count();
    print_grid(&grid);
    println!("{count2} {iterations}");
}

fn simulate<const M: usize, const N: usize>(
    grid: &mut [[char; M]; N],
    cur_pos: &mut Option<(usize, usize)>,
) -> bool {
    // println!("{cur_pos:?}");
    if let Some((x, y)) = cur_pos.take() {
        let mut new_pos = None;
        let ty = y + 1;
        for tx in [x, x - 1, x + 1] {
            match grid.get(ty).and_then(|row| row.get(tx)) {
                Some('.' | '+') => {
                    grid[ty][tx] = '+';
                    new_pos = Some((tx, ty));
                    break;
                }
                Some(_) => (),
                None => return false,
            }
        }
        if new_pos.is_none() {
            grid[y][x] = 'o';
        }
        *cur_pos = new_pos;
    } else {
        if grid[0][500] != '.' {
            return false;
        }
        *cur_pos = Some((500, 0));
    }

    true
}

fn draw_line<const M: usize, const N: usize>(
    grid: &mut [[char; M]; N],
    (ax, ay): (usize, usize),
    (bx, by): (usize, usize),
) {
    if ax == bx {
        let y = ay.min(by);
        for i in 0..=ay.abs_diff(by) {
            grid[y + i][ax] = '#';
        }
    } else {
        let x = ax.min(bx);
        for i in 0..=ax.abs_diff(bx) {
            grid[ay][x + i] = '#';
        }
    }
}

fn print_grid<const M: usize, const N: usize>(grid: &[[char; M]; N]) {
    for row in grid {
        for &x in row {
            print!("{x}");
        }
        println!()
    }
}
