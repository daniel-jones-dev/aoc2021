use std::cmp::{max, min};
use std::{fmt, fs};
use std::collections::{HashMap, HashSet};
use std::hash::Hasher;
use std::rc::Rc;
use priority_queue::PriorityQueue;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coord(usize, usize);

struct Grid {
    values: Vec<Vec<i32>>,
}

fn between(low: usize, val: usize, high: usize) -> bool {
    return low <= val && val <= high;
}

impl Grid {
    fn from(filepath: &str) -> Grid {
        let file_contents = fs::read_to_string(filepath).unwrap();
        let lines: Vec<String> = file_contents.lines().map(|f| f.to_string()).collect();
        let mut values = vec![Vec::new(); lines.len()];
        for i in 0..lines.len() {
            values[i] = lines[i].chars().map(|char| char.to_digit(10).unwrap() as i32).collect();
        }
        Grid { values }
    }

    fn ys(&self) -> usize { return self.values.len(); }

    fn xs(&self) -> usize { return self.values.first().unwrap().len(); }

    fn get(&self, coord: Coord) -> i32 {
        return *self.values.get(coord.1).unwrap().get(coord.0).unwrap();
    }

    fn start(&self) -> Coord { Coord(0, 0) }

    fn goal(&self) -> Coord {
        Coord(self.xs() - 1, self.ys() - 1)
    }

    fn h(&self, coord: Coord) -> i32 {
        return 1 * (
            (self.ys() - 1 - coord.1) as i32 +
                (self.xs() - 1 - coord.0) as i32
        );
    }

    fn neighbours(&self, coord: Coord) -> Vec<Coord> {
        let mut result = Vec::new();
        for (yd, xd) in [(0, 1), (1, 2), (2, 1), (1, 0)] {
            if !between(1, coord.1 + yd, self.ys()) || !between(1, coord.0 + xd, self.xs()) {
                continue;
            }
            let new_coord = Coord(coord.0 + xd - 1, coord.1 + yd - 1);
            result.push(new_coord);
        }
        result
    }

    fn print(&self) {
        for y in 0..(self.ys()) {
            println!("{}", self.values[y].iter().fold(
                String::new(),
                |mut acc, ele| {
                    acc + ele.to_string().as_str()
                },
            ));
        }
        println!();
    }
}


#[derive(Hash, PartialEq, Eq)]
struct Node {
    prev: Option<Rc<Node>>,
    coord: Coord,
    g: i32,
    f: i32,
}

impl Node {
    fn init(grid: &Grid) -> Node {
        Node { prev: None, coord: grid.start(), g: 0, f: grid.h(grid.start()) }
    }

    fn f(&self, grid: &Grid) -> i32 {
        self.g + grid.h(self.coord)
    }

    fn to_string(&self) -> String {
        (match &self.prev {
            Some(node) => node.to_string() + ", ",
            None => String::new(),
        }) + &format!("({}, {})", self.coord.0, self.coord.1)
    }
}

fn is_better_cost(best_node: &Option<Rc<Node>>, g:i32) -> bool {
    best_node.is_none() || best_node.as_ref().unwrap().g > g
}

struct AStar {
    explored: HashMap<Coord, Rc<Node>>,
    open_set: PriorityQueue<Rc<Node>, i32>,
}

impl AStar {
    fn new(grid: &Grid) -> AStar {
        let mut result = AStar {
            explored: HashMap::new(),
            open_set: PriorityQueue::new(),
        };

        let start_node = Rc::new(Node::init(&grid));
        result.push_node(&start_node);
        result
    }

    fn push_node(&mut self, node: &Rc<Node>) {
        let f = node.f;
        self.open_set.push(node.clone(), -f);
        self.explored.insert(node.coord, node.clone());
    }

    fn better_cost(&self, coord: Coord, g: i32) -> bool {
        !self.explored.contains_key(&coord) || g < self.explored.get(&coord).unwrap().g
    }
}

fn main() {
    let grid = Grid::from("input.txt");
    grid.print();

    let mut best_node = None;
    let mut astar = AStar::new(&grid);

    while !astar.open_set.is_empty() {
        let current_node = astar.open_set.pop().unwrap().0;

        if current_node.coord == grid.goal() {
            if is_better_cost(&best_node, current_node.g) {
                best_node = Some(current_node.clone());
                println!("Goal reached: {}", current_node.to_string());
                println!("Cost: {}", current_node.g);
            }
        }

        for neighbour in grid.neighbours(current_node.coord) {
            let g = current_node.g + grid.get(neighbour);

            if astar.better_cost(neighbour, g) {
                // Improved score
                let f = g + grid.h(neighbour);
                let new_node = Rc::from(Node { prev: Some(current_node.clone()), coord: neighbour, g, f });
                astar.push_node(&new_node);
            }
        }
    }

    let best_node = best_node.unwrap();
    println!("Best cost: {} with {}", best_node.g, best_node.to_string());
}
