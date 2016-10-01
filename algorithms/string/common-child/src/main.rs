use std::cmp::max;
use std::io::prelude::*;
use std::io;

const MAX_SIZE: usize = 5000;

fn read_input() -> (Vec<u8>, Vec<u8>) {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .map(|line| line.expect("Input not readable").as_bytes().to_vec())
        .collect::<Vec<_>>();
    (content[0].clone(), content[1].clone())
}

struct Finder {
    cache: Vec<Option<u16>>,
}

impl Finder {
    fn new() -> Finder {
        Finder { cache: vec![None; MAX_SIZE*MAX_SIZE] }
    }

    fn find_common(&mut self, a: &[u8], b: &[u8], pos_a: usize, pos_b: usize) -> u16 {
        if pos_a == a.len() || pos_b == b.len() {
            return 0;
        }

        if let Some(c) = self.cache[pos_a * MAX_SIZE + pos_b] {
            return c;
        }

        let res = if a[pos_a] == b[pos_b] {
            1 + self.find_common(a, b, pos_a + 1, pos_b + 1)
        } else {
            max(self.find_common(a, b, pos_a + 1, pos_b),
                self.find_common(a, b, pos_a, pos_b + 1))
        };

        self.cache[pos_a * MAX_SIZE + pos_b] = Some(res);
        res
    }
}


fn main() {
    let (str_a, str_b) = read_input();

    let mut finder = Finder::new();
    let result = finder.find_common(&str_a, &str_b, 0, 0);
    println!("{}", result);
}
