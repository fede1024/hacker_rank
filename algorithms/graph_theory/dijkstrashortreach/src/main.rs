use std::cmp::min;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io;

struct Graph {
    edges: HashMap<i32, Vec<(i32, i32)>>,
    n_nodes: i32,
    start: i32,
}

impl Graph {
    fn new(n_nodes: i32) -> Graph {
        Graph {
            edges: HashMap::new(),
            start: -1,
            n_nodes: n_nodes,
        }
    }

    fn get_nodes(&self) -> Vec<&i32> {
        self.edges.keys().collect::<Vec<_>>()
    }

    fn add_edge(&mut self, node: i32, neighbor: i32, weight: i32) {
        let node_edges = self.edges.entry(node).or_insert_with(Vec::new);
        node_edges.push((neighbor, weight));
    }

    fn get_edges(&self, node: i32) -> &Vec<(i32, i32)> {
        self.edges.get(&node).expect(&format!("Node {} doesn't exist", node))
    }
}

fn read_graphs() -> Vec<Graph> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.expect("Input not readable"));
    let mut graphs = Vec::new();
    let mut content = lines.map(|line| {
        line.split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let n_graphs = content.next().unwrap()[0];
    for _ in 0..n_graphs {
        let line = content.next().unwrap();
        let mut graph = Graph::new(line[0]);
        let mut min_weight = HashMap::new();
        for _ in 0..line[1] {
            let line = content.next().unwrap();
            let min = match min_weight.get(&(line[0], line[1])) {
                Some(&n) => min(n, line[2]),
                None => line[2],
            };
            min_weight.insert((line[0], line[1]), min);
        }
        for ((n1, n2), w) in min_weight {
            graph.add_edge(n1, n2, w);
            graph.add_edge(n2, n1, w);
        }
        graph.start = content.next().unwrap()[0];
        graphs.push(graph);
    }
    graphs
}

#[derive(PartialEq, Eq, Debug)]
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

fn dijkstra(graph: &Graph) -> HashMap<i32, i32> {
    let mut q = HashSet::new();
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    for &node in graph.get_nodes() {
        dist.insert(node, std::i32::MAX);
        heap.push(HeapNode(node, std::i32::MAX));
        q.insert(node);
    }

    dist.insert(graph.start, 0);
    heap.push(HeapNode(graph.start, 0));

    while !q.is_empty() {
        let mut node = -1;
        while !q.contains(&node) {
            if let Some(HeapNode(n, _)) = heap.pop() {
                node = n;
            }
        }
        q.remove(&node);
        if dist[&node] == std::i32::MAX {
            break;
        }

        for &(neighbor, distance) in graph.get_edges(node) {
            let updated_dist = dist[&node] + distance;
            if updated_dist < dist[&neighbor] {
                dist.insert(neighbor, updated_dist);
                heap.push(HeapNode(neighbor, updated_dist));
            }
        }
    }
    dist
}

fn main() {
    for graph in &read_graphs() {
        let dist = dijkstra(graph);
        for n in (1..(graph.n_nodes + 1)).filter(|&x| x != graph.start) {
            let &d = dist.get(&n).unwrap_or(&std::i32::MAX);
            if d == std::i32::MAX {
                print!("-1 ");
            } else {
                print!("{} ", d);
            }
        }
        println!("");
    }
}
