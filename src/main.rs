#![allow(clippy::wildcard_imports)]

use musical_waffle::*;

fn main() {
    // let data = include_str!("../data/seven_segment_search.txt");
    let data =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    println!("Day 8: Seven Segment Search Part 2");
    println!("{:?}", seven_segment_search::solve_segments(data));
}
