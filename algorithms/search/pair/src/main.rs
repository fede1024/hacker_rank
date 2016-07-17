use std::collections::HashSet;
use std::io::prelude::*;
use std::io;

fn read_input() -> (i32, Vec<i32>) {
    let stdin = io::stdin();
    let mut content = stdin.lock()
        .lines()
        .map(|line| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<i32>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        });
    (content.next().unwrap()[1], content.next().unwrap())
}

fn main() {
    let (k, seq) = read_input();
    let mut set = HashSet::new();
    let mut res = 0;

    for a in seq {
        if set.contains(&(a - k)) {
            res += 1;
        }
        if set.contains(&(a + k)) {
            res += 1;
        }
        set.insert(a);
    }

    println!("{}", res);
}
