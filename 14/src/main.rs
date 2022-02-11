use std::cmp::{max, min};
use std::{fmt, fs};
use std::collections::{HashMap, HashSet};

struct PairInsertionRules {
    rules: HashMap<String, String>,
}

impl PairInsertionRules {
    fn result(&self, pair: &str) -> Option<&String> {
        self.rules.get(pair)
    }
}

trait Polymer {
    fn apply(&mut self, rules: &PairInsertionRules);
    fn count_char(&self, c: char) -> usize;
    fn score(&self) -> usize;
}

impl Polymer for String {
    fn apply(&mut self, rules: &PairInsertionRules) {
        let mut inserts = Vec::new();
        for i in 0..(self.len() - 1) {
            inserts.push(rules.result(&self[i..(i + 2)]).unwrap());
        }

        for i in 0..(inserts.len()) {
            self.insert(2 * i + 1, inserts[i].chars().next().unwrap());
        }
    }

    fn count_char(&self, v: char) -> usize {
        self.chars().filter(|c| *c == v).count()
    }

    fn score(&self) -> usize {
        let mut chars = HashSet::new();
        self.chars().for_each(|c| { chars.insert(c); });

        let mut highest = 0;
        let mut lowest = 1000000000;
        for c in chars {
            let score = self.count_char(c);
            highest = max(highest, score);
            lowest = min(lowest, score);
        }
        return highest - lowest;
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut lines = file_contents.lines();
    let mut polymer = lines.next().unwrap().to_string();
    let mut rules = PairInsertionRules { rules: HashMap::new() };

    lines.next(); // Ignore empty line

    for line in lines {
        let vec: Vec<&str> = line.splitn(2, " -> ").collect();
        let (pair, result) = (vec[0].to_string(), vec[1].to_string());
        rules.rules.insert(pair, result);
    }

    println!("Template:     {}", polymer);
    for step in 1..11 {
        polymer.apply(&rules);
        println!("After step {}: {}", step, polymer);
    }

    println!("{}", polymer.score());
}
