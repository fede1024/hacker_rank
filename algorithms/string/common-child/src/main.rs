use std::collections::HashMap;
use std::io::prelude::*;
use std::io;

fn read_input() -> (Vec<u8>, Vec<u8>) {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .map(|line| {
            line.expect("Input not readable").as_bytes().to_vec()
        })
        .collect::<Vec<_>>();
    (content[0].clone(), content[1].clone())
}

type Chain = Vec<Option<usize>>;

fn char_chain(s: &Vec<u8>, ref_char: u8) -> Chain {
    let mut last_pos = None;
    let mut chain = vec![None; s.len()];
    for i in (0..s.len()).rev() {
        if s[i] == ref_char {
            last_pos = Some(i);
        }
        chain[i] = last_pos;
    }
    chain
}

struct Finder {
    best: usize,
    best_map: HashMap<(usize, usize), usize>,
}

impl Finder {
    fn new() -> Finder {
        Finder { best: 0, best_map: HashMap::new() }
    }

    fn find_common(&mut self, a: &Vec<u8>, chains_b: &HashMap<u8, Chain>, pos_a: usize, pos_b: usize, len: usize) {
        if self.best < len {
            self.best = len;
        }

        if pos_a >= a.len() || pos_b >= a.len() {
            return;
        }

        match self.best_map.get(&(pos_a, pos_b)) {
            None => { self.best_map.insert((pos_a, pos_b), 0); },
            Some(&n) => {
                if len > n {
                    self.best_map.insert((pos_a, pos_b), len);
                } else {
                    return;
                }
            },
        };

        // match self.best_map.entry((pos_a, pos_b)) {
        //     Vacant(entry) => { entry.insert(len); },
        //     Occupied(entry) => { entry.insert(max(entry, len)); },
        // };

        // for _ in (0..len) {
        //     print!("  ");
        // }
        // println!("{} {}   {}", pos_a, pos_b, len);

        if let Some(next_b) = chains_b.get(&a[pos_a]).unwrap()[pos_b] {
            // for _ in (0..len) {
            //     print!("  ");
            // }
            // println!("found {} {}   {}", pos_a, pos_b, next_b);
            self.find_common(a, chains_b, pos_a + 1, next_b + 1, len + 1);
        }

        self.find_common(a, chains_b, pos_a + 1, pos_b, len);
    }
}

fn main() {
    let (str_a, str_b) = read_input();

    let chains = (65..91)
        .map(|c| (c, char_chain(&str_b, c)))
        .collect::<HashMap<_, _>>();

    // for (&c, chain) in &chains {
    //     println!("{} {:?}", c as char, chain);
    // }

    let mut finder = Finder::new();
    finder.find_common(&str_a, &chains, 0, 0, 0);
    println!("{}", finder.best);
}
