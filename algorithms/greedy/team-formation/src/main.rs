use std::cell::Cell;
use std::collections::BTreeMap;
use std::cmp::min;
use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<BTreeMap<i32, Cell<i32>>> {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .map(|line| {
            line.expect("Input not readable")
                .split(" ")
                .skip(1)
                .map(|n| n.parse::<i32>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        });
    let mut test_cases = Vec::new();
    for line in content {
        let mut counters = BTreeMap::new();
        for n in line {
            let count = counters.entry(n).or_insert(Cell::new(0));
            count.set(count.get() + 1);
        }
        test_cases.push(counters);
    }

    test_cases
}

fn calc_seq(counters: &BTreeMap<i32, Cell<i32>>, curr: i32, curr_counter: &Cell<i32>) -> i32 {
    curr_counter.set(curr_counter.get() - 1);

    match counters.get(&(curr + 1)) {
        None => 1,
        Some(next_counter) => {
            if next_counter.get() <= curr_counter.get() {
                1
            } else {
                1 + calc_seq(counters, curr + 1, next_counter)
            }
        }
    }
}

fn calc(mut counters: BTreeMap<i32, Cell<i32>>) -> i32 {
    if counters.is_empty() {
        return 0;
    }

    let mut res = std::i32::MAX;
    while !counters.is_empty() {
        if counters.iter().nth(0).unwrap().1.get() == 0 {
            let &k = counters.iter().nth(0).unwrap().0;
            counters.remove(&k);
            continue;
        }
        let (&k, v) = counters.iter().nth(0).unwrap();
        res = min(res, calc_seq(&counters, k, v));
    }
    res
}

fn main() {
    for test_case in read_input() {
        println!("{:?}", calc(test_case));
    }
}
