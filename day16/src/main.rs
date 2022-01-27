use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<char> {
    lines[0].chars().collect()
}

fn read_number(bits: &[char]) -> i64 {
    let mut v = 0;
    for c in bits.iter() {
        v *= 2;
        v += c.to_digit(2).unwrap() as i64;
    }
    v
}

fn traverse_versum(bits: &[char]) -> (usize, i64) {
    let ver = read_number(&bits[0..3]);
    let mut ver_sum = ver;

    let type_id = read_number(&bits[3..6]);

    // println!("ver={} type_id={} len={}", ver, type_id, bits.len());

    if type_id == 4 {
        // println!("literal!");
        // literal packet
        // let mut lit = Vec::new();
        let mut i = 6;
        while i < bits.len() {
            // bits[i + 1..i + 5].iter().for_each(|b| lit.push(b));
            i += 5;
            if bits[i - 5] == '0' {
                break;
            }
        }
        (i, ver_sum)
    } else {
        // operator packet
        let len_type_id = read_number(&bits[6..7]);
        if len_type_id == 0 {
            // Next 15 bits represents length of all sub-packets.
            let subpackets_len = read_number(&bits[7..22]);
            // println!("sublen: {}", subpackets_len);
            let mut rbits = &bits[22..22 + subpackets_len as usize];
            // println!(" - rbits = {}", rbits.iter().collect::<String>());
            while rbits.len() > 0 {
                let (next_i, s) = traverse_versum(rbits);
                ver_sum += s;
                // println!("next_i: {}", next_i);
                rbits = &rbits[next_i..];
            }
            (22 + subpackets_len as usize, ver_sum)
        } else {
            // Next 11 bits is number of sub-packets.
            let num_subpackets = read_number(&bits[7..18]);
            let mut rbits = &bits[18..];
            // println!("numpkts: {} - init len = {}", num_subpackets, rbits.len());
            let mut last_idx = 0;
            for _i in 0..num_subpackets {
                let (next_i, s) = traverse_versum(rbits);
                ver_sum += s;
                rbits = &rbits[next_i..];
                last_idx += next_i;
                // println!("next_i={}", next_i);
            }
            (18 + last_idx, ver_sum)
        }
    }
}

fn solve1(h: &Vec<char>) -> i64 {
    let bits = h
        .iter()
        .map(|c| {
            let d = c.to_digit(16).unwrap();
            let s = format!("{:04b}", d);
            // println!("d={}, s={}", d, s);
            s.chars().collect::<Vec<char>>()
        })
        .flatten()
        .collect::<Vec<char>>();
    // println!("{}", bits.iter().collect::<String>());
    let (_, s) = traverse_versum(&bits);
    s as i64
}

fn traverse(bits: &[char]) -> (usize, i64) {
    let type_id = read_number(&bits[3..6]);

    // println!("ver={} type_id={} len={}", ver, type_id, bits.len());

    if type_id == 4 {
        // literal packet
        let mut lit = Vec::new();
        let mut i = 6;
        while i < bits.len() {
            bits[i + 1..i + 5].iter().for_each(|b| lit.push(*b));
            i += 5;
            if bits[i - 5] == '0' {
                break;
            }
        }
        (i, read_number(lit.as_slice()) as i64)
    } else {
        let mut val: Vec<i64> = Vec::new();
        let next_id;
        // operator packet
        let len_type_id = read_number(&bits[6..7]);
        if len_type_id == 0 {
            // Next 15 bits represents length of all sub-packets.
            let subpackets_len = read_number(&bits[7..22]);
            // println!("sublen: {}", subpackets_len);
            let mut rbits = &bits[22..22 + subpackets_len as usize];
            // println!(" - rbits = {}", rbits.iter().collect::<String>());
            while rbits.len() > 0 {
                let (next_i, s) = traverse(rbits);
                val.push(s);
                // println!("next_i: {}", next_i);
                rbits = &rbits[next_i..];
            }
            next_id = 22 + subpackets_len as usize;
        } else {
            // Next 11 bits is number of sub-packets.
            let num_subpackets = read_number(&bits[7..18]);
            let mut rbits = &bits[18..];
            // println!("numpkts: {} - init len = {}", num_subpackets, rbits.len());
            let mut last_idx = 0;
            for _i in 0..num_subpackets {
                let (next_i, s) = traverse(rbits);
                val.push(s);
                rbits = &rbits[next_i..];
                last_idx += next_i;
                // println!("next_i={}", next_i);
            }
            next_id = 18 + last_idx;
        }
        // println!("t: {} vals: {:?}", type_id, val);
        let res = match type_id {
            0 => val.iter().sum(),
            1 => val.iter().product(),
            2 => *val.iter().min().unwrap(),
            3 => *val.iter().max().unwrap(),
            5 => {
                if val[0] > val[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if val[0] < val[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if val[0] == val[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!("oops"),
        };
        (next_id, res)
    }
}

fn solve2(h: &Vec<char>) -> i64 {
    let bits = h
        .iter()
        .map(|c| {
            let d = c.to_digit(16).unwrap();
            let s = format!("{:04b}", d);
            // println!("d={}, s={}", d, s);
            s.chars().collect::<Vec<char>>()
        })
        .flatten()
        .collect::<Vec<char>>();
    // println!("{}", bits.iter().collect::<String>());
    let (_, s) = traverse(&bits);
    s
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
