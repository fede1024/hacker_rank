use std::collections::HashMap;
use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<String> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().skip(1).map(|x| x.expect("Input not readable"));
    lines.collect::<Vec<_>>()
}


fn positions(length: usize) -> Vec<(usize, usize, usize, usize)> {
    let mut pos = Vec::new();

    for l in 1..length {
        for i in 0..(length-l) {
            for j in (i+1)..(length-l+1) {
                pos.push((i, i+l, j, j+l));
            }
        }
    }

    pos
}

fn counter(s: &[u8]) -> HashMap<u8, u8> {
    let mut map = HashMap::new();

    for &b in s { *map.entry(b).or_insert(0) += 1 }

    map
}

fn main() {
    for line in read_input() {
        let bytes = line.as_bytes();
        let tot = positions(bytes.len()).iter()
            .filter(|&&(a, b, c, d)| counter(&bytes[a..b]) == counter(&bytes[c..d]))
            .count();
        println!("{}", tot);
    }
}
