use std::cmp::max;
use std::collections::HashMap;
use std::fs;

enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Number(i32),
    // TODO Also add a value for Nothing? Might make addition simpler
}

impl SnailfishNumber {
    fn add(l: &SnailfishNumber, r: &SnailfishNumber) -> SnailfishNumber {
        let bl = Box::from(l.clone());
        let br = Box::from(r.clone());
        SnailfishNumber::Pair(bl, br)
    }
}

impl Clone for SnailfishNumber {
    fn clone(&self) -> Self {
        match self {
            SnailfishNumber::Pair(l, r) =>
                SnailfishNumber::Pair(*Box::from(l.clone()), *Box::from(r.clone())),
            SnailfishNumber::Number(i) => SnailfishNumber::Number(*i),
        }
    }
}

fn read_snailfish_number(text: &str) -> (SnailfishNumber, usize) {
    if text.starts_with("[") {
        let (left, read_left) = read_snailfish_number(&text[1..]);
        let (right, read_right) = read_snailfish_number(&text[(2 + read_left)..]);
        (SnailfishNumber::Pair(Box::from(left), Box::from(right)), read_left + read_right + 3)
    } else {
        (SnailfishNumber::Number(text[0..1].parse::<i32>().unwrap()), 1)
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SnailfishNumber::Pair(l, r) => write!(f, "[{},{}]", l, r),
            SnailfishNumber::Number(num) => write!(f, "{}", num),
        }
    }
}

enum ExplodeResult {
    None,
    Exploded(i32, i32),
}


impl SnailfishNumber {
    fn reduce(&mut self) {
        while self.reduce_step() {}
    }

    fn reduce_step(&mut self) -> bool {
        match self._check_for_explodes(0) {
            ExplodeResult::None => false,
            _ => {
                println!("Explode: {}", self);
                return true;
            }
        };

        if self._check_for_splits() {
            println!("Split  : {}", self);
            return true;
        }
        false
    }

    fn _check_for_explodes(&mut self, depth: i32) -> ExplodeResult {
        match self {
            SnailfishNumber::Pair(l, r) => {
                if depth == 4 {
                    // There are already 4 nested pairs above this one
                    let (lv, rv) = match (&**l, &**r) {
                        (SnailfishNumber::Number(lv), SnailfishNumber::Number(rv)) => (*lv, *rv),
                        _ => panic!("case with 5 nested pairs is not handled"),
                    };
                    *self = SnailfishNumber::Number(0);
                    ExplodeResult::Exploded(lv, rv)
                } else {
                    match l._check_for_explodes(depth + 1) {
                        ExplodeResult::Exploded(lv, mut rv) => {
                            rv = r._apply_exploded_values(rv, false);
                            return ExplodeResult::Exploded(lv, rv);
                        }
                        _ => ()
                    };

                    match r._check_for_explodes(depth + 1) {
                        ExplodeResult::Exploded(mut lv, rv) => {
                            lv = l._apply_exploded_values(lv, true);
                            return ExplodeResult::Exploded(lv, rv);
                        }
                        _ => ExplodeResult::None
                    }
                }
            }
            _ => ExplodeResult::None
        }
    }

    // If dir is true, exploded values are moving left (i.e. applied to nested-right values
    fn _apply_exploded_values(&mut self, value: i32, dir: bool) -> i32 {
        if value == 0 { return 0; }
        match self {
            SnailfishNumber::Number(i) => {
                *i += value;
                0
            }
            SnailfishNumber::Pair(l, r) => {
                (if dir { r } else { l })._apply_exploded_values(value, dir)
            }
        }
    }

    fn split(&mut self) {
        match self {
            SnailfishNumber::Number(i) => {
                let (l, r) = (*i / 2, ((*i) + 1) / 2);
                *self = SnailfishNumber::Pair(Box::from(SnailfishNumber::Number(l)), Box::from(SnailfishNumber::Number(r)));
            }
            _ => panic!("can only split a number"),
        }
    }

    fn _check_for_splits(&mut self) -> bool {
        match self {
            SnailfishNumber::Number(i) => {
                if *i >= 10 {
                    self.split();
                    return true;
                }
            }
            SnailfishNumber::Pair(l, r) => {
                return l._check_for_splits() || r._check_for_splits();
            }
        }
        false
    }

    fn magnitude(&self) -> i32 {
        match self {
            SnailfishNumber::Number(i) => *i,
            SnailfishNumber::Pair(l, r) => (3 * l.magnitude() + 2 * r.magnitude()),
        }
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let numbers: Vec<SnailfishNumber> = file_contents.lines().map(|line| {
        read_snailfish_number(line).0
    }).collect();

    for number in &numbers {
        println!("Number: {}", number);
    }
    let mut iter = numbers.iter();
    let val = &*iter.next().unwrap();
    let mut sum = val.clone();

    println!("Start  : {}", sum);

    iter.for_each(|next| {
        sum.reduce();
        // println!("Reduced: {}", sum);
        sum = SnailfishNumber::add(&sum, next);
        println!("Added  : {}", sum);
    });
    sum.reduce();
    // println!("Reduced: {}", sum);
    println!("Magnitude: {}", sum.magnitude());

    // Part 2
    let mut highest_magnitude = 0;

    let n = numbers.len();
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }

            let mut num = SnailfishNumber::add(&numbers[i], &numbers[j]);
            num.reduce();
            let magnitude = num.magnitude();
            highest_magnitude = max(highest_magnitude, magnitude);
        }
    }

    println!("Highest magnitude: {}", highest_magnitude);
}
