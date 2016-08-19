/// https://www.hackerrank.com/challenges/insertion-sort/
///
/// Solution: sort each input using merge sort. In the merge phase,
/// if the current element of the right part is smaller than the current
/// element if the left part, it means that it has to be inserted first,
/// and all the remaining elements of the left part will be shifted by one
/// position.
///
///    if left[i] <= right[j]
///        add left[i] to sorted array
///        no shift required
///    else:
///        add right[j] to sorted array
///        all the len(left) - i elements remaining in left count as shifted
///

use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<Vec<i32>> {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .enumerate()
        .filter(|&(n, _)| n % 2 == 1)
        .map(|(_, line)| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<i32>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        });
    content.collect()
}

fn merge<T>((left, ls): (Vec<T>, usize), (right, rs): (Vec<T>, usize)) -> (Vec<T>, usize)
  where T: Clone + PartialOrd {
    let mut v = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j, mut shifts) = (0, 0, ls + rs);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            v.push(left[i].clone());
            i += 1;
        } else {
            v.push(right[j].clone());
            j += 1;
            shifts += left.len() - i; // This is the important bit :)
        }
    }
    while i < left.len() { v.push(left[i].clone()); i += 1 };
    while j < right.len() { v.push(right[j].clone()); j += 1 };
    (v, shifts)
}

fn merge_sort<T>(input: &[T]) -> (Vec<T>, usize)
  where T: Clone + PartialOrd {
    let l = input.len();
    if l > 1 {
        merge(merge_sort(&input[0..l/2]), merge_sort(&input[l/2..l]))
    } else {
        (input.to_vec(), 0)
    }
}

fn main() {
    for input in read_input() {
        println!("{}", merge_sort(input.as_slice()).1);
    }
}
