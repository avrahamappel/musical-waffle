use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

mod binary_diagnostic;
mod dive;
mod giant_squid;
mod hydrothermal_venture;
mod sonar_sweep;

fn get_file_lines(path: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(Result::ok)
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

fn day_three() {
    let data = || get_file_lines("data/binary_diagnostic.txt");

    println!("Day 3: Binary Diagnostic Part 1");
    println!("{}", binary_diagnostic::diagnose_power_consumption(data()));
    println!("Day 3: Binary Diagnostic Part 2");
    println!("{}", binary_diagnostic::diagnose_life_support(data()));
}

fn day_four() {
    let data = read_to_string("data/giant_squid.txt").unwrap();

    println!("Day 4: Giant Squid Part 1");
    println!("{}", giant_squid::first_winning_board_score(&data).unwrap());
    println!("Day 4: Giant Squid Part 2");
    println!("{}", giant_squid::last_winning_board_score(&data).unwrap());
}

fn day_five() {
    let data = include_str!("../data/hydrothermal_venture.txt");

    println!("Day 5: Hydrothermal Venture");
    println!(
        "{}",
        hydrothermal_venture::right_angle_dangerous_points(data).unwrap()
    );
}

fn main() {
    day_one();
    day_two();
    day_three();
    day_four();
    day_five();
}
