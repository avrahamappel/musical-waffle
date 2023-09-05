#![allow(clippy::wildcard_imports)]

use musical_waffle::*;

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
    day_five();
    day_six();
    day_seven();
    day_eight();
}
