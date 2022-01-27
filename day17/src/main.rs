use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> ((i64, i64), (i64, i64)) {
    let nums = lines[0]
        .split(&['=', '.', ','][..])
        .filter_map(|s| {
            if let Ok(n) = s.parse::<i64>() {
                Some(n)
            } else {
                None
            }
        })
        .collect::<Vec<i64>>();
    ((nums[0], nums[1]), (nums[2], nums[3]))
}

fn is_in_target_area(
    x: i64,
    y: i64,
    ((min_x, max_x), (min_y, max_y)): &((i64, i64), (i64, i64)),
) -> bool {
    *min_x <= x && x <= *max_x && *min_y <= y && y <= *max_y
}

fn is_good((mut vx, mut vy): &(i64, i64), target_area: &((i64, i64), (i64, i64))) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    loop {
        if y > max_y {
            max_y = y;
        }

        if is_in_target_area(x, y, target_area) {
            break Some(max_y);
        }

        if y < target_area.1 .0 {
            break None;
        }

        if x > target_area.0 .1 {
            break None;
        }

        x += vx;
        y += vy;

        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }

        vy -= 1;
    }
}

fn solve1(target_area: &((i64, i64), (i64, i64))) -> i64 {
    let mut best_v = 0;
    for vx in 1..2 * target_area.0 .1 {
        for vy in target_area.1 .0..100 {
            if let Some(v) = is_good(&(vx, vy), target_area) {
                if v > best_v {
                    best_v = v;
                }
            }
        }
    }
    best_v
}

fn solve2(target_area: &((i64, i64), (i64, i64))) -> i64 {
    let mut count = 0;
    for vx in 1..2 * target_area.0 .1 {
        for vy in target_area.1 .0..200 {
            if let Some(_) = is_good(&(vx, vy), target_area) {
                count += 1;
            }
        }
    }
    count
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
