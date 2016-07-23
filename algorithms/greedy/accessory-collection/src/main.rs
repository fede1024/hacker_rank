use std::collections::HashSet;
use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<(u32, u32, u32, u32)> {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .map(|line| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<u32>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        })
        .map(|row| {(row[0], row[1], row[2], row[3])})
        .collect::<Vec<_>>();
    content
}

fn subsets(v: &[u32], n: u32) -> Vec<Vec<u32>> {
    if v.is_empty() || n == 0 {
        return vec![Vec::new()]
    }

    let mut ret = Vec::new();

    if v.len() > (n as usize) {
        for sub in subsets(&v[1..], n) {
            ret.push(sub);
        }
    }

    for mut sub in subsets(&v[1..], n-1) {
        sub.push(v[0]);
        ret.push(sub);
    }

    ret
}

fn subsets_rep(v: &Vec<u32>, n: u32) -> Vec<Vec<u32>> {
    if v.is_empty() || n == 0 {
        return vec![Vec::new()]
    }

    let mut ret = Vec::new();

    for sub in subsets_rep(v, n-1) {
        for &i in v {
            let mut copy = sub.clone();
            copy.push(i);
            ret.push(copy);
        }
    }

    ret
}

fn brute_force(l: u32, a: u32, n: u32, d: u32) -> (u32, Vec<u32>) {
    let v = (1..(a+1)).collect::<Vec<_>>();

    let mut best = (0, Vec::new());
    for sol in subsets_rep(&v, l) {
        let mut valid = true;
        for subset in subsets(&sol, n) {
            let types = subset.iter().collect::<HashSet<_>>();
            if types.len() < (d as usize) {
                valid = false;
                break;
            }
        }
        let tot = sol.iter().fold(0, |acc, e| acc + e);
        if tot > best.0 && valid {
            best = (tot, sol.clone());
        }
    }

    best
}

fn main() {
    for (l, a, n, d) in read_input() {
        let best = brute_force(l, a, n ,d);
        if best.0 == 0 {
            println!("SAD");
        } else {
            println!("{} {:?}", best.0, best.1);
        }
    }
}
