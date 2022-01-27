use std::io::{self, BufRead};

#[derive(Debug, Default, Clone)]
struct Tree {
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
struct Node {
    left: Val,
    right: Val,
    parent: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Val {
    Sub(usize),
    Num(i64),
}

impl Tree {
    fn add(self: &mut Self, left: Val, right: Val, parent: Option<usize>) -> usize {
        let v = self.nodes.len();
        self.nodes.push(Node {
            left,
            right,
            parent,
        });
        v
    }

    fn format(self: &Self, v: &mut Vec<char>, ix: usize) {
        v.push('[');
        match self.nodes[ix].left {
            Val::Num(n) => {
                let s: String = format!("{}", n);
                v.extend(s.chars());
            }
            Val::Sub(t) => self.format(v, t),
        };
        v.push(',');
        match self.nodes[ix].right {
            Val::Num(n) => {
                let s: String = format!("{}", n);
                v.extend(s.chars());
            }
            Val::Sub(t) => self.format(v, t),
        };
        v.push(']');
    }

    fn string(self: &Self) -> String {
        let mut v = Vec::new();
        self.format(&mut v, 0);
        v.iter().collect()
    }

    // left value returned is index of new node formed. right value is index in
    // `s` upto which parsing happened.
    fn tree_from(self: &mut Self, s: &[char], k: usize) -> (usize, usize) {
        let node_idx = self.add(Val::Num(0), Val::Num(0), None);
        // println!("s: {}, k: {}", s.iter().collect::<String>(), k);
        assert_eq!(s[k], '[');
        let (left_val, next_idx) = if s[k + 1] == '[' {
            let (tx, ix) = self.tree_from(s, k + 1);
            (Val::Sub(tx), ix)
        } else {
            let d = s[k + 1].to_digit(10).unwrap() as i64;
            (Val::Num(d), k + 2)
        };
        assert_eq!(s[next_idx], ',');
        // println!("left: {:?}, next_idx: {}", left_val, next_idx);

        let (right_val, last_idx) = if s[next_idx + 1] == '[' {
            let (tx, ix) = self.tree_from(s, next_idx + 1);
            (Val::Sub(tx), ix)
        } else {
            let d = s[next_idx + 1].to_digit(10).unwrap() as i64;
            (Val::Num(d), next_idx + 2)
        };
        assert_eq!(s[last_idx], ']');

        if let Val::Sub(c) = left_val {
            self.nodes[c].parent = Some(node_idx);
        }

        if let Val::Sub(c) = right_val {
            self.nodes[c].parent = Some(node_idx);
        }

        self.nodes[node_idx].left = left_val;
        self.nodes[node_idx].right = right_val;
        (node_idx, last_idx + 1)
    }

    fn magnitude(self: &Self, i: usize) -> i64 {
        let l = match self.nodes[i].left {
            Val::Num(n) => n,
            Val::Sub(k) => self.magnitude(k),
        };
        let r = match self.nodes[i].right {
            Val::Num(n) => n,
            Val::Sub(k) => self.magnitude(k),
        };
        3 * l + 2 * r
    }

    fn find_depth4_node(self: &Self, depth: usize, ix: usize) -> Option<(usize, i64, i64)> {
        if depth < 4 {
            let l_opt = if let Val::Sub(left_ix) = self.nodes[ix].left {
                self.find_depth4_node(depth + 1, left_ix)
            } else {
                None
            };
            if l_opt.is_none() {
                if let Val::Sub(right_ix) = self.nodes[ix].right {
                    self.find_depth4_node(depth + 1, right_ix)
                } else {
                    None
                }
            } else {
                l_opt
            }
        } else {
            match (self.nodes[ix].left, self.nodes[ix].right) {
                (Val::Num(l), Val::Num(r)) => Some((ix, l, r)),
                _ => {
                    let l_opt = if let Val::Sub(left_ix) = self.nodes[ix].left {
                        self.find_depth4_node(depth + 1, left_ix)
                    } else {
                        None
                    };
                    if l_opt.is_none() {
                        if let Val::Sub(right_ix) = self.nodes[ix].right {
                            self.find_depth4_node(depth + 1, right_ix)
                        } else {
                            None
                        }
                    } else {
                        l_opt
                    }
                }
            }
        }
    }

    fn explode(self: &mut Self, ix: usize, l: i64, r: i64) {
        // println!(
        //     "explode: ix: {} node: {:?}, {:?}",
        //     ix,
        //     self.nodes[ix],
        //     (l, r)
        // );

        // Find predecessor
        {
            let mut pred_ix = ix;
            let mut pred_subtree_ix = None;
            let mut is_left = false;
            let mut found = false;
            while let Some(t_ix) = self.nodes[pred_ix].parent {
                // println!(
                //     "pred_ix: {}, parent: {} ({:?})",
                //     pred_ix, t_ix, self.nodes[t_ix]
                // );
                if self.nodes[t_ix].right == Val::Sub(pred_ix) {
                    match self.nodes[t_ix].left {
                        Val::Num(_) => {
                            is_left = true;
                            found = true;
                            pred_ix = t_ix;
                        }
                        Val::Sub(l_ix) => {
                            pred_subtree_ix = Some(l_ix);
                        }
                    }
                    break;
                }
                pred_ix = t_ix;
            }
            while let Some(ix) = pred_subtree_ix {
                if let Val::Sub(r_ix) = self.nodes[ix].right {
                    pred_subtree_ix = Some(r_ix);
                } else {
                    found = true;
                    is_left = false;
                    pred_ix = ix;
                    break;
                }
            }
            if found {
                if is_left {
                    if let Val::Num(n) = self.nodes[pred_ix].left {
                        self.nodes[pred_ix].left = Val::Num(n + l);
                    }
                } else {
                    if let Val::Num(n) = self.nodes[pred_ix].right {
                        self.nodes[pred_ix].right = Val::Num(n + l);
                    }
                }
            }
        }

        // Find successor
        {
            let mut succ_ix = ix;
            let mut succ_subtree_ix = None;
            let mut is_left = false;
            let mut found = false;
            while let Some(t_ix) = self.nodes[succ_ix].parent {
                if self.nodes[t_ix].left == Val::Sub(succ_ix) {
                    match self.nodes[t_ix].right {
                        Val::Num(_) => {
                            is_left = false;
                            found = true;
                            succ_ix = t_ix;
                        }
                        Val::Sub(r_ix) => {
                            succ_subtree_ix = Some(r_ix);
                        }
                    }
                    break;
                }
                succ_ix = t_ix;
            }
            while let Some(ix) = succ_subtree_ix {
                if let Val::Sub(l_ix) = self.nodes[ix].left {
                    succ_subtree_ix = Some(l_ix);
                } else {
                    found = true;
                    is_left = true;
                    succ_ix = ix;
                    break;
                }
            }
            if found {
                if is_left {
                    if let Val::Num(n) = self.nodes[succ_ix].left {
                        self.nodes[succ_ix].left = Val::Num(n + r);
                    }
                } else {
                    if let Val::Num(n) = self.nodes[succ_ix].right {
                        self.nodes[succ_ix].right = Val::Num(n + r);
                    }
                }
            }
        }

        let parent_id = self.nodes[ix].parent.unwrap();
        if self.nodes[parent_id].left == Val::Sub(ix) {
            self.nodes[parent_id].left = Val::Num(0);
        } else if self.nodes[parent_id].right == Val::Sub(ix) {
            self.nodes[parent_id].right = Val::Num(0);
        } else {
            panic!("weird!");
        }
    }

    fn split_big_regular(self: &mut Self, ix: usize) -> bool {
        match self.nodes[ix].left {
            Val::Num(n) => {
                if n >= 10 {
                    // split this!
                    let (ln, rn) = if n % 2 == 1 {
                        (n / 2, n / 2 + 1)
                    } else {
                        (n / 2, n / 2)
                    };
                    let t_ix = self.add(Val::Num(ln), Val::Num(rn), Some(ix));
                    self.nodes[ix].left = Val::Sub(t_ix);
                    return true;
                }
            }
            Val::Sub(l_ix) => {
                if self.split_big_regular(l_ix) {
                    return true;
                }
            }
        };
        match self.nodes[ix].right {
            Val::Num(n) => {
                if n >= 10 {
                    // split this!
                    let (ln, rn) = if n % 2 == 1 {
                        (n / 2, n / 2 + 1)
                    } else {
                        (n / 2, n / 2)
                    };
                    let t_ix = self.add(Val::Num(ln), Val::Num(rn), Some(ix));
                    self.nodes[ix].right = Val::Sub(t_ix);
                    return true;
                }
            }
            Val::Sub(r_ix) => {
                if self.split_big_regular(r_ix) {
                    return true;
                }
            }
        };
        return false;
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Tree> {
    lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .map(|s| {
            let mut t: Tree = Default::default();
            t.tree_from(&s, 0);
            // println!("{:?}", node);
            t
        })
        .collect()
}

fn add(n1: Tree, n2: Tree) -> Tree {
    // print!("add: n1: {} n2: {}", n1.string(), n2.string());
    let n1_len = n1.nodes.len();
    let mut r: Tree = Default::default();
    r.add(Val::Num(0), Val::Num(0), None);
    // r.push()
    [n1, n2].into_iter().for_each(|tree| {
        let n = r.nodes.len();
        tree.nodes.into_iter().for_each(|mut node| {
            if let Val::Sub(c) = node.left {
                node.left = Val::Sub(c + n);
            }
            if let Val::Sub(c) = node.right {
                node.right = Val::Sub(c + n);
            }
            if let Some(c) = node.parent {
                node.parent = Some(c + n);
            }
            r.nodes.push(node);
        })
    });

    r.nodes[0].left = Val::Sub(1);
    r.nodes[0].right = Val::Sub(n1_len + 1);
    r.nodes[1].parent = Some(0);
    r.nodes[n1_len + 1].parent = Some(0);
    // println!(" sum: {}", r.string());
    // println!("r: {:?}", r);
    r
}

fn reduce(mut n: Tree) -> Tree {
    loop {
        // println!("red iter: {}", n.string());
        // Check if first rule applies.
        if let Some((ix, l, r)) = n.find_depth4_node(0, 0) {
            n.explode(ix, l, r);
        } else if n.split_big_regular(0) {
        } else {
            break;
        }
    }
    n
}

fn solve1(h: &Vec<Tree>) -> i64 {
    // println!("h[0]: {}", h[0].string());
    let sum = h.iter().skip(1).fold(h[0].clone(), |sum, x| {
        // println!("sum: {}, x: {}", sum.string(), x.string());
        let s = add(sum, x.clone());
        // println!("sum is: {}", s.string());
        reduce(s)
    });

    sum.magnitude(0)
}

fn solve2(h: &Vec<Tree>) -> i64 {
    let mut max = 0;
    for i in 0..h.len() {
        for j in 0..h.len() {
            if i != j {
                let s = reduce(add(h[i].clone(), h[j].clone()));
                let m = s.magnitude(0);
                if m > max {
                    max = m;
                }
            }
        }
    }
    max
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
