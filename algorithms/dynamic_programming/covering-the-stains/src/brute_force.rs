use std::collections::btree_map::BTreeMap;
use std::cmp::max;
use std::cmp::min;
use std::io::prelude::*;
use std::io;
use std::cell::Cell;
use std::borrow::Borrow;

const MIN_S: i32 = 0;
const MAX_S: i32 = 100000;

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

fn combinations<T>(v: &[T]) -> Vec<Vec<&T>> {
    if v.is_empty() {
        return vec![vec![]];
    }

    let mut res = Vec::new();
    for comb in combinations(&v[1..]) {
        res.push(comb.clone());
        let mut tmp = comb.clone();
        tmp.push(&v[0]);
        res.push(tmp);
    }

    res
}

fn take_n<T>(v: &[T], n: i32) -> Vec<Vec<&T>> {
    if v.is_empty() || n == 0 {
        return vec![vec![]];
    }

    let mut res = Vec::new();
    res.extend(take_n(&v[1..], n).iter().cloned());
    for c in take_n(&v[1..], n-1) {
        let mut tmp = c.clone();
        tmp.push(&v[0]);
        res.push(tmp);
    }

    res
}

fn main() {
    let (clean, stains) = read_input();
    let borders = Borders::from_stains(&stains);

    let mut res = 0;

    for comb in take_n(&stains, clean) {
        if comb.len() as i32 != clean {
            continue;
        }
        for stain in &comb {
            stain.2.set(true);
        }
        if Borders::from_stains(&stains) != borders {
            // println!("> {:?}", comb);
            res += 1;
        }
        for stain in &comb {
            stain.2.set(false);
        }
    }

    println!("{}", res);
}
