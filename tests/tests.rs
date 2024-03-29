use musical_waffle::*;

#[test]
fn day_one() {
    let data = include_str!("../data/sonar_sweep.txt");

    println!("Day 1: Sonar Sweep Part 1");
    assert_eq!(1390, sonar_sweep::sweep_increases(data.lines()));
    println!("Day 1: Sonar Sweep Part 2");
    assert_eq!(1457, sonar_sweep::sweep_window_increases(data.lines()));
}

#[test]
fn day_two() {
    let data = include_str!("../data/dive.txt");

    println!("Day 2: Dive! Part 1");
    assert_eq!(1_484_118, dive::plot_course(data.lines()));
    println!("Day 2: Dive! Part 2");
    assert_eq!(1_463_827_010, dive::plot_aimed_course(data.lines()));
}

#[test]
fn day_three() {
    let data = include_str!("../data/binary_diagnostic.txt");

    println!("Day 3: Binary Diagnostic Part 1");
    assert_eq!(
        3_309_596,
        binary_diagnostic::diagnose_power_consumption(data.lines())
    );
    println!("Day 3: Binary Diagnostic Part 2");
    assert_eq!(
        2_981_085,
        binary_diagnostic::diagnose_life_support(data.lines())
    );
}

#[test]
fn day_four() {
    let data = include_str!("../data/giant_squid.txt");

    println!("Day 4: Giant Squid Part 1");
    assert_eq!(Some(27027), giant_squid::first_winning_board_score(data));
    println!("Day 4: Giant Squid Part 2");
    assert_eq!(Some(36975), giant_squid::last_winning_board_score(data));
}

#[test]
fn day_five() {
    let data = include_str!("../data/hydrothermal_venture.txt");

    println!("Day 5: Hydrothermal Venture Part 1");
    assert_eq!(
        Ok(7085),
        hydrothermal_venture::right_angle_dangerous_points(data)
    );
    println!("Day 5: Hydrothermal Venture Part 2");
    assert_eq!(Ok(20271), hydrothermal_venture::all_dangerous_points(data));
}

#[test]
fn day_six() {
    let data = include_str!("../data/lanternfish.txt");

    println!("Day 6: Lanternfish Part 1");
    assert_eq!(Ok(360_268), lanternfish::simulate_fish(data, 80));
    println!("Day 6: Lanternfish Part 2");
    assert_eq!(Ok(1_632_146_183_902), lanternfish::simulate_fish(data, 256));
}

#[test]
fn day_seven() {
    let data = include_str!("../data/the_treachery_of_whales.txt");

    println!("Day 7: The Treachery of Whales Part 1");
    assert_eq!(
        Ok(355_989),
        the_treachery_of_whales::crab_alignment_constant(data)
    );
    println!("Day 7: The Treachery of Whales Part 2");
    assert_eq!(
        Ok(102_245_489),
        the_treachery_of_whales::crab_alignment_increasing(data)
    );
}

#[test]
fn day_eight() {
    let data = include_str!("../data/seven_segment_search.txt");

    println!("Day 8: Seven Segment Search Part 1");
    assert_eq!(521, seven_segment_search::unique_segment_total(data));
    println!("Day 8: Seven Segment Search Part 2");
    assert_eq!(1_016_804, seven_segment_search::solve_segments(data));
}
