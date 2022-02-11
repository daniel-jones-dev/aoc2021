use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::{fmt, fs};

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
    used_double_visit: bool,
}

impl PartialPath {
    fn initial() -> PartialPath {
        return PartialPath {
            nodes: [String::from("start")].to_vec(),
            used_double_visit: false,
        };
    }

    fn visited(&self, node: &str) -> bool {
        self.nodes.iter().any(|n| node.cmp(n) == Ordering::Equal)
    }

    fn visited_pt2(&self, node: &str) -> bool {
        self.used_double_visit && self.visited(node)
    }

    fn extend_with(&self, node: &str) -> PartialPath {
        let node_is_lowercase = node.chars().all(|c| c.is_lowercase());
        let used_double_visit = self.used_double_visit || (node_is_lowercase && self.visited(node));
        let mut nodes = self.nodes.clone();
        nodes.push(node.to_string());
        PartialPath { nodes, used_double_visit }
    }
}

impl fmt::Display for PartialPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.nodes.join(","), if self.used_double_visit { " *" } else { "" })
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
            if *possible_next_node == "start"
                || (possible_next_node.chars().all(|c| c.is_lowercase()) &&
                    path_to_check.visited_pt2(possible_next_node)) {
                continue;
            }
            let new_path = path_to_check.extend_with(possible_next_node);
            if *possible_next_node == "end" {
                println!("{}", new_path);
                paths_found += 1;
            } else {
                to_check.push(new_path);
            }
        }
    }

    println!("{}", paths_found);
}
