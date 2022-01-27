use std::io::{self, BufRead};

fn parse_input(lines: &Vec<String>) -> Vec<u8> {
    lines[0].split(',').map(|x| x.parse().unwrap()).collect()
}

/*

    0,
 1: 6,8
 2: 5,7
 3: 4,6
 4: 3,5
 5: 2,4
 6: 1,3
 7: 0,2
 8: 6,1,8
 9: 5,0,7
10: 4,6,6,8
11: 3,5,5,7
12: 2,4,4,6
13: 1,3,3,5
14: 0,2,2,4
15: 6,1,1,3,8
16: 5,0,0,2,7
17: 4,6,6,1,6,8,8
18: 3,5,5,0,5,7,7


*/

fn solve1(fish: &Vec<u8>) -> u64 {
    // f(n, k) = (n-k)/7 + 1 + Sigma(f(n-k-7i, 8))

    let mut counts = [[0 as u64; 9]; 81];
    for n in 1..81 {
        for k in 0..9 {
            // count[n][k] => number of fish produced by a fish with initial
            // timer k after n days.
            if n > k {
                // Number of direct children
                let c = (n - k - 1) / 7 + 1;
                counts[n][k] = c as u64;
                // For i-th child where first child has i=0:
                // it is born on day k+1, and has (n-k-1)
                let mut d = 0;
                while d < c && n - k - 1 - 7 * d > 0 {
                    counts[n][k] += counts[n - k - 1 - 7 * d][8];
                    d += 1;
                }
            }
        }
    }

    // print!("k: ");
    // for k in 0..9 {
    //     print!("{} ", k);
    // }
    // println!("");
    // for n in 1..20 {
    //     print!("{}: ", n);
    //     for k in 0..9 {
    //         print!("{} ", counts[n][k]);
    //     }
    //     println!("");
    // }

    fish.iter().map(|x| counts[80][*x as usize]).sum::<u64>() + fish.len() as u64
}

fn solve2(fish: &Vec<u8>) -> u64 {
    let mut counts = [[0 as u64; 9]; 257];
    for n in 1..257 {
        for k in 0..9 {
            // count[n][k] => number of fish produced by a fish with initial
            // timer k after n days.
            if n > k {
                // Number of direct children
                let c = (n - k - 1) / 7 + 1;
                counts[n][k] = c as u64;
                // For i-th child where first child has i=0:
                // it is born on day k+1, and has (n-k-1)
                let mut d = 0;
                while d < c && n - k - 1 - 7 * d > 0 {
                    counts[n][k] += counts[n - k - 1 - 7 * d][8];
                    d += 1;
                }
            }
        }
    }

    fish.iter().map(|x| counts[256][*x as usize]).sum::<u64>() + fish.len() as u64
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let v: Vec<String> = stdin.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let lines = parse_input(&v);
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
    Ok(())
}
