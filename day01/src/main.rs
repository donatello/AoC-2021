use std::io::{self, BufRead};

fn solve1(lines: &Vec<String>) -> i64 {
    let mut prev = 1000000000;
    let mut count = 0;
    for line in lines {
        let v: i64 = line.parse().unwrap();
        if v > prev {
            count += 1;
        }
        prev = v;
    }
    count
}

fn solve2(lines: &Vec<String>) -> i64 {
    let mut prev = 1000000000;
    let mut count = 0;
    let nums = lines
        .iter()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    for i in 0..(nums.len() - 2) {
        let s = nums[i] + nums[i + 1] + nums[i + 2];
        if s > prev {
            count += 1;
        }
        prev = s;
    }
    count
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
