use std::collections::VecDeque;
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn low_points(h: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    (0..h.len())
        .into_iter()
        .map(|i| (0..h[0].len()).into_iter().map(move |j| (i, j)))
        .flatten()
        .filter_map(|(i, j)| {
            if i > 0 && h[i - 1][j] <= h[i][j] {
                None
            } else if i + 1 < h.len() && h[i + 1][j] <= h[i][j] {
                None
            } else if j > 0 && h[i][j - 1] <= h[i][j] {
                None
            } else if j + 1 < h[0].len() && h[i][j + 1] <= h[i][j] {
                None
            } else {
                Some((i, j))
            }
        })
        .collect()
}

fn solve1(h: &Vec<Vec<u8>>) -> u64 {
    low_points(h)
        .iter()
        .map(|(i, j)| h[*i][*j] as u64 + 1)
        .sum()
}

fn solve2(h: &Vec<Vec<u8>>) -> u64 {
    let dxs = [-1i32, 0, 1];
    let dys = [-1i32, 0, 1];
    let mut basins = low_points(h)
        .iter()
        .map(|(i, j)| {
            let mut q = VecDeque::from([(*i as i32, *j as i32)]);
            let mut seen = vec![vec![false; h[0].len()]; h.len()];
            let mut basin_size = 0;
            while q.len() > 0 {
                let (x, y) = q.pop_front().unwrap();
                if seen[x as usize][y as usize] {
                    continue;
                }
                seen[x as usize][y as usize] = true;
                basin_size += 1;
                for dx in dxs {
                    for dy in dys {
                        if dx.abs() + dy.abs() == 1 {
                            let px = (x + dx) as i32;
                            let py = (y + dy) as i32;
                            if 0 <= px && px < h.len() as i32 && 0 <= py && py < h[0].len() as i32 {
                                if h[px as usize][py as usize] != 9
                                    && !seen[px as usize][py as usize]
                                {
                                    q.push_back((px as i32, py as i32));
                                }
                            }
                        }
                    }
                }
            }
            basin_size
        })
        .collect::<Vec<u64>>();
    basins.sort_unstable();
    let n = basins.len();
    basins[n - 1] * basins[n - 2] * basins[n - 3]
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
