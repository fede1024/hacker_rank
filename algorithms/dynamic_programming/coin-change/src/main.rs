use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io;

fn read_input() -> (i32, Vec<i32>) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.expect("Input not readable"));
    let content = lines.map(|line| {
            line.split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (content[0][0], content[1].clone())
}

fn calc(tot: i32, coins: &Vec<i32>, cache: &mut HashMap<i32, HashSet<Vec<i32>>>) -> HashSet<Vec<i32>> {
    let mut res = HashSet::new();

    if coins.iter().find(|&x| *x == tot).is_some() {
        let mut sol = Vec::new();
        sol.push(tot);
        res.insert(sol);
    }

    for sub in 1..(tot/2 + 1) {
        let mut sol1 = HashSet::new();
        let mut sol2 = HashSet::new();
        if cache.contains_key(&sub) {
            sol1 = cache.get(&sub).unwrap().clone();
        } else {
            let tmp = calc(sub, coins, cache);
            cache.insert(sub, tmp);
            sol1 = cache.get(&sub).unwrap().clone();
        }
        if cache.contains_key(&(tot-sub)) {
            sol2 = cache.get(&(tot-sub)).unwrap().clone();
        } else {
            let tmp = calc((tot-sub), coins, cache);
            cache.insert((tot-sub), tmp);
            sol2 = cache.get(&(tot-sub)).unwrap().clone();
        }
        for e1 in &sol1 {
            for e2 in &sol2 {
                let mut sol = Vec::new();
                sol.extend(e1);
                sol.extend(e2);
                println!("{:?} {:?}  > {:?}", e1, e2, sol);
                res.insert(sol);
            }
        }
    }

    res
}

fn main() {
    let (tot, coins) = read_input();
    let mut cache: HashMap<i32, HashSet<Vec<i32>>> = HashMap::new();
    let mut solutions = HashSet::new();

    for mut s in calc(tot, &coins, &mut cache) {
        s.sort();
        solutions.insert(s);
    }

    for s in solutions {
        println!("> {:?}", s);
    }
}
