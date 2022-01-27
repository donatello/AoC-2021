use std::collections::HashSet;
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    lines
        .iter()
        .map(|line| {
            let pts = line.split_whitespace();
            let pattern = pts
                .clone()
                .take_while(|x| *x != "|")
                .map(|x| x.to_string())
                .collect();
            let outputs = pts
                .clone()
                .skip_while(|x| *x != "|")
                .skip(1)
                .map(|x| x.to_string())
                .collect();
            (pattern, outputs)
        })
        .collect()
}

fn solve1(notes: &Vec<(Vec<String>, Vec<String>)>) -> i64 {
    let n: usize = notes
        .iter()
        .map(|(_, o)| {
            o.iter()
                .filter(|t| {
                    let s = *t;
                    s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7
                })
                .count()
        })
        .sum();
    n as i64
}

fn solve2(notes: &Vec<(Vec<String>, Vec<String>)>) -> i64 {
    // S(1), S(4), S(7), S(8) are known.

    // a = S(7) - S(1) => Sig(a) is known.
    // eg = S(2) - S(4) - S(7)
    // g = S(3) - S(4) - S(7);
    // g = S(5) - S(4) - S(7); => Sig(g), Sig(e) are known. S(2) is known
    // c = S(8) - S(6)
    // e = S(8) - S(9)
    // d = S(8) - S(0)
    // b = S(8) - S(2) - S(1) => Sig(b) is known.
    // Among S(2), S(3), S(5), only S(2) contains e => S(2) is known.
    //                       , only S(5) contains b => S(5) is known.
    //                                              => S(3) is known.
    // Among S(0), S(6), S(9), only S(9) does not contain e => S(9) is known.
    // S(7) - S(6) is non-empty and S(7) - S(0) is empty => S(0) and S(7) are known.
    // S(7) - S(9) is empty.
    notes
        .iter()
        .map(|(d, output)| {
            let filter_len = |n: usize| -> HashSet<char> {
                d.iter()
                    .filter(|x| x.len() == n)
                    .nth(0)
                    .unwrap()
                    .chars()
                    .collect::<HashSet<char>>()
            };
            let mut s: [HashSet<char>; 10] = Default::default();
            s[1] = filter_len(2);
            s[4] = filter_len(4);
            s[7] = filter_len(3);
            s[8] = filter_len(7);

            let mut five_segs = d
                .iter()
                .filter(|x| x.len() == 5)
                .map(|s| s.chars().collect())
                .collect::<Vec<HashSet<char>>>();
            let five_eqns: Vec<HashSet<char>> = five_segs
                .iter()
                .map(|fseg| {
                    let t: HashSet<char> = fseg.difference(&s[4]).map(|c| c.clone()).collect();
                    t.difference(&s[7]).map(|c| c.clone()).collect()
                })
                .collect();
            s[2] = five_segs
                .iter()
                .zip(five_eqns.iter())
                .filter_map(|(sval, eqval)| if eqval.len() == 2 { Some(sval) } else { None })
                .nth(0)
                .unwrap()
                .clone();
            five_segs = five_segs
                .iter()
                .filter(|x| *x != &s[2])
                .map(|x| x.clone())
                .collect();
            let sig_b: char = *s[8]
                .difference(&s[2])
                .map(|x| x.clone())
                .collect::<HashSet<char>>()
                .difference(&s[1])
                .map(|x| x.clone())
                .collect::<HashSet<char>>()
                .iter()
                .nth(0)
                .unwrap();
            s[5] = five_segs
                .iter()
                .filter(|x| x.contains(&sig_b))
                .nth(0)
                .unwrap()
                .clone();
            s[3] = five_segs
                .iter()
                .filter(|x| !x.contains(&sig_b))
                .nth(0)
                .unwrap()
                .clone();

            let sig_g: char = *five_eqns
                .iter()
                .filter(|v| v.len() == 1)
                .nth(0)
                .unwrap()
                .iter()
                .nth(0)
                .unwrap();
            let mut eg_val = five_eqns
                .iter()
                .filter(|v| v.len() == 2)
                .nth(0)
                .unwrap()
                .clone();
            eg_val.remove(&sig_g);
            let sig_e: char = *eg_val.iter().nth(0).unwrap();

            let mut six_segs = d
                .iter()
                .filter(|x| x.len() == 6)
                .map(|s| s.chars().collect())
                .collect::<Vec<HashSet<char>>>();
            s[9] = six_segs
                .iter()
                .filter(|x| !x.contains(&sig_e))
                .nth(0)
                .unwrap()
                .clone();
            six_segs = six_segs
                .iter()
                .filter(|x| *x != &s[9])
                .map(|x| x.clone())
                .collect();
            s[6] = six_segs
                .iter()
                .filter(|x| s[7].difference(x).count() > 0)
                .nth(0)
                .unwrap()
                .clone();
            s[0] = six_segs
                .iter()
                .filter(|x| s[7].difference(x).count() == 0)
                .nth(0)
                .unwrap()
                .clone();

            let v = output
                .iter()
                .enumerate()
                .map(|(n, x)| {
                    let set = x.chars().collect();
                    s.iter()
                        .enumerate()
                        .filter_map(|(i, k)| if k == &set { Some(i) } else { None })
                        .nth(0)
                        .unwrap() as i64
                        * (10i64.pow(3 - n as u32))
                })
                .sum::<i64>();
            // println!("{:?} {:?} => {:?} {}", d, output, s, v);
            v
        })
        .sum()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
