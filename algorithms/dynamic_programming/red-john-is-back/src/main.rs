use std::collections::HashMap;
use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<i64> {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .map(|line| {
            line.expect("Input not readable")
            .parse::<i64>().expect("Can't parse integer")
        });
    content.collect()
}

fn calc(x: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if x == 0 {
        return 1;
    }

    if let Some(&res) = cache.get(&x) {
        return res;
    }

    let res = calc(x-1, cache) + if x >= 4 { calc(x-4, cache) } else { 0 };
    cache.insert(x, res);
    res
}

// Modified version of http://code.activestate.com/recipes/117119/
fn sieve_counter(n: i64) -> i64 {
    let mut q = 2;
    let mut d = HashMap::new();
    let mut res = 0;

    loop {
        if let Some(p) = d.remove(&q) {
            let mut x = p + q;
            while d.contains_key(&x) {
                x += p;
            }
            d.insert(x, p);
        } else {
            if q > n {
                return res;
            }
            d.insert(q * q, q);
            res += 1;
        }
        q += 1;
    }
}

fn main() {
    let sizes = read_input();
    let mut cache = HashMap::new();

    for size in sizes {
        println!("{}", sieve_counter(calc(size, &mut cache)));
    }
}
