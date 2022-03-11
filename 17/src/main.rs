use std::cmp::max;

extern crate regex;

use regex::Regex;

struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Probe {
    fn new(vx: i32, vy: i32) -> Self {
        Self { x: 0, y: 0, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vx = max(self.vx - 1, 0);
        self.vy -= 1;
    }
}

struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

enum ProbeResult {
    Hit { max_y: i32 },
    MissedLeft,
    MissedRight,
}

impl Target {
    fn new(used_text: &str) -> Self {
        let re = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
        let captures = re.captures_iter(used_text).next().unwrap();

        let x_min = captures[1].parse::<i32>().unwrap();
        let x_max = captures[2].parse::<i32>().unwrap();
        let y_min = captures[3].parse::<i32>().unwrap();
        let y_max = captures[4].parse::<i32>().unwrap();
        Self { x_min, x_max, y_min, y_max }
    }

    fn contains_probe(&self, probe: &Probe) -> bool {
        probe.x >= self.x_min && probe.x <= self.x_max && probe.y >= self.y_min && probe.y <= self.y_max
    }

    fn probe_missed(&self, probe: &Probe) -> Option<ProbeResult> {
        if probe.vy < 0 && probe.y <= self.y_min {
            if probe.x < self.x_min {
                Some(ProbeResult::MissedLeft)
            } else {
                Some(ProbeResult::MissedRight)
            }
        } else {
            None
        }
    }

    fn print(&self) {
        println!("target area: x={}..{}, y={}..{}", self.x_min, self.x_max, self.y_min, self.y_max);
    }
}

fn calc_probe_result(vx: i32, vy: i32, target: &Target) -> ProbeResult {
    let mut probe = Probe::new(vx, vy);
    let mut max_y = 0;
    loop {
        probe.step();
        max_y = max(max_y, probe.y);
        if target.contains_probe(&probe) {
            return ProbeResult::Hit { max_y };
        }
        match target.probe_missed(&probe) {
            Some(result) => return result,
            _ => ()
        }
    }
}

fn main() {
    let input = "target area: x=240..292, y=-90..-57";
    let _example1 = "target area: x=20..30, y=-10..-5";

    let target = Target::new(input);
    target.print();

    // To maximize our vertical height, we want to maximize the number of steps. This means we
    // should aim to have vx=0 when we cross the target. The change in x-distance to the target
    // over the trajectory are triangular numbers (e.g. 15, 10, 6, 3, 2, 1) so by inverting the
    // triangular formula, we can get a range of starting vx values that lead to us
    // having vx=0 within the target.
    let best_vx_min = f32::floor(f32::sqrt(2.0 * target.x_min as f32)) as i32;
    let best_vx_max = f32::ceil(f32::sqrt(2.0 * target.x_max as f32)) as i32;

    let mut best_result = 0;

    for vx in best_vx_min..best_vx_max + 1 {
        // Our vy trajectory is symmetrical, so we will have a later step where y=0. If our vy is
        // larger than the target is below 0, we'll miss it entirely, so its our upper bound.
        // Our lower bound could be improved, but it works well enough.
        for vy in 0..-target.y_min + 1 {
            match calc_probe_result(vx, vy, &target) {
                ProbeResult::Hit { max_y } => best_result = max(best_result, max_y),
                _ => ()
            }
        }
    }
    println!("Part 1: {}", best_result);

    // Our vx lower bound is our triangular number lower bound from above -- if we don't have at
    // least this speed, we'll never reach the target.
    // Our vx upper bound is the right edge of the target -- higher than this and we will miss the
    // target entirely.

    // Our vy lower bound is the bottom of the target, which we can achieve in a single step by
    // aiming directly at the target.
    // Our vy upper bound is the negative of the bottom of the target, same as above, because beyond
    // this we will skip the target in a single step following the second y=0 step.
    let mut solution_count = 0;
    for vx in best_vx_min..target.x_max + 1 {
        for vy in target.y_min..-target.y_min + 1 {
            match calc_probe_result(vx, vy, &target) {
                ProbeResult::Hit { max_y } => {
                    solution_count += 1;
                    println!("{},{}", vx, vy);
                }
                _ => ()
            }
        }
    }
    println!("Part 2: {}", solution_count);
}
