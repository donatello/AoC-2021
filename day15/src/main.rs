use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

#[derive(PartialEq, Eq, Debug)]
struct Node((i64, i64), i64);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve1(h: &Vec<Vec<char>>) -> i64 {
    // Dijkstra's single source shortest paths
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Node((0, 0), 0)));

    let dx = &[-1, 0, 1, 0];
    let dy = &[0, -1, 0, 1];

    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    distances.insert((0, 0), 0);

    while let Some(Reverse(Node((i, j), curr_dist))) = to_visit.pop() {
        if !visited.insert((i, j)) {
            // Node is already visited. Skip it.
            continue;
        }

        // println!(
        //     "C: {:?}\nvisited: {:?}\nto_visit: {:?}\ndistances: {:?}\n",
        //     Node((i, j), curr_dist),
        //     visited,
        //     to_visit,
        //     distances
        // );
        for k in 0..4 {
            if 0 <= i + dx[k]
                && i + dx[k] < h.len() as i64
                && 0 <= j + dy[k]
                && j + dy[k] < h[0].len() as i64
            {
                let x = (i + dx[k]) as usize;
                let y = (j + dy[k]) as usize;
                let new_distance = curr_dist + h[x][y].to_digit(10).unwrap() as i64;
                if let Some(v) = distances.get_mut(&(x, y)) {
                    if new_distance < *v {
                        *v = new_distance;
                    }
                } else {
                    distances.insert((x, y), new_distance);
                    to_visit.push(Reverse(Node((x as i64, y as i64), new_distance)));
                }
            }
        }
    }
    *distances.get(&(h.len() - 1, h[0].len() - 1)).unwrap()
}

fn solve2(h: &Vec<Vec<char>>) -> i64 {
    let next_char = |c: char| -> char {
        let mut d = c.to_digit(10).unwrap();
        d += 1;
        if d == 10 {
            d = 1;
        }
        char::from_digit(d, 10).unwrap()
    };
    let mut h2 = vec![vec!['0'; h[0].len() * 5]; h.len() * 5];
    let a = h.len();
    let b = h[0].len();
    for i in 0..h.len() {
        for j in 0..h[0].len() {
            let mut c = h[i][j];
            for k in 0..5 {
                if k > 0 {
                    c = next_char(h2[i + (k - 1) * a][j])
                }
                for l in 0..5 {
                    h2[i + k * a][j + l * b] = c;
                    c = next_char(c);
                }
            }
        }
    }

    solve1(&h2)
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
