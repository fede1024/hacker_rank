use std::collections::HashMap;
use std::io::prelude::*;
use std::io;

fn read_input() -> (i64, Vec<i64>) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.expect("Input not readable"));
    let content = lines.map(|line| {
            line.split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (content[0][0], content[1].clone())
}

fn calc(tot: i64, coins: &[i64], index: usize, cache: &mut HashMap<(i64, usize), i64>) -> i64 {
    if tot == 0 {
        return 1;
    }
    if tot < 0 || coins.is_empty() || index == coins.len() {
        return 0;
    }

    if let Some(&n) = cache.get(&(tot, index)) {
        return n;
    }

    let sol = calc(tot - coins[index], coins, index, cache) + calc(tot, coins, index + 1, cache);
    cache.insert((tot, index), sol);
    sol
}

fn main() {
    let (tot, coins) = read_input();
    let mut cache = HashMap::new();
    println!("{}", calc(tot, &coins, 0, &mut cache));
}
