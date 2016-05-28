use std::cell::Cell;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io;

struct Graph {
    edges: HashMap<i32, Vec<(i32, i32)>>,
}

impl Graph {
    fn new() -> Graph {
        Graph{edges: HashMap::new()}
    }

    fn add_edge(&mut self, node: i32, neighbor: i32, weight: i32) {
        let edges = self.edges.entry(node).or_insert(Vec::new());
        edges.push((neighbor, weight));
    }

    fn get_nodes(&self) -> Vec<&i32> {
        self.edges.keys().collect::<Vec<_>>()
    }

    fn get_edges(&self, node: i32) -> &Vec<(i32, i32)> {
        self.edges.get(&node).expect(&format!("Node {} doesn't exist", node))
    }
}

fn read_input() -> (Graph, Vec<i32>) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let content = lines
        .map(|line| line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut graph = Graph::new();
    for x in content[2..].iter() {
        graph.add_edge(x[0], x[1], x[2]);
        graph.add_edge(x[1], x[0], x[2]);
    }
    (graph, content[1].clone())
}

#[derive(PartialEq, Eq)]
struct HeapNode(i32, i32);

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Graph, source: i32, dest: &HashSet<i32>) -> Option<(i32, i32)> {
    let mut q = HashSet::new();
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    for &node in graph.get_nodes() {
        dist.insert(node, std::i32::MAX);
        heap.push(HeapNode(node, std::i32::MAX));
        q.insert(node);
    }

    dist.insert(source, 0);
    heap.push(HeapNode(source, 0));

    while !q.is_empty() {
        let mut node = -1;
        while !q.contains(&node) {
            if let Some(HeapNode(n, _)) = heap.pop(){
                node = n;
            }
        }
        if dest.contains(&node) {
            return Some((node, dist[&node]));
        }
        q.remove(&node);

        for &(neighbor, distance) in graph.get_edges(node) {
            let updated_dist = dist[&node] + distance;
            if updated_dist < dist[&neighbor] {
                dist.insert(neighbor, updated_dist);
                heap.push(HeapNode(neighbor, updated_dist));
            }
        }
    }

    None
}


fn search(distances: &HashMap<i32, HashMap<i32, i32>>, letters: &Vec<i32>, curr_node: i32, curr_dist: i32, best: &Cell<i32>) {
    if letters.is_empty() {
        if curr_dist < best.get() {
            best.set(curr_dist);
        }
    }

    for i in 0..letters.len() {
        let new_dist = curr_dist + distances[&curr_node][&letters[i]];
        if new_dist > best.get() {
            continue;
        }
        let mut tmp = Vec::new();
        tmp.extend_from_slice(&letters[0..i]);
        tmp.extend_from_slice(&letters[i+1..]);

        search(distances, &tmp, letters[i], new_dist, best);
    }
}

fn main() {
    let (graph, letters) = read_input();
    // let mut distances = HashMap::new();
    let mut letters_set = HashSet::new();

    for &l in &letters {
        letters_set.insert(l);
    }

    for &l in &letters {
        let mut new_set = letters_set.clone();
        let mut dist = 0;
        let mut curr = l;
        while new_set.len() > 1 {
            new_set.remove(&curr);
            let res = dijkstra(&graph, curr, &new_set).unwrap();
            curr = res.0;
            dist += res.1;
        }
        println!(">> {}", dist);
    }

    // for &l in &letters {
    //     distances.insert(l, dijkstra(&graph, l));
    // }

    // let best = Cell::new(std::i32::MAX);
    // for i in 0..letters.len() {
    //     let mut others = Vec::new();
    //     others.extend_from_slice(&letters[0..i]);
    //     others.extend_from_slice(&letters[i+1..]);
    //     search(&distances, &others, letters[i], 0, &best);
    // }
    // println!("{}", best.get());
}
