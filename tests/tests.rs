use musical_waffle::*;

#[test]
fn day_one() {
    let data = || include_str!("../data/sonar_sweep.txt").lines();

    println!("Day 1: Sonar Sweep Part 1");
    assert_eq!(1390, sonar_sweep::sweep_increases(data()));
    println!("Day 1: Sonar Sweep Part 2");
    assert_eq!(1457, sonar_sweep::sweep_window_increases(data()));
}
