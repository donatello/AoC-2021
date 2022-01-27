use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut h: HashMap<String, Vec<String>> = HashMap::new();
    lines.iter().for_each(|line| {
        let parts = (*line)
            .split('-')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        for i in 0..2 {
            if let Some(v) = h.get_mut(&parts[i]) {
                v.push(parts[1 - i].clone());
            } else {
                h.insert(parts[i].clone(), vec![parts[1 - i].clone()]);
            }
        }
    });
    h
}

fn is_big(s: &str) -> bool {
    s.chars().nth(0).unwrap().is_uppercase()
}

fn visited(node: &str, path: &Vec<&str>) -> bool {
    path.iter().filter(|x| **x == node).nth(0).is_some()
}

fn dfs<'a>(
    node: &'a str,
    h: &'a HashMap<String, Vec<String>>,
    path: &mut Vec<&'a str>,
    path_count: &mut i64,
) {
    path.push(node);
    if node == "end" {
        *path_count += 1;
        path.pop();
        return;
    }
    let nbrs = h.get(node).unwrap();
    for nbr in nbrs.iter() {
        let can_visit = is_big(nbr) || !visited(nbr, path);
        if can_visit {
            dfs(nbr, h, path, path_count);
        }
    }
    path.pop();
}

fn solve1(h: &HashMap<String, Vec<String>>) -> i64 {
    let mut count = 0;
    dfs("start", h, &mut vec![], &mut count);
    count
}

fn visited2(node: &str, path: &Vec<&str>, twice_visited: &Option<&str>) -> bool {
    // node is a small cave.
    let exists = path.iter().filter(|x| **x == node).nth(0).is_some();
    // If node is not in path, we have not visited it.
    if !exists {
        return false;
    }
    // If node is in path, check if some (possibly other node) has already been
    // visited twice.
    if let Some(_) = twice_visited {
        return true;
    }
    // If not, node can be visited for a second time.
    return false;
}

fn dfs2<'a>(
    node: &'a str,
    h: &'a HashMap<String, Vec<String>>,
    path: &mut Vec<&'a str>,
    twice_visited: &mut Option<&'a str>,
    path_count: &mut i64,
) {
    // Since we got here, it is valid to visit node.

    // If node already is in path, it is being visited twice.
    if !is_big(node) && visited(node, path) {
        *twice_visited = Some(node);
    }
    path.push(node);
    if node == "end" {
        // println!("{:?}", path);
        *path_count += 1;
        path.pop();
        return;
    }
    let nbrs = h.get(node).unwrap();
    // println!("{} -> {:?}", node, nbrs);
    for nbr in nbrs.iter() {
        // println!(
        //     "nbr={} (node={}) -> path={:?} twice_visited={:?}",
        //     nbr, node, path, twice_visited
        // );
        let can_visit = nbr != "start" && (is_big(nbr) || !visited2(nbr, path, twice_visited));
        if can_visit {
            dfs2(nbr, h, path, twice_visited, path_count);
        }
    }
    path.pop();
    if *twice_visited == Some(node) {
        *twice_visited = None;
    }
}

fn solve2(h: &HashMap<String, Vec<String>>) -> i64 {
    let mut count = 0;
    dfs2("start", h, &mut vec![], &mut None, &mut count);
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
