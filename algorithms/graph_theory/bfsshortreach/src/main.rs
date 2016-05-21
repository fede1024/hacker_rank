use std::collections::HashMap;
use std::collections::LinkedList;
use std::io::prelude::*;
use std::io;

struct Graph {
    edges: HashMap<i32, Vec<i32>>,
    n_nodes: i32,
    start: i32,
}

impl Graph {
    fn new(n_nodes: i32) -> Graph {
        Graph{edges: HashMap::new(), start: -1, n_nodes: n_nodes}
    }

    fn add_edge(&mut self, node: i32, neighbor: i32) {
        let node_edges = self.edges.entry(node).or_insert_with(Vec::new);
        node_edges.push(neighbor);
    }

    fn get_edges(&self, node: i32) -> &Vec<i32> {
        self.edges.get(&node).expect(&format!("Node {} doesn't exist", node))
    }
}

fn read_graphs() -> Vec<Graph> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.expect("Input not readable"));
    let mut graphs = Vec::new();
    let mut content = lines
        .map(|line| line.split(' ').filter(|n| !n.is_empty())
             .map(|n| n.parse::<i32>().unwrap())
             .collect::<Vec<_>>()
    );
    let n_graphs = content.next().unwrap()[0];
    for _ in 0..n_graphs {
        let line = content.next().unwrap();
        let mut graph = Graph::new(line[0]);
        for _ in 0..line[1] {
            let line = content.next().unwrap();
            graph.add_edge(line[0], line[1]);
            graph.add_edge(line[1], line[0]);
        }
        if graph.n_nodes > 0 {
            let start = content.next().unwrap()[0];
            graph.start = start;
            graph.add_edge(start, start);
        }
        graphs.push(graph);
    }
    graphs
}

fn bfs(graph: &Graph) -> HashMap<i32, i32> {
    let mut dist = HashMap::new();
    let mut q = LinkedList::new();

    q.push_back(graph.start);
    dist.insert(graph.start, 0);

    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        let node_dist = dist[&node];
        for &n in graph.get_edges(node) {
            if dist.contains_key(&n) {
                continue;
            }
            dist.insert(n, node_dist + 1);
            q.push_back(n);
        }
    }

    dist
}

fn main() {
    let graphs = read_graphs();

    for graph in &graphs {
        if graph.n_nodes == 0 {
            println!("");
            continue;
        }
        let dist = bfs(graph);
        for n in (1..(graph.n_nodes + 1)).filter(|&x| x != graph.start) {
            let d = match dist.get(&n) {
                Some(x) => x * 6,
                None    => -1,
            };
            print!("{} ", d);
        }
        println!("");
    }
}
