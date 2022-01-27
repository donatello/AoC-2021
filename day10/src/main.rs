use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<String> {
    lines.clone()
}

fn is_corrupt(s: &str) -> Option<i64> {
    let score: HashMap<char, i64> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let bkts: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut v = vec![];
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if !score.contains_key(&c) {
            v.push(c);
        } else {
            if v[v.len() - 1] == *bkts.get(&c).unwrap() {
                v.pop();
            } else {
                return Some(*score.get(&c).unwrap());
            }
        }
    }
    None
}

fn solve1(h: &Vec<String>) -> i64 {
    h.iter().filter_map(|s| is_corrupt(s)).sum()
}

fn solve2(h: &Vec<String>) -> i64 {
    let bkts: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let bkts2: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let score: HashMap<char, i64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut scores = h
        .iter()
        .filter_map(|s| {
            let mut v = vec![];
            let mut it = s.chars();
            while let Some(c) = it.next() {
                if !score.contains_key(&c) {
                    v.push(c);
                } else {
                    if v[v.len() - 1] == *bkts.get(&c).unwrap() {
                        v.pop();
                    } else {
                        return None;
                    }
                }
            }
            if v.len() == 0 {
                None
            } else {
                Some(
                    v.iter()
                        .rev()
                        .map(|c| bkts2.get(&c).unwrap())
                        .fold(0, |acc, c| acc * 5 + score.get(c).unwrap()),
                )
            }
        })
        .collect::<Vec<i64>>();
    scores.sort_unstable();
    let n = scores.len();
    scores[n / 2]
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
