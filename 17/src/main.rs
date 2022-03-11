use std::cmp::{max, min};
use std::fs;

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
    let mut step = 0;
    loop {
        step += 1;
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
    let example1 = "target area: x=20..30, y=-10..-5";

    let target = Target::new(input);
    target.print();

    let best_vx_min = f32::floor(f32::sqrt(2.0*target.x_min as f32))as i32;
    let best_vx_max = f32::ceil(f32::sqrt(2.0*target.x_max as f32)) as i32;

    let mut best_result = 0;

    for vx in best_vx_min..best_vx_max+1 {
        for vy in 0..(-target.y_min) {
            match calc_probe_result(vx, vy, &target) {
                ProbeResult::Hit {max_y} => best_result = max(best_result, max_y),
                _ => ()
            }
        }
    }

    println!("Best result: {}", best_result);
}
