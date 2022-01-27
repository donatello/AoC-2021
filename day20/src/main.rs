use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> (Vec<char>, Vec<Vec<char>>) {
    (
        lines[0].chars().collect(),
        lines[2..]
            .into_iter()
            .map(|s| s.chars().collect())
            .collect(),
    )
}

fn solve1((algo, image_t): &(Vec<char>, Vec<Vec<char>>), iter_count: i64) -> i64 {
    let mut old_image = image_t.clone();
    let mut image = image_t.clone();
    let (n, m) = (image.len() as i64, image[0].len() as i64);
    let mut old_h: HashMap<(i64, i64), char> = HashMap::new();
    let mut h: HashMap<(i64, i64), char> = HashMap::new();
    let dxy = (-1..2)
        .into_iter()
        .map(|k| {
            (-1..2)
                .into_iter()
                .map(|j| (k, j))
                .collect::<Vec<(i64, i64)>>()
        })
        .flatten()
        .collect::<Vec<(i64, i64)>>();
    let is_outside = |x, y| x < 0 || x >= n || y < 0 || y >= m;
    for i in 0..iter_count {
        let (sr, er, sc, ec) = (-(i + 1), n + i + 1, -(i + 1), m + i + 1);
        // println!("{:?}", (sr, er, sc, ec));
        h.clear();
        for a in sr..er {
            for b in sc..ec {
                let mut idx = 0;
                for (da, db) in dxy.iter() {
                    let (x, y) = (a + da, b + db);
                    let c = if is_outside(x, y) {
                        let def_char = if i % 2 == 1 { '#' } else { '.' };
                        *old_h.get(&(x, y)).unwrap_or(&def_char)
                    } else {
                        old_image[x as usize][y as usize]
                    };
                    idx *= 2;
                    if c == '#' {
                        idx += 1;
                    }
                }
                let o_char = algo[idx];
                if is_outside(a, b) {
                    h.insert((a, b), o_char);
                } else {
                    image[a as usize][b as usize] = o_char;
                }
            }
        }
        old_image = image.clone();
        old_h = h.clone();
    }
    let mut res = image
        .iter()
        .map(|x| x.iter())
        .flatten()
        .filter(|x| **x == '#')
        .count();
    res += h.values().filter(|x| **x == '#').count();
    res as i64
}

fn solve2(h: &(Vec<char>, Vec<Vec<char>>)) -> i64 {
    solve1(h, 50)
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines, 2));
    println!("{}", solve2(&lines));
    Ok(())
}
