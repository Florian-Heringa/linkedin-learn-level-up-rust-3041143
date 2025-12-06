use std::collections::{HashMap, HashSet};

type Node = usize;
type Cost = usize;
type Distance = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &[(Node, Node, Cost)]) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_default();

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {

    let n_nodes = g.nodes.len();
    let mut dist: Vec<Distance> = vec![usize::MAX; n_nodes];
    let mut prev: Vec<Option<Node>> = vec![None; n_nodes];
    let mut q: HashSet<Node> = g.nodes.clone();

    dist[start] = 0;
    let mut current: Node = start;

    while current != goal {
        current = dist
            .iter()
            .enumerate()
            .min_by_key(|(_, x)| **x)
            .map(|(i, _)| i)
            .unwrap();
        q.remove(&current);

        let neighbors = g.edges.get(&current).unwrap();
        for (node, cost) in neighbors {
            let total_distance = dist[current] + cost;
            if total_distance < dist[current] {
                dist[*node] = total_distance;
                prev[*node] = Some(current);
            }
        }
    }

    // Find the shortest path
    let mut s = Vec::new();
    let total_cost = dist[goal];
    let mut n = goal;

    while n != start {
        s.push(n);
        n = prev[n].unwrap();
    }

    Some((s, total_cost))
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
