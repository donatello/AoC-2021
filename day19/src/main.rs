use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};
use std::ops::{Add, Mul, Sub};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pt {
    m: [i64; 3],
}

impl Default for Pt {
    fn default() -> Pt {
        Pt { m: [0; 3] }
    }
}

impl Pt {
    fn distance(&self, o: &Pt) -> i64 {
        (0..3).into_iter().map(|i| (self.m[i] - o.m[i]).abs()).sum()
    }
}

impl Add for Pt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut r: Self = Default::default();
        (0..3)
            .into_iter()
            .for_each(|i| r.m[i] = self.m[i] + rhs.m[i]);
        r
    }
}

impl Sub for Pt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut r: Self = Default::default();
        (0..3)
            .into_iter()
            .for_each(|i| r.m[i] = self.m[i] - rhs.m[i]);
        r
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct M {
    m: [[i64; 3]; 3],
}

const I: M = M {
    m: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
};
const ROT_Z: M = M {
    m: [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
};
const ROT_X: M = M {
    m: [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
};
const ROT_Y: M = M {
    m: [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
};

fn mk_orientations() -> [M; 24] {
    [
        // Facing +x,
        I,
        I * ROT_X,
        I * ROT_X * ROT_X,
        I * ROT_X * ROT_X * ROT_X,
        // Facing +y,
        I * ROT_Z,
        I * ROT_Z * ROT_Y,
        I * ROT_Z * ROT_Y * ROT_Y,
        I * ROT_Z * ROT_Y * ROT_Y * ROT_Y,
        // Facing -x,
        I * ROT_Z * ROT_Z,
        I * ROT_Z * ROT_Z * ROT_X,
        I * ROT_Z * ROT_Z * ROT_X * ROT_X,
        I * ROT_Z * ROT_Z * ROT_X * ROT_X * ROT_X,
        // Facing -y,
        I * ROT_Z * ROT_Z * ROT_Z,
        I * ROT_Z * ROT_Z * ROT_Z * ROT_Y,
        I * ROT_Z * ROT_Z * ROT_Z * ROT_Y * ROT_Y,
        I * ROT_Z * ROT_Z * ROT_Z * ROT_Y * ROT_Y * ROT_Y,
        // Facing +z,
        I * ROT_Y * ROT_Y * ROT_Y,
        I * ROT_Y * ROT_Y * ROT_Y * ROT_Z,
        I * ROT_Y * ROT_Y * ROT_Y * ROT_Z * ROT_Z,
        I * ROT_Y * ROT_Y * ROT_Y * ROT_Z * ROT_Z * ROT_Z,
        // Facing -z,
        I * ROT_Y,
        I * ROT_Y * ROT_Z,
        I * ROT_Y * ROT_Z * ROT_Z,
        I * ROT_Y * ROT_Z * ROT_Z * ROT_Z,
    ]
}

impl Mul for M {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut v = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    v[i][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }
        M { m: v }
    }
}

impl Mul<Pt> for M {
    type Output = Pt;
    fn mul(self, p: Pt) -> Pt {
        let mut r = [0; 3];
        for i in 0..3 {
            for j in 0..3 {
                r[i] += self.m[i][j] * p.m[j];
            }
        }
        Pt { m: r }
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Pt>> {
    lines.iter().fold(vec![], |mut acc, line| {
        let n = acc.len();
        if line.strip_prefix("--- ").is_some() {
            acc.push(vec![]);
            if acc.len() > 1 {
                println!("num pts: {}", acc[acc.len() - 2].len());
            }
        } else if line.len() == 0 {
        } else {
            let mut p: Pt = Default::default();
            line.split(',')
                .enumerate()
                .for_each(|(i, x)| p.m[i] = x.parse::<i64>().unwrap());
            acc[n - 1].push(p);
        }
        acc
    })
}

fn disp(p: &HashSet<Pt>) -> String {
    let mut v = p.iter().cloned().collect::<Vec<Pt>>();
    v.sort_unstable();
    format!("{:?}", v)
}

// all_o[oi][j] is the set of points for some scanner oriented by o[oi] and with
// j-th point (in the input) as origin.
//
// ref_pts[i] is the set of points for some scanner (with known origin and
// oriented same as scanner-0) with the i-th point (in the input) as origin.
//
// Returns (i, j, oi, pts) such that ref_pts[i] and all_o[oi][j] have at least
// 12 common beacons - pt i and j were considered as a common origin and scanner
// j's points were oriented with o[oi]. `pts` is the set of common points found
// with the chosen common origin and with the orientation of ref_pts[i].
fn find_overlap(
    ref_pts: &Vec<HashSet<Pt>>,
    all_o: &Vec<Vec<HashSet<Pt>>>,
) -> Option<(usize, usize, usize, HashSet<Pt>)> {
    ref_pts.iter().enumerate().find_map(|(i, pts_a)| {
        all_o.iter().enumerate().find_map(|(oi, pt_origin_sets)| {
            pt_origin_sets.iter().enumerate().find_map(|(j, pts_b)| {
                let common_pts = pts_b.intersection(pts_a).copied().collect::<HashSet<Pt>>();
                if common_pts.len() >= 12 {
                    Some((i, j, oi, common_pts))
                } else {
                    None
                }
            })
        })
    })
}

fn solve1(h: &Vec<Vec<Pt>>) -> (Vec<Pt>, i64) {
    let ns = h.len();
    let o = mk_orientations();

    // all_orientations[i][j] is the i-th scanner's points oriented by o[j].
    let all_orientations = h
        .iter()
        .map(|spts| {
            o.iter()
                .map(|oi| spts.iter().map(|pt| *oi * *pt).collect())
                .collect()
        })
        .collect::<Vec<Vec<Vec<Pt>>>>();
    // all_pt_origin_sets[i][j][k] is the set of points from the scanner-i in
    // orientation o[j] with the k-th point (i.e. h[i][k]) as origin.
    let all_pt_origin_sets = all_orientations
        .iter()
        .map(|orientations| {
            orientations
                .iter()
                .map(|or| {
                    (0..or.len())
                        .into_iter()
                        .map(|k| or.iter().map(|x| *x - or[k]).collect::<HashSet<Pt>>())
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<Vec<Vec<HashSet<Pt>>>>>();

    // scanner-i's position and orientation wrt scanner-0 (if known).
    let mut scanner_pos: Vec<Option<(Pt, usize)>> = vec![None; ns];
    scanner_pos[0] = Some((Default::default(), 0));
    let mut q = VecDeque::new();
    q.push_back(0);
    while q.len() > 0 {
        let i = q.pop_front().unwrap();
        assert_eq!(scanner_pos[i].is_some(), true);
        let (scanner_pos_i, scanner_i_o_idx) = scanner_pos[i].unwrap();
        for j in 0..ns {
            if scanner_pos[j].is_some() {
                continue;
            }
            let now = Instant::now();
            let ropt = find_overlap(
                &all_pt_origin_sets[i][scanner_i_o_idx],
                &all_pt_origin_sets[j],
            );
            let elapsed = now.elapsed();
            println!(
                "Res: i: {}, j: {}, ropt: {:?} ({} ms)",
                i,
                j,
                ropt,
                elapsed.as_millis()
            );
            match ropt {
                Some((ii, jj, oi, common_pts)) => {
                    // let common_pts_i = common_pts
                    //     .iter()
                    //     .map(|p| *p + h[i][ii])
                    //     .collect::<HashSet<Pt>>();
                    // println!(
                    //     "CommonPts ({}): {}",
                    //     common_pts_i.len(),
                    //     disp(&common_pts_i)
                    // );
                    let pt_a = o[scanner_i_o_idx] * h[i][ii] + scanner_pos_i;
                    let pt_b = o[oi] * h[j][jj];

                    // scanner_pos_i is known.
                    // scanner_pos_j is unknown.
                    //
                    // pt_a is ii-th point of scanner-i wrt scanner-0 and oriented same as scanner-0.
                    // pt_b is jj-th point of scanner-j wrt scanner-j but oriented same as scanner-0.
                    //
                    // pt_a and pt_b are the same beacon.
                    //
                    // Therefore pt_a = scanner_pos_j + pt_b.
                    let scanner_pos_j = pt_a - pt_b;
                    scanner_pos[j] = Some((scanner_pos_j, oi));
                    // println!("scanner_pos_{}: {:?}", j, scanner_pos[j]);
                    q.push_back(j);
                }
                None => {}
            }
        }
    }

    let mut beacons = HashSet::new();
    scanner_pos
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.unwrap()))
        .for_each(|(i, (spos, oi))| {
            h[i].iter().map(|p| o[oi] * *p + spos).for_each(|p| {
                let _ = beacons.insert(p);
            });
        });
    (
        scanner_pos.iter().map(|x| x.unwrap().0).collect(),
        beacons.len() as i64,
    )
}

fn solve2(spos: Vec<Pt>) -> i64 {
    spos.iter()
        .map(|p| spos.iter().map(|q| p.distance(q)))
        .flatten()
        .max()
        .unwrap()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    let res = solve1(&lines);
    println!("{}", res.1);
    println!("{}", solve2(res.0));
    Ok(())
}
