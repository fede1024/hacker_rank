use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io;

fn read_input() -> (i32, HashSet<i32>) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.expect("Input not readable"));
    let content = lines.map(|line| {
            line.split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (content[0][0], content[1].iter().cloned().collect())
}

fn calc(tot: i32, coins: &HashSet<i32>, cache: &mut HashMap<i32, HashSet<Vec<i32>>>) -> HashSet<Vec<i32>> {
    let mut res = HashSet::new();

    if let Some(s) = cache.get(&tot) {
        return s.clone();
    }

    if coins.contains(&tot) {
        let mut sol = Vec::new();
        sol.push(tot);
        res.insert(sol);
    }

    for sub in 1..(tot/2 + 1) {
        let subset1 = calc(sub, coins, cache);
        if subset1.is_empty() {
            continue;
        }
        let subset2 = calc(tot-sub, coins, cache);
        for e1 in &subset1 {
            for e2 in &subset2 {
                let mut sol = Vec::new();
                sol.extend(e1);
                sol.extend(e2);
                sol.sort();
                res.insert(sol);
            }
        }
    }

    cache.insert(tot, res.clone());
    res
}

fn main() {
    let (tot, coins) = read_input();
    let mut cache: HashMap<i32, HashSet<Vec<i32>>> = HashMap::new();

    println!("{}", calc(tot, &coins, &mut cache).len());
}
