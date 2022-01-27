use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn next_step(h: &mut Vec<Vec<u8>>) -> i64 {
    // inc energy level by 1
    let mut d = VecDeque::new();
    let mut s = HashSet::new();
    for i in 0..h.len() {
        for j in 0..h[0].len() {
            h[i][j] += 1;
            if h[i][j] > 9 {
                d.push_back((i as i32, j as i32));
                h[i][j] = 0;
                s.insert((i, j));
            }
        }
    }

    // count flashes and increase all neighbors; add neighbors to queue.
    let dxs = [-1i32, 0, 1];
    let dys = [-1i32, 0, 1];
    while !d.is_empty() {
        let (x, y) = d.pop_front().unwrap();
        for dx in dxs.iter() {
            for dy in dys.iter() {
                if 0 <= x + dx
                    && x + dx < h.len() as i32
                    && 0 <= y + dy
                    && y + dy < h[0].len() as i32
                {
                    let cx = (x + dx) as usize;
                    let cy = (y + dy) as usize;
                    if !s.contains(&(cx, cy)) {
                        h[cx][cy] += 1;
                        if h[cx][cy] > 9 {
                            h[cx][cy] = 0;
                            s.insert((cx, cy));
                            d.push_back((cx as i32, cy as i32));
                        }
                    }
                }
            }
        }
    }

    s.len() as i64
}

fn solve1(h: &Vec<Vec<u8>>) -> i64 {
    let mut g = h.clone();
    let mut res = 0;
    for _i in 0..100 {
        res += next_step(&mut g);
    }
    res
}

fn solve2(g: &Vec<Vec<u8>>) -> i64 {
    let mut h = g.clone();
    for step in 0..1000 {
        next_step(&mut h);

        // println!("\nAfter step: {}", step + 1);
        // for i in 0..h.len() {
        //     for j in 0..h[0].len() {
        //         print!("{}", h[i][j]);
        //     }
        //     println!("");
        // }

        let mut all = true;
        'outer: for i in 0..h.len() {
            for j in 0..h[0].len() {
                if h[i][j] != 0 {
                    all = false;
                    break 'outer;
                }
            }
        }
        if all {
            return step + 1;
        }
    }
    -1
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
