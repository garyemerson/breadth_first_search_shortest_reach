use std::io;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let num_queries = get_line().trim().parse::<i32>().unwrap();    
    for _ in 0..num_queries {
        let data = get_data();
        let distances = solve(data.0, data.1, data.2, data.3);
        println!("{}", distances.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "));
    }
}

fn solve(
    start_node: i32,
    num_nodes: i32,
    num_edges: i32,
    edges: HashMap<i32, HashSet<i32>>) -> Vec<i32>
{
    // (node, dist from start)
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((start_node, 0));
    let mut distances: HashMap<i32, i32> = HashMap::new();

    while queue.len() != 0 {
        let curr = queue.pop_front().unwrap();
        let new_nodes: Vec<i32> = match edges.get(&curr.0) {
            Some(neighbors) => {
                neighbors.clone().into_iter().filter(|n| !distances.contains_key(n)).collect()
            },
            None => {
                Vec::new()
            }
        };
        for n in new_nodes {
            distances.insert(n, curr.1 + 6);
            queue.push_back((n, curr.1 + 6));
        }
    }

    let mut soln = Vec::new();
    for n in 1..(num_nodes + 1) {
        if n != start_node {
            match distances.get(&n) {
                Some(d) => {
                    soln.push(*d);
                },
                None => {
                    soln.push(-1);
                }
            }
        }
    }
    soln
}

// return triple containing start node, num node, num edges, edges (as hashmap
// of node->hashset of nodes)
fn get_data() -> (i32, i32, i32, HashMap<i32, HashSet<i32>>) {
    let num_nodes_edges: Vec<i32> = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let num_nodes = num_nodes_edges[0];
    let num_edges = num_nodes_edges[1];
    let mut edges: HashMap<i32, HashSet<i32>> = HashMap::new();
    for _ in 0..num_edges {
        let nodes: Vec<i32> = get_line()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        edges.entry(nodes[0]).or_insert(HashSet::new()).insert(nodes[1]);
        edges.entry(nodes[1]).or_insert(HashSet::new()).insert(nodes[0]);
    }
    let start_node = get_line().trim().parse::<i32>().unwrap();

    (start_node, num_nodes, num_edges, edges)
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
