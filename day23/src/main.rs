use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> M {
    let mut m: M = [[' '; 13]; 7];
    lines.iter().enumerate().for_each(|(i, x)| {
        let cs = x.chars().collect::<Vec<char>>();
        m[i][..cs.len()].clone_from_slice(&cs[..]);
    });
    m
}

type M = [[char; 13]; 7];

fn d(m: &M) {
    for i in 0..m.len() {
        let v = m[i][..].iter().collect::<String>();
        println!("{}", v);
    }
}

fn cost(c: char) -> i64 {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("should not happen!"),
    }
}

fn is_pod(c: char) -> bool {
    match c {
        'A' => true,
        'B' => true,
        'C' => true,
        'D' => true,
        _ => false,
    }
}

fn dest(c: char) -> usize {
    match c {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!("oops"),
    }
}

fn valid_moves(m: M) -> Vec<(M, i64)> {
    let mut moves = vec![];

    // Room leaving moves:
    for i in 2..6 {
        if m[i][3] == '#' {
            break;
        }
        for j in (3..10).step_by(2) {
            if !is_pod(m[i][j]) {
                continue;
            }
            let mut count = 0;
            let mut x = i;
            while x > 1 && m[x - 1][j] == '.' {
                count += 1;
                x -= 1;
            }
            if x != 1 {
                continue;
            }
            let saved_count = count;
            // Go left!
            let mut y = j;
            while y > 1 && m[1][y - 1] == '.' {
                count += 1;
                y -= 1;
                if !(y >= 3 && y <= 9 && y % 2 == 1) {
                    let mut n = m.clone();
                    n[1][y] = m[i][j];
                    n[i][j] = '.';
                    moves.push((n, count * cost(m[i][j])));
                }
            }

            // Go right!
            y = j;
            count = saved_count;
            while y < 11 && m[1][y + 1] == '.' {
                count += 1;
                y += 1;
                if !(y >= 3 && y <= 9 && y % 2 == 1) {
                    let mut n = m.clone();
                    n[1][y] = m[i][j];
                    n[i][j] = '.';
                    moves.push((n, count * cost(m[i][j])));
                }
            }
        }
    }

    // Room entering moves.
    for j in 1..12 {
        let c = m[1][j];
        if !is_pod(c) {
            continue;
        }

        let mut y = j;
        let dst = dest(c);
        let mut count = 0;
        while y < dst && m[1][y + 1] == '.' {
            count += 1;
            y += 1;
        }
        while y > dst && m[1][y - 1] == '.' {
            count += 1;
            y -= 1;
        }
        if y != dst {
            continue;
        }

        // check if there any other pods in the room.
        let mut has_other = false;
        let mut pos = 0;
        for i in 2..6 {
            if m[i][y] == '#' {
                break;
            }
            if m[i][y] != '.' && m[i][y] != c {
                has_other = true;
                break;
            }
            if m[i][y] == '.' {
                count += 1;
                pos = i;
            } else {
                // position has same pod already
                break;
            }
        }

        if has_other {
            continue;
        }
        if pos != 0 {
            let mut n = m.clone();
            n[pos][y] = c;
            n[1][j] = '.';
            moves.push((n, count * cost(c)));
        }
    }

    moves
}

#[derive(Eq, PartialEq)]
struct HeapEntry {
    m: M,
    d: i64,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.d.cmp(&other.d)
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(start: &M, final_state: &M) -> i64 {
    let mut dist: HashMap<M, i64> = HashMap::new();
    let mut found: HashSet<M> = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut prev: HashMap<M, M> = HashMap::new();
    heap.push(Reverse(HeapEntry {
        m: start.clone(),
        d: 0,
    }));
    dist.insert(start.clone(), 0);
    while !heap.is_empty() {
        let Reverse(v) = heap.pop().unwrap();
        if !found.insert(v.m) {
            continue;
        }
        if v.m == *final_state {
            // res = v.d;
            break;
        }
        let moves = valid_moves(v.m);
        // d(&v.m);
        // if v.m == test_state {
        //     println!("Found moves for:");
        //     d(&v.m);
        //     println!("=>");
        //     moves.iter().for_each(|x| {
        //         println!("cost: {}", x.1);
        //         d(&x.0);
        //     });
        //     println!("====");
        // }
        for (mv, move_cost) in moves.iter() {
            let nbr_cost = v.d + *move_cost;
            let is_better = dist.get(mv).map_or(true, |&curr| nbr_cost < curr);
            if is_better {
                dist.insert(*mv, nbr_cost);
                prev.insert(*mv, v.m);
                heap.push(Reverse(HeapEntry {
                    m: *mv,
                    d: nbr_cost,
                }));
            }
        }
    }
    // let mut move_list = vec![test_state];
    // let mut costs = vec![];
    // let mut cost = *dist.get(&test_state).unwrap();
    // let mut node = test_state;
    // println!("From:");
    // d(&node);
    // while let Some(p) = prev.get(&node) {
    //     let p_cost = *dist.get(p).unwrap();
    //     let move_cost = cost - p_cost;
    //     cost = p_cost;
    //     costs.push(move_cost);
    //     move_list.push(*p);
    //     // println!("From:");
    //     // d(&node);
    //     println!("=> ({})", move_cost);
    //     d(p);
    //     node = *p;
    // }

    *dist.get(final_state).unwrap()
}

fn solve1(h: &M) -> i64 {
    let final_state: Vec<String> = vec![
        "#############",
        "#...........#",
        "###A#B#C#D###",
        "  #A#B#C#D#",
        "  #########",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    let final_state = parse_input(&final_state);
    solve(h, &final_state)
}

fn solve2(h: &M) -> i64 {
    let final_state: Vec<String> = vec![
        "#############",
        "#...........#",
        "###A#B#C#D###",
        "  #A#B#C#D#",
        "  #A#B#C#D#",
        "  #A#B#C#D#",
        "  #########",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    let final_state = parse_input(&final_state);

    let mut start = h.clone();
    for i in (5..7).rev() {
        for j in 0..h[i].len() {
            start[i][j] = start[i - 2][j];
        }
    }
    let new_rows: Vec<Vec<char>> = vec!["  #D#C#B#A#", "  #D#B#A#C#"]
        .iter()
        .map(|x| x.chars().collect())
        .collect();

    new_rows.iter().enumerate().for_each(|(i, r)| {
        start[i + 3][..r.len()].clone_from_slice(r);
    });

    solve(&start, &final_state)
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
