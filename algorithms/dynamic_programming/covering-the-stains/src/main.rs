use std::collections::btree_map::BTreeMap;
use std::cmp::max;
use std::cmp::min;
use std::io::prelude::*;
use std::io;
use std::cell::Cell;

const MIN_S: i32 = 0;
const MAX_S: i32 = 100000;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Stain(i32, i32);

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
    let stains = content.map(|v| Stain(v[0], v[1])).collect();
    (clean, stains)
}

fn get_size(stains: &Vec<Stain>) -> i64 {
    let mut bottom = (MAX_S, MAX_S);
    let mut top = (MIN_S, MIN_S);

    for stain in stains {
        // if stain.2.get() {
        //     continue;
        // }
        bottom.0 = min(stain.0, bottom.0);
        bottom.1 = min(stain.1, bottom.1);
        top.0 = max(stain.0, top.0);
        top.1 = max(stain.1, top.1);
    }
    (top.0 - bottom.0) as i64 * (top.1 - bottom.1) as i64
}

fn find_neighbors<'a>(cols: &'a Vec<&Vec<Stain>>, rows: &'a Vec<&Vec<Stain>>, start_col: usize, end_col: usize, start_row: usize, end_row: usize) -> Vec<&'a Vec<Stain>> {
    let tmp = [cols[start_col], cols[end_col], rows[start_row], rows[end_row]];
    let mut res = Vec::new();

    for &n in tmp.iter() {
        if n.len() == 1 && tmp.iter().find(|&v| n != *v && v.iter().find(|&x| *x == n[0]).is_some()).is_some() {
            continue;
        }
        res.push(n);
    }

    res
}


fn main() {
    let (clean, stains) = read_input();
    let mut cols_map = BTreeMap::new();
    let mut rows_map = BTreeMap::new();
    let mut cols = Vec::new();
    let mut rows = Vec::new();

    for &s in &stains {
        let mut col = cols_map.entry(s.0).or_insert_with(Vec::new);
        col.push(s.clone());
        let mut row = rows_map.entry(s.1).or_insert_with(Vec::new);
        row.push(s.clone());
    }

    for (_, v) in &cols_map {
        cols.push(v);
    }

    for (_, v) in &rows_map {
        rows.push(v);
    }

    println!(">> {:?}", find_neighbors(&cols, &rows, 0, 2, 0, 2));
}
