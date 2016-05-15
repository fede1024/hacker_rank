use std::io;
use std::io::prelude::*;

fn is_kp(n: &i64) -> bool {
    let d = format!("{}", n).len();
    let sq = format!("{}", n*n);
    let r = &sq[sq.len()-d..];
    let l = &sq[..(sq.len()-d)];
    r.parse::<i64>().unwrap() + l.parse::<i64>().unwrap_or(0) == *n
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let start = lines.next().unwrap().parse::<i64>().unwrap();
    let end = lines.next().unwrap().parse::<i64>().unwrap();

    let kn = (start..(end+1)).filter(|n| is_kp(n)).collect::<Vec<_>>();

    if kn.is_empty() {
        println!("INVALID RANGE");
        return;
    }

    for n in kn {
        print!("{} ", n);
    }
}
