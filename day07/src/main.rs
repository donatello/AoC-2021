use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<i64> {
    lines[0].split(',').map(|x| x.parse().unwrap()).collect()
}

fn solve1(pos: &Vec<i64>) -> i64 {
    let mut nums = pos.clone();
    nums.sort_unstable();

    // compute cost to get all crabs to nums[0]
    let mut cost: i64 = nums.iter().map(|x| (x - nums[0]).abs()).sum();
    let mut left = 0;
    let mut right = nums.len() as i64 - 1;
    let mut min_cost = cost;
    let mut p = 1;
    while p < nums.len() {
        let dist_moved = nums[p] - nums[p - 1];
        cost += (left + 1) * dist_moved - right * dist_moved;

        if cost < min_cost {
            min_cost = cost;
        }

        p += 1;
        left += 1;
        right -= 1;
    }

    min_cost
}

fn solve2(pos: &Vec<i64>) -> i64 {
    let mut nums = pos.clone();
    nums.sort_unstable();

    let mut p = nums[0];
    let mut min_cost = 1_000_000_000_000;
    while p <= nums[nums.len() - 1] {
        let cost: i64 = nums
            .iter()
            .map(|x| {
                let n = (x - p).abs();
                n * (n + 1) / 2
            })
            .sum();

        if cost < min_cost {
            min_cost = cost;
        }

        p += 1;
    }

    min_cost
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
