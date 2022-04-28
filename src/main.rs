use std::fs::File;
use std::io::{BufRead, BufReader};

mod sonar_sweep;

fn main() {
    day_one();
}

fn day_one() {
    let data = || {
        BufReader::new(File::open("data/sonar_sweep.txt").unwrap())
            .lines()
            .filter_map(Result::ok)
    };

    println!("Day 1: Sonar Sweep Part 1");
    println!("{}", sonar_sweep::sweep_increases(data()));
    println!("Day 1: Sonar Sweep Part 2");
    println!("{}", sonar_sweep::sweep_window_increases(data()));
}
