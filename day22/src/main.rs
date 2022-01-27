use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Pt {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Cube {
    x: Rge,
    y: Rge,
    z: Rge,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Rge {
    start: i64,
    end: i64,
}

impl Rge {
    fn new(start: i64, end: i64) -> Self {
        Rge { start, end }
    }

    fn len(self: &Self) -> i64 {
        (self.end - self.start).abs()
    }

    fn is_empty(self: &Self) -> bool {
        self.start >= self.end
    }

    fn does_not_intersect(self: &Self, b: Self) -> bool {
        self.end <= b.start || b.end <= self.start
    }

    // Given two line segments a, b as ranges, if a contains b, returns the portion
    // of a to the left of b and the portion of a to the right of b.
    fn contains(self: &Self, b: Self) -> Option<(Self, Self)> {
        if self.start <= b.start && b.end <= self.end {
            Some((Self::new(self.start, b.start), Self::new(b.end, self.end)))
        } else {
            None
        }
    }

    // Given two line segments as ranges, where a does not contain b and vice versa,
    // returns ranges (p, q, r) where p = a-b, q = a intersection b, r = b - a
    fn intersection(self: &Self, b: Self) -> Option<(Self, Self, Self)> {
        let a = self;
        let mut ab = a.clone();
        let mut ba = b.clone();
        let mut aib = Self::new(0, 0);
        if a.start < b.start {
            if a.end > b.start {
                ab = Self::new(a.start, b.start);
                if a.end < b.end {
                    aib = Self::new(b.start, a.end);
                    ba = Self::new(a.end, b.end);
                } else {
                    return None;
                    // panic!("a={:?} contains b={:?} unexpectedly", a, b);
                }
            } else {
                // No intersection.
            }
        } else if a.start < b.end {
            if a.end < b.end {
                return None;
                // panic!("b={:?} contains a={:?} unexpectedly", b, a);
            } else {
                aib = Self::new(a.start, b.end);
                ab = Self::new(b.end, a.end);
                ba = Self::new(b.start, a.start);
            }
        } else {
            // No intersection.
        }
        Some((ab, aib, ba))
    }
}

impl Cube {
    fn volume(&self) -> i64 {
        self.x.len() * self.y.len() * self.z.len()
    }

    // a.sub(&b) returns a set of cubes containing all points in the set `a - b`
    // if a intersection b is non-empty. If it is empty, return None.
    fn sub(&self, o: &Cube) -> Option<Vec<Cube>> {
        /*
           sx1         sx2
        sy1+-----------+
           |           |
           |    ox1    |   ox2
           |  oy1+-----+----+
           |     |     |    |
           |     |     |    |
        sy2+-----+-----+    |
                 |          |
                 |          |
              oy2+----------+

         */

        if self.z.does_not_intersect(o.z)
            || self.y.does_not_intersect(o.y)
            || self.x.does_not_intersect(o.x)
        {
            return None;
        }
        let mut res = vec![];

        let mut rem_z = o.z;
        // Z-axis cases
        match (
            self.z.contains(o.z),
            o.z.contains(self.z),
            self.z.intersection(o.z),
        ) {
            (Some((zl, zr)), _, _) => {
                if !zl.is_empty() {
                    res.push(Cube {
                        x: self.x,
                        y: self.y,
                        z: zl,
                    });
                }
                if !zr.is_empty() {
                    res.push(Cube {
                        x: self.x,
                        y: self.y,
                        z: zr,
                    });
                }
            }
            (None, Some(_), _) => {
                // No cuboids to add.
                rem_z = self.z;
            }
            (None, None, Some((z_self, z_both, _))) => {
                if !z_self.is_empty() {
                    res.push(Cube {
                        x: self.x,
                        y: self.y,
                        z: z_self,
                    })
                }
                rem_z = z_both;
            }
            (None, None, None) => panic!("should not happen!"),
        }

        // Remaining z-axis range is fixed (rem_z).

        let mut rem_y = o.y;

        match (
            self.y.contains(o.y),
            o.y.contains(self.y),
            self.y.intersection(o.y),
        ) {
            (Some((yl, yr)), _, _) => {
                if !yl.is_empty() {
                    res.push(Cube {
                        x: self.x,
                        y: yl,
                        z: rem_z,
                    });
                }
                if !yr.is_empty() {
                    res.push(Cube {
                        x: self.x,
                        y: yr,
                        z: rem_z,
                    });
                }
            }
            (None, Some(_), _) => {
                // No cuboids to add.
                rem_y = self.y;
            }
            (None, None, Some((y_self, y_both, _))) => {
                if !y_self.is_empty() {
                    res.push(Cube {
                        x: self.x,
                        y: y_self,
                        z: rem_z,
                    });
                }
                rem_y = y_both;
            }
            (None, None, None) => panic!("should not happen!"),
        }

        // Remaining Y-axis range is fixed (rem_y).

        match (
            self.x.contains(o.x),
            o.x.contains(self.x),
            self.x.intersection(o.x),
        ) {
            (Some((xl, xr)), _, _) => {
                if !xl.is_empty() {
                    res.push(Cube {
                        x: xl,
                        y: rem_y,
                        z: rem_z,
                    });
                }
                if !xr.is_empty() {
                    res.push(Cube {
                        x: xr,
                        y: rem_y,
                        z: rem_z,
                    });
                }
            }
            (None, Some(_), _) => {
                // No cuboids to add.
            }
            (None, None, Some((x_self, _, _))) => {
                if !x_self.is_empty() {
                    res.push(Cube {
                        x: x_self,
                        y: rem_y,
                        z: rem_z,
                    })
                }
            }
            (None, None, None) => panic!("should not happen!"),
        }

        // println!(
        //     "SUB RES: Self={}, o={}, sub=\n{}\n",
        //     disp_cube(self),
        //     disp_cube(o),
        //     disp_v(&res)
        // );
        Some(res)
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<(bool, Pt, Pt)> {
    lines
        .iter()
        .filter(|x| !x.starts_with("# "))
        .map(|s| {
            let w = s.split(' ').collect::<Vec<&str>>();
            let nums = w[1]
                .split(&['x', 'y', 'z', '.', ',', '='][..])
                .filter_map(|t| t.parse::<i64>().ok())
                .collect::<Vec<i64>>();
            (
                w[0] == "on",
                Pt {
                    x: nums[0],
                    y: nums[2],
                    z: nums[4],
                },
                Pt {
                    x: nums[1],
                    y: nums[3],
                    z: nums[5],
                },
            )
        })
        .collect()
}

fn solve1(h: &Vec<(bool, Pt, Pt)>) -> i64 {
    // println!("here");
    let mut b = vec![vec![vec![false; 101]; 101]; 101];
    // let mut b = [[[false; 101]; 101]; 101];
    h.iter().for_each(|&(on, p1, p2)| {
        let (a1, b1, c1, a2, b2, c2) = (p1.x, p1.y, p1.z, p2.x, p2.y, p2.z);
        // println!("{}: {:?}, {:?}", on, Pt(a1, b1, c1), Pt(a2, b2, c2));
        // let v = [a1, a2, b1, b2, c1, c2];
        if a1 > a2 || b1 > b2 || c1 > c2 {
            panic!("unexpected!");
        }
        if a2 < -50 || a1 > 50 || b1 < -50 || b2 > 50 || c1 < -50 || c2 > 50 {
        } else {
            let (x1, x2, y1, y2, z1, z2) = (a1 + 50, a2 + 50, b1 + 50, b2 + 50, c1 + 50, c2 + 50);
            for x in x1..x2 + 1 {
                for y in y1..y2 + 1 {
                    for z in z1..z2 + 1 {
                        b[x as usize][y as usize][z as usize] = on;
                    }
                }
            }
        }
    });
    let mut count: i64 = 0;
    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                if b[i][j][k] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn disp_cube(c: &Cube) -> String {
    format!(
        "x={}..{},y={}..{},z={}..{}({})",
        c.x.start,
        c.x.end - 1,
        c.y.start,
        c.y.end - 1,
        c.z.start,
        c.z.end - 1,
        c.volume(),
    )
}

fn disp_hashset(h: &HashSet<Cube>) -> String {
    let s = h.iter().map(|v| disp_cube(v)).collect::<Vec<String>>();
    format!("{}", s.join("\n"))
}

fn disp_v(v: &Vec<Cube>) -> String {
    let s = v.iter().map(|x| disp_cube(x)).collect::<Vec<String>>();
    format!("{}", s.join("\n"))
}

fn solve2(h: &Vec<(bool, Pt, Pt)>) -> i64 {
    // on_cubes is a set of non-overlapping cubes in on-state.
    let mut on_cubes: HashSet<Cube> = HashSet::new();

    // For each instruction in h is an on-cube or an off-cube.
    //
    // If an on-cube overlaps with a cube in on_cubes, split the
    // non-overlapping part of on-cube into on-cuboids and check if these
    // overlap with the remaining on-cubes.
    //
    // At the end, any remaning on-cuboids are added to `on-cubes`.
    //
    // If an off-cube A overlaps with a cube B in on_cubes, split the cubes into
    // non-overlapping cuboids, such that there is exactly one overlapping
    // cuboid C and cubes forming the points in the set A-B and B-A. The cuboids
    // for A-B are on-cuboids and B-A are off-cuboids. Now, remove B from
    // `on_cubes`, and add the on-cuboids. Continue checking the off-cuboids
    // with subsequent instructions. At the end, and remaining off-cuboids are
    // simply discarded.
    h.iter().for_each(|(on, p1, p2)| {
        let rc = Cube {
            x: Rge::new(p1.x, p2.x + 1),
            y: Rge::new(p1.y, p2.y + 1),
            z: Rge::new(p1.z, p2.z + 1),
        };
        // println!("rc {}", disp_cube(&rc));
        if *on {
            // Form a set of cubes_to_check
            let mut cubes_to_check = HashSet::new();
            cubes_to_check.insert(rc);

            on_cubes.iter().for_each(|oc| {
                loop {
                    let mut add_to_check: Vec<Cube> = vec![];
                    let mut remove_from_check: Vec<Cube> = vec![];
                    let mut found_overlap = false;

                    for cc in cubes_to_check.iter() {
                        if let Some(sub_cubes) = cc.sub(&oc) {
                            found_overlap = true;
                            // Remove cc from cubes_to_check, and add sub_cubes to it.
                            remove_from_check.push(*cc);
                            sub_cubes.iter().for_each(|other| add_to_check.push(*other));
                            break;
                        }
                    }

                    if !found_overlap {
                        break;
                    } else {
                        remove_from_check.iter().for_each(|x| {
                            cubes_to_check.remove(x);
                        });
                        add_to_check.iter().for_each(|x| {
                            cubes_to_check.insert(*x);
                        });
                    }
                }
            });

            cubes_to_check.iter().for_each(|c| {
                on_cubes.insert(*c);
            });
            // println!("cubes added:\n{}", disp_hashset(&cubes_to_check));
        } else {
            // handle an off-cube.
            let mut add_cubes = vec![];
            let mut remove_cubes = vec![];

            on_cubes.iter().for_each(|oc| {
                if let Some(on_cube_parts) = oc.sub(&rc) {
                    // Remove oc from on_cubes, and add on_cube_parts to it.
                    remove_cubes.push(*oc);
                    add_cubes.extend(on_cube_parts.iter());
                }
            });

            remove_cubes.iter().for_each(|x| {
                on_cubes.remove(x);
            });
            add_cubes.iter().for_each(|x| {
                on_cubes.insert(*x);
            });
        }

        // println!("On Cubes: \n{}", disp_hashset(&on_cubes));
    });

    // println!("Final On Cubes: \n{}", disp_hashset(&on_cubes));
    on_cubes.iter().map(|x| x.volume()).sum()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
