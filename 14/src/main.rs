use std::cmp::{max, min};
use std::{fmt, fs};
use std::collections::{HashMap, HashSet};

struct PairInsertionRules {
    rules: HashMap<(char, char), char>,
}

impl PairInsertionRules {
    fn result(&self, pair: (char, char)) -> char {
        *self.rules.get(&pair).unwrap()
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
            let pair = (self.chars().nth(i).unwrap(), self.chars().nth(i + 1).unwrap());
            inserts.push(rules.result(pair));
        }

        for i in 0..(inserts.len()) {
            self.insert(2 * i + 1, inserts[i]);
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
        let pair = (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap());
        let result = result.chars().nth(0).unwrap();
        rules.rules.insert(pair, result);
    }

    let mut polymer_part1 = polymer.clone();
    // Part 1
    println!("Template:     {}", polymer_part1);
    for step in 1..11 {
        polymer_part1.apply(&rules);
        println!("After step {}: {}", step, polymer_part1);
    }
    println!("{}", polymer_part1.score());

    // Part 2
    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for i in 0..(polymer.len() - 1) {
        let (char1, char2) = (polymer.chars().nth(i).unwrap(), polymer.chars().nth(i + 1).unwrap());
        pair_counts.entry((char1, char2)).and_modify(|v| *v += 1).or_insert(1);
    }

    for step in 1..41 {
        let old_pair_counts = pair_counts.clone();
        pair_counts.clear();
        for ((char1, char2), count) in old_pair_counts.iter() {
            let result = rules.result((*char1, *char2));
            let pair1 = (*char1, result);
            pair_counts.entry(pair1).and_modify(|c| *c += count).or_insert(*count);

            let pair2 = (result, *char2);
            pair_counts.entry(pair2).and_modify(|c| *c += count).or_insert(*count);
        }
    }
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for (pair, count) in pair_counts.iter() {
        char_counts.entry(pair.0).and_modify(|c| *c += count).or_insert(*count);
    }
    // Add the last character in the chain on at the end
    let last_polymer_char = polymer.chars().last().unwrap();
    char_counts.entry(last_polymer_char).and_modify(|count| *count += 1);

    let mut highest = 0;
    let mut lowest = 1000000000000000;
    for (_, count) in char_counts.iter() {
        highest = max(highest, *count);
        lowest = min(lowest, *count);
    }
    println!("{}", highest - lowest);
}
