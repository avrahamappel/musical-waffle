use std::fs::File;
use std::io::{BufRead, BufReader};

mod sonar_sweep;

fn main() {
    let reader = BufReader::new(File::open("data/sonar_sweep.txt").unwrap());

    println!("Day 1: Sonar Sweep");
    println!(
        "{}",
        crate::sonar_sweep::sweep_increases(reader.lines().filter_map(Result::ok))
    );
}
