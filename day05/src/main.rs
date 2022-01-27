use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Pt {
    x: i64,
    y: i64,
}
#[derive(Debug, Clone)]
struct Line(Pt, Pt);

impl Line {
    fn normalize(&mut self) {
        if self.0.x > self.1.x || (self.0.x == self.1.x && self.0.y > self.1.y) {
            let t = self.0.clone();
            self.0 = self.1.clone();
            self.1 = t;
        }
    }
    fn ish(&self) -> bool {
        self.0.y == self.1.y
    }
    fn isv(&self) -> bool {
        self.0.x == self.1.x
    }
    fn isd1(&self) -> bool {
        if self.ish() || self.isv() {
            return false;
        }
        self.0.y < self.1.y
    }
    fn isd2(&self) -> bool {
        if self.ish() || self.isv() {
            return false;
        }
        self.0.y > self.1.y
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Line> {
    lines
        .iter()
        .map(|s| {
            let mut line = [0; 4];
            s.split(|x| !char::is_numeric(x))
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .enumerate()
                .for_each(|(i, v)| line[i] = v);
            let mut line = Line(
                Pt {
                    x: line[0],
                    y: line[1],
                },
                Pt {
                    x: line[2],
                    y: line[3],
                },
            );
            line.normalize();
            line
        })
        .collect::<Vec<_>>()
}

fn solve1(input_lines: &Vec<Line>) -> i64 {
    let lines = input_lines
        .iter()
        .filter(|x| x.ish() || x.isv())
        .map(|x| (*x).clone())
        .collect::<Vec<Line>>();
    let mut c = [[0; 1000]; 1000];
    lines.iter().for_each(|line| {
        if line.ish() {
            let y = line.0.y;
            (line.0.x..line.1.x + 1)
                .into_iter()
                .for_each(|x| c[x as usize][y as usize] += 1);
        }
        if line.isv() {
            let x = line.0.x;
            (line.0.y..line.1.y + 1)
                .into_iter()
                .for_each(|y| c[x as usize][y as usize] += 1);
        }
    });
    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if c[i][j] > 1 {
                count += 1
            }
        }
    }
    count
}

fn solve2(lines: &Vec<Line>) -> i64 {
    let mut c = [[0; 1000]; 1000];
    lines.iter().for_each(|line| {
        if line.ish() {
            let y = line.0.y;
            (line.0.x..line.1.x + 1)
                .into_iter()
                .for_each(|x| c[x as usize][y as usize] += 1);
        }
        if line.isv() {
            let x = line.0.x;
            (line.0.y..line.1.y + 1)
                .into_iter()
                .for_each(|y| c[x as usize][y as usize] += 1);
        }
        if line.isd1() {
            (line.0.x..line.1.x + 1)
                .into_iter()
                .enumerate()
                .for_each(|(i, x)| c[x as usize][line.0.y as usize + i] += 1);
        }
        if line.isd2() {
            (line.0.x..line.1.x + 1)
                .into_iter()
                .enumerate()
                .for_each(|(i, x)| c[x as usize][line.0.y as usize - i] += 1);
        }
    });
    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if c[i][j] > 1 {
                count += 1
            }
        }
    }
    count
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
