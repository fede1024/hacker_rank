use std::cmp::max;
use std::cmp::min;
use std::io::prelude::*;
use std::io;
use std::cell::Cell;
use std::borrow::Borrow;

const MIN_S: i32 = 0;
const MAX_S: i32 = 100000;
const MOD: i64 = 1000000007;

#[derive(Debug, Eq, PartialEq)]
struct Stain(i32, i32, Cell<bool>);

fn read_input() -> (i32, Vec<Stain>) {
    let stdin = io::stdin();
    let mut content = stdin.lock()
        .lines()
        .map(|line| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<i32>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        });
    let clean = content.next().unwrap()[1];
    let stains = content.map(|v| Stain(v[0], v[1], Cell::new(false))).collect();
    (clean, stains)
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Borders {
    startx: i32,
    endx: i32,
    starty: i32,
    endy: i32,
}

impl Borders {
    fn new(startx: i32, endx: i32, starty: i32, endy: i32) -> Borders {
        Borders {
            startx: startx,
            endx: endx,
            starty: starty,
            endy: endy,
        }
    }

    fn from_stains<T: Borrow<Stain>>(stains: &Vec<T>) -> Borders {
        let mut b = Borders::new(MAX_S, MIN_S, MAX_S, MIN_S);

        for stain in stains {
            let stain = stain.borrow();
            if stain.2.get() {
                continue;
            }
            b.startx = min(stain.0, b.startx);
            b.endx = max(stain.0, b.endx);
            b.starty = min(stain.1, b.starty);
            b.endy = max(stain.1, b.endy);
        }

        b
    }
}

fn take_n<T>(v: &[T], n: i32) -> Vec<Vec<&T>> {
    if v.is_empty() || n == 0 {
        return vec![vec![]];
    }

    let mut res = Vec::new();
    res.extend(take_n(&v[1..], n).iter().cloned());
    for c in take_n(&v[1..], n - 1) {
        let mut tmp = c.clone();
        tmp.push(&v[0]);
        res.push(tmp);
    }

    res
}

// From: http://stackoverflow.com/a/24500377/1025899
fn bin_coeff(n: i64, k: i64, m: i64) -> i64 {
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

fn main() {
    let (clean, stains) = read_input();
    let borders = Borders::from_stains(&stains);

    let mut res = 0;
    let rim = stains.iter()
        .filter(|s| {
            s.0 == borders.startx || s.0 == borders.endx || s.1 == borders.starty ||
            s.1 == borders.endy
        })
        .collect::<Vec<_>>();

    // take_n is slow :(
    for comb in take_n(&rim, clean) {
        if comb.len() == 0 {
            continue;
        }
        for stain in &comb {
            stain.2.set(true);
        }
        if Borders::from_stains(&stains) != borders {
            let still_to_clean = clean as i64 - comb.len() as i64;
            let internal_stains = (stains.len() - rim.len()) as i64;
            if internal_stains >= still_to_clean {
                res += bin_coeff(internal_stains, still_to_clean, MOD);
            }
            res %= MOD;
        }
        for stain in &comb {
            stain.2.set(false);
        }
    }

    println!("{}", res);
}
