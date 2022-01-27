use std::io::{self, BufRead};

fn parse_card_line(s: &str) -> [i64; 5] {
    let mut r = [0; 5];
    s.split_whitespace()
        .map(|x| x.parse().unwrap())
        .enumerate()
        .for_each(|(i, x)| {
            r[i] = x;
        });
    r
}

fn parse_input(lines: &Vec<String>) -> (Vec<i64>, Vec<[[i64; 5]; 5]>) {
    let nums = lines[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let cards: Vec<[[i64; 5]; 5]> = lines[1..]
        .iter()
        .filter(|x| *x != "")
        .map(|x| parse_card_line(x))
        .enumerate()
        .fold(vec![], |mut acc, (i, x)| {
            if i % 5 == 0 {
                acc.push([[0; 5]; 5]);
            }
            let n = acc.len();
            for idx in 0..5 {
                acc[n - 1][i % 5][idx] = x[idx];
            }
            acc
        });

    (nums, cards)
}

fn got_bingo(card: &[[i64; 5]; 5], marks: &[[bool; 5]; 5]) -> Option<i64> {
    let mut bingo = false;
    for r in 0..5 {
        let mut won = true;
        for c in 0..5 {
            if marks[r][c] == false {
                won = false;
                break;
            }
        }
        if won {
            bingo = true;
            break;
        }

        won = true;
        for c in 0..5 {
            if marks[c][r] == false {
                won = false;
                break;
            }
        }
        if won {
            bingo = true;
            break;
        }
    }

    if bingo {
        let mut sum = 0;
        for a in 0..5 {
            for b in 0..5 {
                if marks[a][b] == false {
                    sum += card[a][b];
                }
            }
        }
        Some(sum)
    } else {
        None
    }
}

fn solve1(nums: &Vec<i64>, cards: &Vec<[[i64; 5]; 5]>) -> i64 {
    let n = cards.len();
    let mut marks: Vec<[[bool; 5]; 5]> = Vec::with_capacity(n);
    for _i in 0..n {
        marks.push([[false; 5]; 5]);
    }

    nums.iter()
        .filter_map(|x| {
            cards
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    for a in 0..5 {
                        for b in 0..5 {
                            if c[a][b] == *x {
                                marks[i][a][b] = true;
                            }
                        }
                    }

                    got_bingo(c, &marks[i]).map(|s| s * *x)
                })
                .nth(0)
        })
        .nth(0)
        .unwrap()
}

fn solve2(nums: &Vec<i64>, cards: &Vec<[[i64; 5]; 5]>) -> i64 {
    let n = cards.len();
    let mut marks: Vec<[[bool; 5]; 5]> = Vec::with_capacity(n);
    for _i in 0..n {
        marks.push([[false; 5]; 5]);
    }

    let mut mycards = cards
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c.clone()))
        .collect::<Vec<(_, _)>>();
    nums.iter()
        .filter_map(|x| {
            let before_count = mycards.len();
            let losing_cards = mycards
                .iter()
                .map(|(i, c)| (*i, c.clone()))
                .filter(|(i, c)| {
                    for a in 0..5 {
                        for b in 0..5 {
                            if c[a][b] == *x {
                                marks[*i][a][b] = true;
                            }
                        }
                    }

                    got_bingo(c, &marks[*i]).is_none()
                })
                .collect::<Vec<(_, _)>>();

            if losing_cards.len() == 0 {
                if before_count > 1 {
                    println!("multiple losing cards!");
                }
                let c = mycards[0].1;
                let i = mycards[0].0;
                got_bingo(&c, &marks[i]).map(|s| s * *x)
            } else {
                mycards = losing_cards;
                None
            }
        })
        .nth(0)
        .unwrap()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let (nums, cards) = parse_input(&v);
    println!("{}", solve1(&nums, &cards));
    println!("{}", solve2(&nums, &cards));
    Ok(())
}
