use std::io::{self, BufRead};

fn solve1(lines: &Vec<String>) -> i64 {
    let s: Vec<Vec<char>> = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let dlen = s[0].len();
    let c = s.iter().fold(vec![0; dlen], |mut ones_count, t| {
        t.iter().enumerate().for_each(|(i, ch)| {
            if *ch == '1' {
                ones_count[i] += 1;
            }
        });
        ones_count
    });

    let gamma_rate_str = c
        .iter()
        .map(|x| if *x > s.len() / 2 { '1' } else { '0' })
        .collect::<String>();
    let epsilon_rate_str = c
        .iter()
        .map(|x| if *x > s.len() / 2 { '0' } else { '1' })
        .collect::<String>();
    let gamma_rate = i64::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = i64::from_str_radix(&epsilon_rate_str, 2).unwrap();

    gamma_rate * epsilon_rate
}

fn filter(s: Vec<Vec<char>>, pos: usize, is_o2: bool) -> i64 {
    if s.len() == 1 {
        i64::from_str_radix(&s[0].iter().collect::<String>(), 2).unwrap()
    } else {
        let (with_ones, with_zeros) = s.into_iter().fold((vec![], vec![]), |(mut o, mut z), x| {
            if x[pos] == '1' {
                o.push(x);
            } else {
                z.push(x);
            }
            (o, z)
        });
        if is_o2 {
            if with_ones.len() >= with_zeros.len() {
                filter(with_ones, pos + 1, is_o2)
            } else {
                filter(with_zeros, pos + 1, is_o2)
            }
        } else {
            if with_ones.len() < with_zeros.len() {
                filter(with_ones, pos + 1, is_o2)
            } else {
                filter(with_zeros, pos + 1, is_o2)
            }
        }
    }
}

fn solve2(lines: &Vec<String>) -> i64 {
    let s: Vec<Vec<char>> = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let s2 = s.clone();
    let o2 = filter(s, 0, true);
    let co2 = filter(s2, 0, false);
    o2 * co2
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
    Ok(())
}
