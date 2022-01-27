use std::io::{self, BufRead};

fn solve1(lines: &Vec<String>) -> i64 {
    let (x, y) = lines.iter().fold((0, 0), |(x, y), e| {
        let parts = e.split(' ').map(|x| x.to_string()).collect::<Vec<String>>();
        let v: i64 = parts[1].parse().unwrap();
        match parts[0].as_str() {
            "forward" => (x + v, y),
            "up" => (x, y - v),
            "down" => (x, y + v),
            _ => (x, y),
        }
    });
    x * y
}

fn solve2(lines: &Vec<String>) -> i64 {
    let (x, y, _) = lines.iter().fold((0, 0, 0), |(x, y, aim), e| {
        let parts = e.split(' ').map(|x| x.to_string()).collect::<Vec<String>>();
        let v: i64 = parts[1].parse().unwrap();
        match parts[0].as_str() {
            "forward" => (x + v, y + v * aim, aim),
            "up" => (x, y, aim - v),
            "down" => (x, y, aim + v),
            _ => (x, y, aim),
        }
    });
    x * y
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
