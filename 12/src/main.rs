use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;

fn add_link(map: &mut HashMap<String, HashSet<String>>, node1: &str, node2: &str) {
    if !map.contains_key(node1) {
        map.insert(node1.to_string(), HashSet::new());
    }
    map.get_mut(node1).unwrap().insert(node2.to_string());
}

fn add_both_links(map: &mut HashMap<String, HashSet<String>>, node1: &str, node2: &str) {
    add_link(map, node1, node2);
    add_link(map, node2, node1);
}

struct PartialPath {
    nodes: Vec<String>,
}

impl PartialPath {
    fn initial() -> PartialPath {
        return PartialPath {
            nodes: [String::from("start")].to_vec()
        };
    }

    fn visited(&self, node: &str) -> bool {
        return self.nodes.iter().any(|n| node.cmp(n) == Ordering::Equal);
    }

    fn extend_with(&self, node: &str) -> PartialPath {
        let mut nodes = self.nodes.clone();
        nodes.push(node.to_string());
        return PartialPath { nodes };
    }

    fn print_path(&self) {
        println!("{}", self.nodes.join(","));
    }
}

fn main() {
    let mut map = HashMap::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let vec: Vec<&str> = line.splitn(2, "-").collect();
        let (node1, node2) = (vec[0], vec[1]);
        add_both_links(&mut map, node1, node2);
    }

    let mut paths_found = 0;
    let mut to_check = vec![PartialPath::initial()];

    while !to_check.is_empty() {
        let path_to_check = to_check.pop().unwrap();
        let last_node = path_to_check.nodes.last().unwrap();
        let possible_next_nodes = &map[last_node];
        for possible_next_node in possible_next_nodes.iter() {
            if *possible_next_node == "start" { continue; }
            if possible_next_node.chars().all(|c| c.is_lowercase()) &&
                path_to_check.visited(possible_next_node) {
                continue;
            }
            let new_path = path_to_check.extend_with(possible_next_node);
            if *possible_next_node == "end" {
                new_path.print_path();
                paths_found += 1;
            } else {
                to_check.push(new_path);
            }
        }
    }

    println!("{}", paths_found);
}
