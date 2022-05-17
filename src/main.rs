#![feature(iter_intersperse)]
#![feature(test)]

use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

mod binary_diagnostic;
mod dive;
mod giant_squid;
mod hydrothermal_venture;
mod lanternfish;
mod seven_segment_search;
mod sonar_sweep;
mod the_treachery_of_whales;
mod utils;

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

    println!("Day 5: Hydrothermal Venture Part 1");
    println!(
        "{}",
        hydrothermal_venture::right_angle_dangerous_points(data).unwrap()
    );
    println!("Day 5: Hydrothermal Venture Part 2");
    println!(
        "{}",
        hydrothermal_venture::all_dangerous_points(data).unwrap()
    );
}

fn day_six() {
    let data = include_str!("../data/lanternfish.txt");

    println!("Day 6: Lanternfish Part 1");
    println!("{}", lanternfish::simulate_fish(data, 80).unwrap());
    println!("Day 6: Lanternfish Part 2");
    println!("{}", lanternfish::simulate_fish(data, 256).unwrap());
}

fn day_seven() {
    let data = include_str!("../data/the_treachery_of_whales.txt");

    println!("Day 7: The Treachery of Whales Part 1");
    println!(
        "{}",
        the_treachery_of_whales::crab_alignment_constant(data).unwrap()
    );
    println!("Day 7: The Treachery of Whales Part 2");
    println!(
        "{}",
        the_treachery_of_whales::crab_alignment_increasing(data).unwrap()
    );
}

fn day_eight() {
    let data = include_str!("../data/seven_segment_search.txt");

    println!("Day 8: Seven Segment Search");
    println!("{}", seven_segment_search::unique_segment_total(data));
}

fn main() {
    day_one();
    day_two();
    day_three();
    day_four();
    day_five();
    day_six();
    day_seven();
    day_eight();
}
