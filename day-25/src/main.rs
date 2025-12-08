// Advent of Code - Day 25: Snowverload

const INPUT: &str = include_str!("./input.txt");

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Graph {
    connections: Vec<Vec<usize>>,
    total_nodes: usize,
}

impl Graph {
    fn from_input(input: &str) -> Self {
        let mut name_to_id = HashMap::new();
        let mut id_to_name = Vec::new();
        let mut next_id = 0;

        let mut get_id = |name: &str| -> usize {
            if let Some(&id) = name_to_id.get(name) {
                id
            } else {
                let id = next_id;
                name_to_id.insert(name.to_string(), id);
                id_to_name.push(name.to_string());
                next_id += 1;
                id
            }
        };

        // Collect edges and build name-to-id mapping
        let mut edges = Vec::new();
        for line in input.lines() {
            if let Some((comp, connected_comps)) = line.split_once(':') {
                let comp = comp.trim();
                let comp_id = get_id(comp);
                for connected_comp in connected_comps.split_whitespace() {
                    let connected_comp_id = get_id(connected_comp);
                    edges.push((comp_id, connected_comp_id));
                }
            }
        }
        let total_nodes = next_id;

        // Build list of connected components
        let mut connections = vec![Vec::new(); total_nodes];
        for (comp_a, comp_b) in edges {
            if !connections[comp_a].contains(&comp_b) {
                connections[comp_a].push(comp_b);
            }
            if !connections[comp_b].contains(&comp_a) {
                connections[comp_b].push(comp_a);
            }
        }

        Self {
            connections,
            total_nodes,
        }
    }

    fn disconnect_wire(&mut self, comp_a: usize, comp_b: usize) {
        // Remove component_b from component_a's connections
        if let Some(pos) = self.connections[comp_a].iter().position(|&x| x == comp_b) {
            self.connections[comp_a].swap_remove(pos);
        }
        // Remove component_a from component_b's connections
        if let Some(pos) = self.connections[comp_b].iter().position(|&x| x == comp_a) {
            self.connections[comp_b].swap_remove(pos);
        }
    }
}

fn find_bussiest_connection(graph: &Graph) -> (usize, usize) {
    // Stores edge counts: (smaller_id, larger_id) -> count
    let mut edge_counts: HashMap<(usize, usize), usize> = HashMap::new();
    for start_node in 0..graph.total_nodes {
        let mut parents = vec![None; graph.total_nodes];
        let mut queue = VecDeque::new();
        let mut visited = vec![false; graph.total_nodes];

        visited[start_node] = true;
        queue.push_back(start_node);
        while let Some(comp_a) = queue.pop_front() {
            for &comp_b in &graph.connections[comp_a] {
                if !visited[comp_b] {
                    visited[comp_b] = true;
                    parents[comp_b] = Some(comp_a);
                    queue.push_back(comp_b);
                }
            }
        }
        for target in 0..graph.total_nodes {
            if target == start_node {
                continue;
            }

            let mut curr = target;
            while let Some(parent) = parents[curr] {
                // Normalize edge representation for undirected graph
                let edge = if curr < parent {
                    (curr, parent)
                } else {
                    (parent, curr)
                };
                *edge_counts.entry(edge).or_insert(0) += 1;
                curr = parent;
            }
        }
    }

    edge_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(edge, _)| edge)
        .expect("Graph should have edges to count")
}

fn component_size(graph: &Graph, start_node: usize) -> usize {
    let mut visited = vec![false; graph.total_nodes];
    let mut queue = VecDeque::new();
    let mut count = 0;

    if start_node >= graph.total_nodes {
        return 0;
    }

    visited[start_node] = true;
    queue.push_back(start_node);

    while let Some(comp_a) = queue.pop_front() {
        count += 1;
        for &comp_b in &graph.connections[comp_a] {
            if !visited[comp_b] {
                visited[comp_b] = true;
                queue.push_back(comp_b);
            }
        }
    }
    count
}

fn solve(input: &str) -> usize {
    let mut apparatus = Graph::from_input(input);
    for _ in 0..3 {
        let edge = find_bussiest_connection(&apparatus);
        apparatus.disconnect_wire(edge.0, edge.1);
    }

    let size_a = component_size(&apparatus, 0);
    let size_b = apparatus.total_nodes - size_a;

    size_a * size_b
}

fn main() {
    let answer = solve(INPUT);
    println!("Answer: {answer:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let example_input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        let answer = crate::solve(example_input);
        assert_eq!(answer, 54);
    }
}
