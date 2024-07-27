use priority_queue::PriorityQueue;
use sqlx::FromRow;
use std::collections::{HashMap, HashSet};

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new();
        let mut priority_queue = PriorityQueue::new();
        let mut visited = HashSet::new();

        distances.insert(from_node_id, 0);
        priority_queue.push(from_node_id, 0);

        while let Some((current_node, _)) = priority_queue.pop() {
            if current_node == to_node_id {
                break;
            }

            if !visited.insert(current_node) {
                continue;
            }

            if let Some(edges) = self.edges.get(&current_node) {
                for edge in edges {
                    let next_node = edge.node_b_id;
                    let new_distance = distances.get(&current_node).unwrap() + edge.weight;

                    if new_distance < *distances.get(&next_node).unwrap_or(&i32::MAX) {
                        distances.insert(next_node, new_distance);
                        priority_queue.push(next_node, -new_distance); // Min-heap by using negative distance
                    }
                }
            }
        }

        *distances.get(&to_node_id).unwrap_or(&i32::MAX)
    }
}
