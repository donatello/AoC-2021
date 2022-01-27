use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> (String, HashMap<String, char>) {
    (
        lines[0].clone(),
        lines
            .iter()
            .skip(2)
            .map(|x| {
                let p = x
                    .split(" -> ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                (p[0].clone(), p[1].chars().nth(0).unwrap())
            })
            .collect(),
    )
}

fn solve1((s, ops): &(String, HashMap<String, char>)) -> i64 {
    let mut prev: Vec<char> = s.chars().collect();
    let mut curr: Vec<char>;
    for _i in 0..10 {
        curr = vec![];
        prev.windows(2).for_each(|s| {
            let mut k = String::new();
            k.push(s[0]);
            k.push(s[1]);
            let c = ops.get(&k).unwrap();
            curr.extend(&[s[0], *c])
        });
        curr.push(prev[prev.len() - 1]);
        // if i < 5 {
        //     println!(
        //         "{}\n{}\n\n",
        //         prev.iter().collect::<String>(),
        //         curr.iter().collect::<String>()
        //     )
        // }
        prev = curr
    }
    let mut counts = HashMap::new();
    prev.iter().for_each(|c| {
        if let Some(v) = counts.get_mut(c) {
            *v += 1;
        } else {
            counts.insert(c, 1);
        }
    });
    let res = counts.into_iter().fold(
        (('-', 1_000_000_000), ('-', 0)),
        |((clo, lo), (chi, hi)), (c, count)| {
            let h = if count > hi { (*c, count) } else { (chi, hi) };
            let l = if count < lo { (*c, count) } else { (clo, lo) };
            (l, h)
        },
    );
    res.1 .1 - res.0 .1
}

type CMap = HashMap<char, i64>;

fn inc(m: &mut CMap, ch: char) {
    if let Some(v) = m.get_mut(&ch) {
        *v += 1;
    } else {
        m.insert(ch, 1);
    }
}

fn concat(c1: &CMap, c2: &CMap, mid: char) -> Option<CMap> {
    let mut rc = c1.clone();
    c2.into_iter().for_each(|(s, c)| {
        if let Some(v) = rc.get_mut(&s) {
            *v += c;
        } else {
            rc.insert(*s, *c);
        }
    });
    if let Some(v) = rc.get_mut(&mid) {
        *v -= 1;
        if *v < 0 {
            return None;
        }
        Some(rc)
    } else {
        None
    }
}

fn solve2((s, ops): &(String, HashMap<String, char>)) -> i64 {
    let o = ops
        .iter()
        .map(|(s, c)| {
            let ch = s.chars().collect::<Vec<char>>();
            ((ch[0], ch[1]), *c)
        })
        .collect::<HashMap<(char, char), char>>();

    let mut r: HashMap<(char, char, i64), CMap> = HashMap::new();
    o.iter().for_each(|((a, b), p)| {
        let mut m = CMap::new();
        inc(&mut m, *a);
        inc(&mut m, *b);
        inc(&mut m, *p);
        r.insert((*a, *b, 1), m);
    });
    for n in 2..41 {
        o.iter().for_each(|((a, b), p)| {
            let m = concat(
                r.get(&(*a, *p, n - 1)).unwrap(),
                r.get(&(*p, *b, n - 1)).unwrap(),
                *p,
            )
            .unwrap();
            r.insert((*a, *b, n), m);
        })
    }

    let n = 40;
    let input_str = s.clone();
    // let input_str = "CB";
    let template = input_str.chars().collect::<Vec<char>>();
    let mut h = r.get(&(template[0], template[1], n)).unwrap().clone();
    template.windows(2).skip(1).for_each(|ch| {
        let t = r.get(&(ch[0], ch[1], n)).unwrap();
        h = concat(&h, t, ch[0]).unwrap();
    });
    // r(a,b,n) = concat(r(a, s(ab), n-1), r(s(ab), b, n-1))
    // println!("s: {}\nh: {:?}", input_str, h);

    let res = h.into_iter().fold(
        (('-', 1_000_000_000_000_000_000), ('-', 0)),
        |((clo, lo), (chi, hi)), (c, count)| {
            let h = if count > hi { (c, count) } else { (chi, hi) };
            let l = if count < lo { (c, count) } else { (clo, lo) };
            (l, h)
        },
    );
    println!("res: {:?}", res);
    res.1 .1 - res.0 .1
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
