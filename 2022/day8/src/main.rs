fn check_dir<I: Iterator<Item = u8>>(dir: I) -> Vec<usize> {
    let mut highest_prev = None;
    let mut res = Vec::new();
    for (i, n) in dir.enumerate() {
        if let Some(prev) = highest_prev {
            if prev < n {
                highest_prev = Some(n);
                res.push(i)
            }
        } else {
            highest_prev = Some(n);
            res.push(i);
        }
    }

    res
}

fn grid_iter<'a>(grid: &'a Vec<Vec<u8>>, i: usize) -> impl DoubleEndedIterator<Item = u8> + 'a {
    (0..grid.len()).map(move |j| grid[j][i]).into_iter()
}

fn scenic_score(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u64 {
    let mut dir_scores = Vec::new();
    let n = grid[x][y];
    let col_len = grid.len();
    let row_len = grid[0].len();

    {
        let mut res = 0;
        for i in (0..x).rev() {
            res += 1;
            if grid[i][y] >= n {
                break;
            }
        }
        dir_scores.push(res);
    }
    {
        let mut res = 0;
        for i in (x + 1)..col_len {
            res += 1;
            if grid[i][y] >= n {
                break;
            }
        }
        dir_scores.push(res);
    }
    {
        let mut res = 0;
        for i in (0..y).rev() {
            res += 1;
            if grid[x][i] >= n {
                break;
            }
        }
        dir_scores.push(res);
    }
    {
        let mut res = 0;
        for i in (y + 1)..row_len {
            res += 1;
            if grid[x][i] >= n {
                break;
            }
        }
        dir_scores.push(res);
    }

    println!("{dir_scores:?}");
    dir_scores.into_iter().fold(1, |x, y| x * y)
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let grid = data
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let col_len = grid.len();
    let row_len = grid[0].len();
    let mut visible = Vec::from_iter(grid.iter().map(|row| vec![false; row.len()]));

    for (j, row) in grid.iter().enumerate() {
        for i in check_dir(row.iter().cloned()) {
            visible[j][i] = true;
        }
        for i in check_dir(row.iter().rev().cloned()) {
            visible[j][row_len - i - 1] = true;
        }
    }

    for i in 0..row_len {
        for j in check_dir(grid_iter(&grid, i)) {
            visible[j][i] = true;
        }
        for j in check_dir(grid_iter(&grid, i).rev()) {
            visible[col_len - j - 1][i] = true;
        }
    }

    let res: usize = visible
        .iter()
        .map(|row| row.iter().filter(|x| **x).count())
        .sum();

    println!("{res}");

    let score = scenic_score(&grid, 1, 2);
    println!("{score}");
    let mut res = 0;
    for x in 0..col_len {
        for y in 0..row_len {
            let score = scenic_score(&grid, x, y);
            if score > res {
                res = score;
            }
        }
    }
    println!("{res}");
}
