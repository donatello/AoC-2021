use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Clone)]
struct Pt(i64, i64);

struct Fold(String, i64);

fn parse_input(lines: &Vec<String>) -> (Vec<Pt>, Vec<Fold>) {
    let mut i = lines.iter();
    let pts = i
        .by_ref()
        .take_while(|x| *x != "")
        .map(|x| {
            let ns = x
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Pt(ns[0], ns[1])
        })
        .collect::<Vec<Pt>>();
    let folds = i
        .map(|x| {
            let p = x
                .strip_prefix("fold along ")
                .unwrap()
                .split('=')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            Fold(p[0].clone(), p[1].parse::<i64>().unwrap())
        })
        .collect::<Vec<Fold>>();
    (pts, folds)
}

fn fold_x(pts: &Vec<Pt>, c: i64) -> Vec<Pt> {
    pts.iter()
        .map(|Pt(x, y)| {
            let d = *x - c;
            if *x > c {
                Pt(*x - 2 * d, *y)
            } else {
                Pt(*x, *y)
            }
        })
        .collect::<HashSet<Pt>>()
        .into_iter()
        .collect::<Vec<Pt>>()
}

fn fold_y(pts: &Vec<Pt>, c: i64) -> Vec<Pt> {
    pts.iter()
        .map(|Pt(x, y)| {
            let d = *y - c;
            if *y > c {
                Pt(*x, *y - 2 * d)
            } else {
                Pt(*x, *y)
            }
        })
        .collect::<HashSet<Pt>>()
        .into_iter()
        .collect::<Vec<Pt>>()
}

fn solve1(h: &(Vec<Pt>, Vec<Fold>)) -> i64 {
    let (pts, folds) = h;
    let fold = &folds[0];
    let newpts = match fold.0.as_str() {
        "x" => fold_x(pts, fold.1),
        "y" => fold_y(pts, fold.1),
        _ => panic!("!"),
    };
    newpts.len() as i64
}

fn solve2(h: &(Vec<Pt>, Vec<Fold>)) -> i64 {
    let (pts, folds) = h;
    let mut v = folds.iter().fold(pts.to_vec(), |acc, fold| {
        let p = match fold.0.as_str() {
            "x" => fold_x(&acc, fold.1),
            "y" => fold_y(&acc, fold.1),
            _ => panic!("!"),
        };
        p
    });
    let max_x = v.iter().map(|Pt(x, _)| x).max().unwrap();
    let max_y = v.iter().map(|Pt(_, x)| x).max().unwrap();
    let mut o = vec![vec!['.'; *max_x as usize + 1]; *max_y as usize + 1];
    v.iter()
        .for_each(|Pt(x, y)| o[*y as usize][*x as usize] = '#');
    o.iter()
        .for_each(|s| println!("{}", s.iter().collect::<String>()));

    v.sort_unstable();
    // v.iter().for_each(|Pt(x, y)| {});
    0
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
