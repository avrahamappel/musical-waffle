use std::fs::File;
use std::io::{BufRead, BufReader};

mod dive;
mod sonar_sweep;

fn main() {
    day_one();
    day_two();
}

fn day_one() {
    let data = || get_file_lines("data/sonar_sweep.txt");

    println!("Day 1: Sonar Sweep Part 1");
    println!("{}", sonar_sweep::sweep_increases(data()));
    println!("Day 1: Sonar Sweep Part 2");
    println!("{}", sonar_sweep::sweep_window_increases(data()));
}

fn day_two() {
    let data = || get_file_lines("data/dive.txt");

    println!("Day 2: Dive! Part 1");
    println!("{}", dive::plot_course(data()));
    println!("Day 2: Dive! Part 2");
    println!("{}", dive::plot_aimed_course(data()));
}

fn get_file_lines(path: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(Result::ok)
}
