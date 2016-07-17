use std::io::prelude::*;
use std::io;

fn read_input() -> (Vec<i32>, i32, i32) {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .map(|line| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<i32>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (content[1].clone(), content[2][0], content[2][1])
}

fn find_closest(vec: &Vec<i32>, n: i32) -> i32 {
    vec.iter().min_by_key(|&x| (n - x).abs()).cloned().unwrap()
}

fn main() {
    let (mut vec, p, q) = read_input();
    vec.sort();

    let mut pairs = vec.iter()
        .zip(vec.iter().skip(1))
        .map(|(&a, &b)| ((a + b) / 2, (b - a) / 2))
        .filter(|&(m, _)| p <= m && m <= q)
        .collect::<Vec<_>>();

    pairs.push((p, (p - find_closest(&vec, p)).abs()));
    pairs.push((q, (q - find_closest(&vec, q)).abs()));

    pairs.sort_by_key(|&(_, dist)| -dist);

    let sol = pairs.iter()
        .filter(|&&(_, dist)| dist == pairs[0].1)
        .min_by_key(|&&(m, _)| m);

    println!("{}", sol.unwrap().0);
}
