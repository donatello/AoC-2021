use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct ALU {
    // input: Vec<i64>,
    i_count: usize,
    double_eq: Vec<Term>,

    w_term: Term,
    x_term: Term,
    y_term: Term,
    z_term: Term,
}

#[derive(Debug, Clone, Copy)]
enum Var {
    W,
    X,
    Y,
    Z,
}

impl Var {
    fn parse(s: &str) -> Self {
        match s {
            "w" => Var::W,
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            _ => panic!("var"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone, Copy)]
enum Cmd {
    Inp(Var),
    Cmd(Op, Var, Val),
}

#[derive(Debug, Clone, Copy)]
enum Val {
    V(Var),
    L(i64),
}

impl Val {
    fn parse(s: &str) -> Self {
        if let Ok(n) = s.parse::<i64>() {
            Val::L(n)
        } else {
            Val::V(match s {
                "w" => Var::W,
                "x" => Var::X,
                "y" => Var::Y,
                "z" => Var::Z,
                _ => panic!("var"),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum Term {
    L(i64),
    I(usize),
    Add(Vec<Box<Term>>),
    Mul(Vec<Box<Term>>),
    Div(Box<Term>, Box<Term>),
    Mod(Box<Term>, Box<Term>),
    Eql(Box<Term>, Box<Term>),
    E(usize),
}

impl Term {
    fn get_deps(&self, i: &mut HashSet<usize>, e: &mut HashSet<usize>) {
        match self {
            Term::L(_) => {}
            Term::I(k) => {
                i.insert(*k);
            }
            Term::E(k) => {
                e.insert(*k);
            }
            Term::Add(v) => v.iter().for_each(|t| t.get_deps(i, e)),
            Term::Mul(v) => v.iter().for_each(|t| t.get_deps(i, e)),
            Term::Div(a, b) => {
                a.get_deps(i, e);
                b.get_deps(i, e);
            }
            Term::Mod(a, b) => {
                a.get_deps(i, e);
                b.get_deps(i, e);
            }
            Term::Eql(a, b) => {
                a.get_deps(i, e);
                b.get_deps(i, e);
            }
        }
    }

    fn min_max(&self) -> Option<(i64, i64)> {
        match self {
            Term::L(n) => Some((*n, *n)),
            Term::I(_) => Some((1, 9)),
            Term::Add(v) => {
                let k = v
                    .iter()
                    .map(|x| x.min_max())
                    .collect::<Option<Vec<(i64, i64)>>>();
                k.map(|v| v.iter().fold((0, 0), |(ma, mb), (a, b)| (ma + a, mb + b)))
            }
            Term::Mul(v) => {
                let k = v
                    .iter()
                    .map(|x| x.min_max())
                    .collect::<Option<Vec<(i64, i64)>>>();
                k.map(|v| {
                    let mins = v.iter().map(|x| x.0).product::<i64>();
                    let maxs = v.iter().map(|x| x.1).product::<i64>();
                    (mins, maxs)
                })
            }
            Term::Mod(_, k) => {
                if let Term::L(n) = **k {
                    Some((-(n - 1), n - 1))
                } else {
                    None
                }
            }
            Term::Div(a, b) => {
                if let Some((min_a, max_a)) = a.min_max() {
                    if let Some((min_b, max_b)) = b.min_max() {
                        if min_b == max_b {
                            return Some((min_a / min_b, max_a / min_b));
                        }
                    }
                }
                return None;
            }
            Term::Eql(_, _) => Some((0, 1)),
            Term::E(_) => Some((0, 1)),
        }
    }

    fn add(ts: Vec<Box<Term>>) -> Term {
        let mut v: Vec<Box<Term>> = vec![];
        ts.iter().for_each(|t| match &**t {
            Term::Add(s) => v.extend((*s).clone()),
            _ => v.push((*t).clone()),
        });
        let mut r: Vec<Box<Term>> = vec![];
        let mut sum = 0;
        v.iter().for_each(|t| match **t {
            Term::L(n) => sum += n,
            _ => r.push(Box::new(*t.clone())),
        });
        r.sort_unstable();
        if sum != 0 {
            r.push(Box::new(Term::L(sum)));
        }
        if r.len() == 0 {
            Term::L(0)
        } else if r.len() == 1 {
            *r[0].clone()
        } else {
            Term::Add(r)
        }
    }

    fn mul(ts: Vec<Box<Term>>) -> Term {
        let mut v: Vec<Box<Term>> = vec![];
        ts.iter().for_each(|t| match &**t {
            Term::Mul(s) => v.extend((*s).clone()),
            _ => v.push((*t).clone()),
        });
        let mut r: Vec<Box<Term>> = vec![];
        let mut product = 1;
        v.iter().for_each(|t| match **t {
            Term::L(n) => product *= n,
            _ => r.push(Box::new(*t.clone())),
        });
        r.sort_unstable();
        if product == 0 {
            Term::L(0)
        } else {
            if product != 1 {
                r.push(Box::new(Term::L(product)));
            }
            if r.len() == 1 {
                *r[0].clone()
            } else {
                Term::Mul(r)
            }
        }
    }

    fn divides_term(t: Term, n: i64) -> bool {
        match t {
            Term::L(k) => k % n == 0,
            Term::Mul(vs) => {
                vs.iter()
                    .filter(|x| {
                        if let Term::L(k) = ***x {
                            k % n == 0
                        } else {
                            false
                        }
                    })
                    .count()
                    > 0
            }
            _ => false,
        }
    }

    fn modt(lhs_term: Term, rhs_term: Term) -> Term {
        if Term::L(1) == rhs_term {
            Term::L(0)
        } else if Term::L(0) == lhs_term {
            Term::L(0)
        } else {
            match lhs_term.clone() {
                Term::L(n) => {
                    if let Term::L(k) = rhs_term {
                        return Term::L(n % k);
                    }
                }
                Term::Mod(a, b) => {
                    // Simplify mod(mod(a, b), b) => mod(a, b)
                    if b == Box::new(rhs_term.clone()) {
                        return Term::Mod(a, b);
                    }
                }
                Term::Mul(ts) => {
                    // MOD(MUL(a, b), c) = 0 when c|a or c|b.
                    if let Term::L(n) = rhs_term.clone() {
                        let div_terms = ts
                            .iter()
                            .filter(|x| {
                                if let Term::L(k) = ***x {
                                    k % n == 0
                                } else {
                                    false
                                }
                            })
                            .count();
                        if div_terms > 0 {
                            return Term::L(0);
                        }
                    }
                }
                Term::Add(vs) => {
                    // MOD(Add(a, b), c) = MOD(a, c) when c|b
                    if let Term::L(n) = rhs_term {
                        let mut nv = vec![];
                        vs.iter().for_each(|v| {
                            if !Term::divides_term((**v).clone(), n) {
                                nv.push((*v).clone())
                            }
                        });
                        if nv.len() < vs.len() {
                            let res_term =
                                Term::add(vec![Box::new(Term::Add(nv)), Box::new(Term::L(0))]);
                            return Term::modt(res_term, rhs_term);
                        }
                    }
                }
                _ => {}
            }
            if let Some((lmin, lmax)) = lhs_term.min_max() {
                if let Term::L(n) = rhs_term {
                    if lmin > -n && lmax < n {
                        return lhs_term;
                    }
                }
            }
            Term::Mod(Box::new(lhs_term), Box::new(rhs_term))
        }
    }

    fn div(lhs_term: Term, rhs_term: Term) -> Term {
        match (lhs_term.clone(), rhs_term.clone()) {
            (Term::L(a), Term::L(b)) => Term::L(a / b),
            (_, Term::L(1)) => lhs_term,
            (Term::L(0), _) => Term::L(0),
            _ => Term::Div(Box::new(lhs_term), Box::new(rhs_term)),
        }
    }

    fn eql(lhs_term: Term, rhs_term: Term) -> Term {
        if lhs_term == rhs_term {
            return Term::L(1);
        }
        match (lhs_term.min_max(), rhs_term.min_max()) {
            (Some((lmin, lmax)), Some((rmin, rmax))) => {
                if lmax < rmin || rmax < lmin {
                    return Term::L(0);
                }
                if lmin == lmax && rmin == rmax && lmin == rmin {
                    return Term::L(1);
                }
            }
            _ => {}
        }
        Term::Eql(Box::new(lhs_term), Box::new(rhs_term))
    }

    fn eval(&self, inputs: &Vec<i64>, double_eqs: &Vec<i64>) -> Term {
        match self {
            Term::L(_) => self.clone(),
            Term::I(k) => inputs.get(*k).map_or(self.clone(), |v| {
                if *v != 0 {
                    Term::L(*v)
                } else {
                    self.clone()
                }
            }),
            Term::E(k) => double_eqs.get(*k).map_or(self.clone(), |v| Term::L(*v)),
            Term::Add(vs) => Term::add(
                vs.iter()
                    .map(|v| Box::new(v.eval(inputs, double_eqs)))
                    .collect(),
            ),
            Term::Mul(vs) => Term::mul(
                vs.iter()
                    .map(|v| Box::new(v.eval(inputs, double_eqs)))
                    .collect(),
            ),
            Term::Mod(a, b) => Term::modt(a.eval(inputs, double_eqs), b.eval(inputs, double_eqs)),
            Term::Div(a, b) => Term::div(a.eval(inputs, double_eqs), b.eval(inputs, double_eqs)),
            Term::Eql(a, b) => Term::eql(a.eval(inputs, double_eqs), b.eval(inputs, double_eqs)),
        }
    }
}

impl ALU {
    fn print(&self) {
        // println!("ALU: input={}", self.input.len());
        println!("ALU:");
        println!("w_term: {:?}", self.w_term);
        println!("x_term: {:?}", self.x_term);
        println!("y_term: {:?}", self.y_term);
        println!("z_term: {:?}", self.z_term);
    }

    fn new() -> Self {
        ALU {
            // input: vec![],
            i_count: 0,
            double_eq: vec![],
            w_term: Term::L(0),
            x_term: Term::L(0),
            y_term: Term::L(0),
            z_term: Term::L(0),
        }
    }

    fn get_prval_term(&self, p: &Val) -> Term {
        match p {
            Val::L(n) => Term::L(*n),
            Val::V(v) => self.get_var_term(*v),
        }
    }

    fn get_var_term(&self, v: Var) -> Term {
        match v {
            Var::W => self.w_term.clone(),
            Var::X => self.x_term.clone(),
            Var::Y => self.y_term.clone(),
            Var::Z => self.z_term.clone(),
        }
    }

    fn inp(&mut self, v: Var) {
        let t = Term::I(self.i_count);
        match v {
            Var::W => self.w_term = t,
            Var::X => self.x_term = t,
            Var::Y => self.y_term = t,
            Var::Z => self.z_term = t,
        };
        self.i_count += 1;
    }

    fn eql(&mut self, lhs_term: Term, rhs_term: Term) -> Term {
        let r = Term::eql(lhs_term, rhs_term);
        let is_eq = |t| match t {
            Term::Eql(_, _) => true,
            _ => false,
        };
        let is_double_eq = match r.clone() {
            Term::Eql(a, b) => is_eq(*a) || is_eq(*b),
            _ => false,
        };
        if is_double_eq {
            self.double_eq.push(r);
            return Term::E(self.double_eq.len() - 1);
        }
        r
    }

    fn op(&mut self, op: Op, var: Var, val: Val) {
        let lhs_term = self.get_var_term(var);
        let rhs_term = self.get_prval_term(&val);

        match (&lhs_term, &rhs_term) {
            (Term::L(a), Term::L(b)) => {
                let op_result = match op {
                    Op::Add => a + b,
                    Op::Mul => a * b,
                    Op::Div => a / b,
                    Op::Mod => a % b,
                    Op::Eql => {
                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                };

                match var {
                    Var::W => self.w_term = Term::L(op_result),
                    Var::X => self.x_term = Term::L(op_result),
                    Var::Y => self.y_term = Term::L(op_result),
                    Var::Z => self.z_term = Term::L(op_result),
                }
            }
            _ => {
                let op_term = match op {
                    Op::Add => Term::add(vec![Box::new(lhs_term), Box::new(rhs_term)]),
                    Op::Mul => Term::mul(vec![Box::new(lhs_term), Box::new(rhs_term)]),
                    Op::Div => Term::div(lhs_term, rhs_term),
                    Op::Mod => Term::modt(lhs_term, rhs_term),
                    Op::Eql => self.eql(lhs_term, rhs_term),
                };
                match var {
                    Var::W => self.w_term = op_term,
                    Var::X => self.x_term = op_term,
                    Var::Y => self.y_term = op_term,
                    Var::Z => self.z_term = op_term,
                };
            }
        }
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Cmd> {
    lines
        .iter()
        .map(|s| {
            let ps = s.split(" ").collect::<Vec<_>>();
            match ps[0] {
                "inp" => Cmd::Inp(Var::parse(ps[1])),
                "add" => Cmd::Cmd(Op::Add, Var::parse(ps[1]), Val::parse(ps[2])),
                "mul" => Cmd::Cmd(Op::Mul, Var::parse(ps[1]), Val::parse(ps[2])),
                "div" => Cmd::Cmd(Op::Div, Var::parse(ps[1]), Val::parse(ps[2])),
                "mod" => Cmd::Cmd(Op::Mod, Var::parse(ps[1]), Val::parse(ps[2])),
                "eql" => Cmd::Cmd(Op::Eql, Var::parse(ps[1]), Val::parse(ps[2])),
                _ => panic!("oops"),
            }
        })
        .collect()
}

fn next_dec(v: &mut Vec<i64>) -> bool {
    let mut n = v.len() - 1;
    loop {
        if v[n] == 0 {
            n -= 1;
            continue;
        }
        if v[n] > 1 {
            v[n] -= 1;
            break true;
        }
        if n > 0 {
            v[n] = 9;
            n -= 1;
        } else {
            break false;
        }
    }
}

fn next_inc(v: &mut Vec<i64>) -> bool {
    let mut n = v.len() - 1;
    loop {
        if v[n] == 0 {
            n -= 1;
            continue;
        }
        if v[n] < 9 {
            v[n] += 1;
            break true;
        }
        if n > 0 {
            v[n] = 1;
            n -= 1;
        } else {
            break false;
        }
    }
}

fn solve_it(
    de: &Vec<i64>,
    eq_exp: &Vec<Term>,
    input: &mut Vec<i64>,
    z_term: &Term,
    s: &Term,
    i: usize,
    find_smallest: bool,
) -> bool {
    if i >= eq_exp.len() {
        println!("FOUND: {:?}", input);
        return true;
    }

    // println!("i: {}, input: {:?}", i, input);

    let mut is = HashSet::new();
    let mut es = HashSet::new();
    eq_exp[i].get_deps(&mut is, &mut es);

    let mut inps = is
        .into_iter()
        .filter(|x| input[*x] == 0)
        .collect::<Vec<usize>>();
    inps.sort_unstable();
    if inps.len() == 0 {
        if eq_exp[i].eval(&input, &de) == Term::L(de[i]) {
            // recurse
            if solve_it(de, eq_exp, input, z_term, s, i + 1, find_smallest) {
                return true;
            }
        }
        return false;
    }
    let mut v = vec![9; inps.len()];
    if find_smallest {
        v = vec![1; inps.len()];
    }
    loop {
        inps.iter()
            .enumerate()
            .for_each(|(i, inp_i)| input[*inp_i] = v[i]);
        let res = eq_exp[i].eval(&input, &de);
        if let Term::L(_) = res {
        } else {
            panic!("did not eval: {:?}", res);
        }
        // println!("With: i={} {:?}", i, input);
        // println!("Res: {:?} for {:?}", res, eq_exp[i]);
        if res == Term::L(de[i]) {
            // recurse
            if solve_it(de, eq_exp, input, z_term, s, i + 1, find_smallest) {
                return true;
            }
        }
        let found = if !find_smallest {
            next_dec(&mut v)
        } else {
            next_inc(&mut v)
        };
        if !found {
            // Reset modified positions of input to zero.
            inps.iter().for_each(|ix| input[*ix] = 0);
            break false;
        }
    }
}

fn solve1(h: &Vec<Cmd>) -> i64 {
    let mut alu = ALU::new();
    // Symbolic evaluation and simple simplifications:
    h.iter().enumerate().for_each(|(_i, s)| match s {
        Cmd::Inp(v) => alu.inp(*v),
        Cmd::Cmd(op, var, arg) => alu.op(*op, *var, *arg),
    });

    println!("ZTermMinMax: {:?}", alu.z_term.min_max());
    let n = alu.double_eq.len();

    // There are n terms like Term::E(i) all of which have only 0 or 1 values.
    // Try all possible values for these expressions and see if z_term still can
    // be 0.

    let mut de = vec![0; n];
    let mut valid = 0;
    for i in 0..(1 << n) {
        for j in 0..n {
            if i & (1 << j) != 0 {
                de[j] = 1;
            } else {
                de[j] = 0;
            }
        }

        // eval z-term
        let s = alu.z_term.eval(&vec![], &de);
        // Are we lucky?! (got a min-max value?)
        let (min, max) = s.min_max().unwrap();
        if min > 0 || max < 0 {
            continue;
        }
        valid += 1;

        // This just appears to be the case.
        if min != max {
            panic!("oops!");
        }

        // println!("MinMax: {:?}", s.min_max());
        println!("{}: With: {:?}", valid, de);

        // Simplify the Term::E(i) expressions given that the other Term::E(j)
        // values are known. Now each Term::E(i) will be in terms of inputs,
        // operations and literals.
        let mut de_exp = vec![];
        alu.double_eq
            .iter()
            .for_each(|exp| de_exp.push(exp.eval(&vec![], &de)));

        // Now try each possible value of the inputs till we get some values
        // satisfying the Term::E(i) constraints.
        let mut input = vec![0; 14];
        let rs = solve_it(&de, &de_exp, &mut input, &alu.z_term, &s, 0, false);
        if rs {
            return input.iter().fold(0, |a, i| a * 10 + i);
        }
    }
    // println!("Valid: {}", valid);
    0
}

fn solve2(h: &Vec<Cmd>) -> i64 {
    let mut alu = ALU::new();
    h.iter().enumerate().for_each(|(_i, s)| match s {
        Cmd::Inp(v) => alu.inp(*v),
        Cmd::Cmd(op, var, arg) => alu.op(*op, *var, *arg),
    });

    println!("ZTermMinMax: {:?}", alu.z_term.min_max());
    let n = alu.double_eq.len();
    let mut de = vec![0; n];
    let mut valid = 0;
    for i in 0..(1 << n) {
        for j in 0..n {
            if i & (1 << j) != 0 {
                de[j] = 1;
            } else {
                de[j] = 0;
            }
        }

        // eval z-term
        let s = alu.z_term.eval(&vec![], &de);
        // Are we lucky?!
        let (min, max) = s.min_max().unwrap();
        if min > 0 || max < 0 {
            continue;
        }
        valid += 1;
        if min != max {
            panic!("oops!");
        }

        // println!("MinMax: {:?}", s.min_max());
        println!("{}: With: {:?}", valid, de);
        // println!("ZTerm Before: {:?}", alu.z_term);
        // println!("ZTerm After: {:?}", s);
        // let mut i = HashSet::new();
        // let mut e = HashSet::new();
        // s.get_deps(&mut i, &mut e);
        // println!(
        //     "Deps: I:({}) E:({})\n Is:{:?} Es:{:?}",
        //     i.len(),
        //     e.len(),
        //     i,
        //     e
        // );

        let mut de_exp = vec![];
        alu.double_eq
            .iter()
            .for_each(|exp| de_exp.push(exp.eval(&vec![], &de)));

        let mut input = vec![0; 14];
        let rs = solve_it(&de, &de_exp, &mut input, &alu.z_term, &s, 0, true);
        if rs {
            return input.iter().fold(0, |a, i| a * 10 + i);
        }
    }
    // println!("Valid: {}", valid);

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
