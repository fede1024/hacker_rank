use std::collections::HashMap;
use std::io::prelude::*;

const MOD: u64 = 1000000007;

use std::io;

fn read_input() -> (u64, u64) {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .map(|line| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<u64>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (content[0][0], content[1][0])
}

fn calc_mem(n: i64, p: u64, prev: u64, cache: &mut Vec<HashMap<u64, u64>>) -> u64 {
    let mut tot = 0;
    let l = p/prev;

    if n == 0 {
        return 1;
    }

    if let Some(&res) = cache[n as usize].get(&l) {
        return res;
    }

    for i in 1..(l+1) {
        tot += calc_mem(n-1, p, i, cache);
        tot %= MOD;
    }

    cache[n as usize].insert(l, tot);

    tot
}


fn main() {
    let (n, p) = read_input();
    let mut caches = vec![HashMap::new(); (n+1) as usize];

    println!("{}", calc_mem(n as i64, p, 1, &mut caches));
}
