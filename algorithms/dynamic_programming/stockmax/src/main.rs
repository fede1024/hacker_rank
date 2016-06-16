use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<Vec<i64>> {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .enumerate()
        .filter(|&(n, _)| n % 2 == 1)
        .map(|(_, line)| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<i64>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        });
    content.collect()
}

fn calc_test(v: &[i64]) -> i64 {
    if v.is_empty() {
        return 0;
    }

    let last_max = v.iter().enumerate().fold(0, |m, (n, &x)| if x >= v[m] { n } else { m });
    let spent = v.iter().take(last_max).fold(0, |acc, n| acc + n);
    let earned = last_max as i64 * v[last_max];

    earned - spent + calc_test(&v[last_max+1..])
}

fn main() {
    for input in read_input() {
        println!("{}", calc_test(&input));
    }
}
