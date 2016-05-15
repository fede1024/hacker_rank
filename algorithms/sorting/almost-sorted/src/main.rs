use std::io;
use std::io::prelude::*;

fn read_array() -> Vec<i32> {
    let stdin = io::stdin();
    let line = stdin.lock().lines().skip(1).next().unwrap().unwrap();
    line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()
}

fn find_indexes(array: &Vec<i32>) -> Option<(usize, usize)> {
    for f in 0..(array.len()-1) {
        if array[f] > array[f+1] {
            for l in ((f+1)..array.len()).rev() {
                if array[l] < array[l-1] {
                    return Some((f, l))
                }
            }
            break;
        }
    }
    return None
}

fn is_sorted(array: &Vec<i32>) -> bool {
    for i in 0..(array.len()-1) {
        if array[i] > array[i+1] {
            return false;
        }
    }
    true
}

fn try_swap(array: &Vec<i32>, (f, l): (usize, usize)) -> bool {
    let mut new = array.clone();
    new.swap(f, l);
    is_sorted(&new)
}

fn try_reverse(array: &Vec<i32>, (f, l): (usize, usize)) -> bool {
    let mut new = array.clone();
    new[f..(l+1)].reverse();
    is_sorted(&new)
}

fn main() {
    let array = read_array();

    if let Some(indexes) = find_indexes(&array) {
        if try_swap(&array, indexes) {
            println!("yes\nswap {} {}", indexes.0+1, indexes.1+1);
            return;
        }
        if try_reverse(&array, indexes) {
            println!("yes\nreverse {} {}", indexes.0+1, indexes.1+1);
            return;
        }
    }
    println!("no");
}
