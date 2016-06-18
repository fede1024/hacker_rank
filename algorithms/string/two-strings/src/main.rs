use std::collections::HashSet;
use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<(String, String)> {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .map(|line| { line.expect("Input not readable") })
        .collect::<Vec<_>>();

    content.chunks(2).map(|x| (x[0].clone(), x[1].clone())).collect::<Vec<_>>()
}

fn main() {
    for (a, b) in read_input() {
        let chars_set = a.chars().collect::<HashSet<_>>();
        if b.chars().find(|&x| chars_set.contains(&x)).is_some() {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
