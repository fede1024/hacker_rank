use std::cmp::max;
use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<(i64, i8, i64)> {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .map(|line| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<i64>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        });
    let mut steps = Vec::new();
    for line in content {
        steps.push((line[0], 0, line[2]));   // step up
        steps.push((line[1], 1, -line[2]));  // step down
    }
    steps.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    steps
}

fn main() {
    let res = read_input().iter()
        .fold((std::i64::MIN, 0), |(res, curr), &(_, _, v)| (max(res, curr + v), curr + v)).0;
    println!("{}", res);
}
