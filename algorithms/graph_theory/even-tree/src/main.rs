use std::cell::Cell;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io;

struct Node {
    parent: i32,
    count: Cell<i32>,
    children: Vec<i32>,
}

impl Node {
    fn new() -> Node {
        Node { parent: -1, count: Cell::new(0), children: Vec::new() }
    }
}

struct Tree {
    nodes: HashMap<i32, Node>,
    root: i32,
}

impl Tree {
    fn new() -> Tree {
        Tree { nodes: HashMap::new(), root: -1 }
    }

    fn get_node(&self, id: i32) -> &Node {
        self.nodes.get(&id).expect("Node doesn't exit")
    }

    fn get_or_add_node(&mut self, id: i32) -> &mut Node {
        self.nodes.entry(id).or_insert(Node::new())
    }

    fn add_edge(&mut self, start: i32, end: i32) {
        self.get_or_add_node(start).children.push(end);
        self.get_or_add_node(end).parent = start;
    }
}

fn read_input() -> Tree {
    let stdin = io::stdin();
    let content = stdin.lock()
        .lines()
        .skip(1)
        .map(|line| {
            line.expect("Input not readable")
                .split(" ")
                .map(|n| n.parse::<i32>().expect("Can't parse integer"))
                .collect::<Vec<_>>()
        });

    let mut tree = Tree::new();
    for v in content {
        tree.add_edge(v[1], v[0]);
    }
    tree.root = *tree.nodes.iter().find(|&(_, n)| n.parent == -1).expect("This is not a tree").0;

    tree
}

fn calc_count(tree: &Tree, node_id: i32) -> i32 {
    let node = tree.get_node(node_id);
    let count = 1 + node.children.iter().fold(0, |acc, &c_id| acc + calc_count(tree, c_id));
    node.count.set(count);
    count
}

fn calc_cuts(tree: &Tree, node_id: i32, group_size: i32) -> i32 {
    let node = tree.get_node(node_id);
    if group_size % 2 == 1 {
        return 0;
    }

    let mut res = 0;
    for &c_id in &node.children {
        let child = tree.get_node(c_id);
        res += match child.count.get() % 2 {
            0 => 1 + calc_cuts(tree, c_id, child.count.get()),
            _ => calc_cuts(tree, c_id, group_size),
        }
    }
    res
}

fn main() {
    let tree = read_input();
    calc_count(&tree, tree.root);
    println!("{}", calc_cuts(&tree, tree.root, tree.get_node(tree.root).count.get()));
}
