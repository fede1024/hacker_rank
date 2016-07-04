use std::io::prelude::*;
use std::io;

const MOD: u64 = 1000000007;

fn read_input() -> Vec<Vec<u32>> {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .enumerate()
        .filter(|&(n, _)| n % 2 == 1)
        .map(|(_ , line)| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<u32>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        });
    content.collect::<Vec<_>>()
}

fn count_ones(v: &Vec<u32>, pos: u32) -> u32 {
    let mask = 1u32 << pos;
    v.iter().filter(|&n| n & mask != 0).count() as u32
}

// From: http://stackoverflow.com/a/24500377/1025899
fn bin_coeff(n: u64, k: u64, m: u64) -> u64 {
    let mut coef = 1;
    let mut sieve = vec![false; (n+1) as usize];

    for p in 2..(n + 1) {
        if sieve[p as usize] {
            continue;
        }
        let mut i = p * p;
        while i <= n {
            sieve[i as usize] = true;
            i += p;
        }
        let mut pow = p;
        while pow <= n {
            let cnt = n / pow - k / pow - (n - k) / pow;
            for _ in 0..cnt {
                coef *= p;
                coef %= m;
            }
            pow *= p;
        }
    }
    coef
}

fn mod_pow(exp: u64) -> u64 {
    let mut c = 1;
    for _ in 1..(exp+1) {
        c = (c * 2u64) % MOD;
    }
    c
}

fn calc(input: &Vec<u32>) -> u64 {
    let mut tot = 0;
    let n = input.len() as u32;
    for pos in 0..31 {
        let p = count_ones(&input, pos);
        let mut res = 0;
        for k in (1..(p + 1)).filter(|n| n % 2 == 1) {
            res += mod_pow((n-p) as u64) * bin_coeff(p as u64, k as u64, MOD);
            res %= MOD;
        }
        tot += (res * mod_pow(pos as u64)) % MOD;
        tot %= MOD;
    }
    tot
}

fn main() {
    for input in read_input() {
        println!("{}", calc(&input));
    }
}
