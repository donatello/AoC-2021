use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn print(h: &Vec<Vec<char>>) {
    h.iter()
        .map(|x| x.iter().collect::<String>())
        .for_each(|x| println!("{}", x));
}

fn solve1(h: &Vec<Vec<char>>) -> i64 {
    let mut p = h.clone();
    let mut c = h.clone();
    let mut steps = 0;
    let rows = p.len();
    let cols = p[0].len();
    loop {
        // Clear c.
        for i in 0..rows {
            for j in 0..cols {
                c[i][j] = '.';
            }
        }

        let mut moved_count = 0;

        // Check east facing
        for i in 0..rows {
            for j in 0..cols {
                // c[i][j] = '.';
                match p[i][j] {
                    '>' => {
                        if p[i][(j + 1) % cols] == '.' {
                            c[i][(j + 1) % cols] = '>';
                            moved_count += 1;
                        } else {
                            c[i][j] = '>';
                        }
                    }
                    'v' => {
                        if (p[(i + 1) % rows][j] == '.'
                            && p[(i + 1) % rows][(j + cols - 1) % cols] != '>')
                            || (p[(i + 1) % rows][j] == '>'
                                && p[(i + 1) % rows][(j + 1) % cols] == '.')
                        {
                            c[(i + 1) % rows][j] = 'v';
                            moved_count += 1;
                        } else {
                            c[i][j] = 'v';
                        }
                    }
                    '.' => {}
                    _ => panic!("should not happen: {}", p[i][j]),
                }
            }
        }

        if moved_count == 0 {
            break steps + 1;
        }
        steps += 1;
        let tmp = p;
        p = c;
        c = tmp;

        // println!("Step {}:", steps);
        // print(&p);
    }
}

fn solve2(h: &Vec<Vec<char>>) -> i64 {
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
