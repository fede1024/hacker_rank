use std::cmp::max;
use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<i64> {
    let stdin = io::stdin();
    let numbers = stdin.lock()
        .lines()
        .map(|x| {
            x.expect("Input not readable")
                .parse::<i64>()
                .expect("Integer not readable")
        })
        .skip(1)
        .collect::<Vec<_>>();
    numbers
}

fn rank(v: &Vec<i64>) -> Vec<i64> {
    let mut ranking = Vec::new();
    let mut seq = 1;

    ranking.push(1);
    for i in 1..v.len() {
        seq = match v[i] > v[i - 1] {
            true => seq + 1,
            false => 1,
        };
        ranking.push(seq);
    }

    ranking
}

fn rev_clone<T: Clone>(v: &Vec<T>) -> Vec<T> {
    v.iter().rev().cloned().collect::<Vec<T>>()
}

fn main() {
    let v = read_input();
    let r1 = rank(&v);
    let r2 = rev_clone(&rank(&rev_clone(&v)));
    let sol = r1.iter().zip(&r2).map(|(v1, v2)| max(v1, v2)).fold(0, |acc, n| acc + n);
    println!("{}", sol);
}
