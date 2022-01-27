use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<i64> {
    lines
        .iter()
        .map(|x| x.split(' ').last().unwrap().parse::<i64>().unwrap())
        .collect()
}

fn solve1(h: &Vec<i64>) -> i64 {
    let mut pos = h.iter().map(|x| (x - 1) as u8).collect::<Vec<u8>>();
    let mut roll_count = 0;
    let mut score: [u64; 2] = [0; 2];
    let mut die_val = 0;
    let mut next_roll = || {
        die_val += 1;
        roll_count += 1;
        if die_val == 101 {
            die_val = 1;
        }
        die_val
    };
    while score[0] < 1000 && score[1] < 1000 {
        for i in 0..2 {
            let die_sum: u64 = (0..3).into_iter().map(|_| next_roll()).sum::<u64>() % 10;
            pos[i] = (pos[i] + (die_sum as u8)) % 10;
            score[i] += (pos[i] + 1) as u64;
            if score[i] >= 1000 {
                break;
            }
        }
    }
    (score[..].iter().find(|x| **x < 1000).unwrap() * roll_count) as i64
}

fn doit(pos: [u8; 2], s: [u64; 2], rolls: &mut Vec<usize>, die_sums: &[u64; 10]) -> (u64, u64) {
    let turn = rolls.len() % 2;
    let mut npos = pos.clone();
    let mut ns = s.clone();
    let mut wins = [0; 2];
    // println!("{:?} {:?} {:?}", pos, s, rolls);
    for roll in 3usize..10 {
        npos[turn] = (pos[turn] + (roll as u8)) % 10;
        ns[turn] = s[turn] + (npos[turn] as u64 + 1);
        if ns[turn] >= 21 {
            // println!("WIN: {:?} {:?} {:?} {}", npos, ns, rolls, roll);
            let ways = rolls.iter().map(|x| die_sums[*x]).product::<u64>() * die_sums[roll];
            wins[turn] += ways;
        } else {
            rolls.push(roll);
            let (w1, w2) = doit(npos, ns, rolls, die_sums);
            rolls.pop();
            wins[0] += w1;
            wins[1] += w2;
        }
    }
    (wins[0], wins[1])
}

fn solve2(h: &Vec<i64>) -> u64 {
    let pos = h.iter().map(|x| (x - 1) as u8).collect::<Vec<u8>>();
    let mut die_sums = [0; 10];
    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                die_sums[i + j + k] += 1;
            }
        }
    }

    let (w1, w2) = doit([pos[0], pos[1]], [0, 0], &mut vec![], &die_sums);
    if w1 > w2 {
        w1
    } else {
        w2
    }
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
