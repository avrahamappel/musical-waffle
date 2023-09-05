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
