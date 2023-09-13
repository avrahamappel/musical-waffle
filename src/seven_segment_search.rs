use std::cmp::Reverse;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// Corresponds to digits 1, 4, 7, and 8
const DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS: [usize; 4] = [2, 4, 3, 7];

#[repr(usize)]
#[derive(Clone, Copy)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Display for Wire {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::A => 'a',
                Self::B => 'b',
                Self::C => 'c',
                Self::D => 'd',
                Self::E => 'e',
                Self::F => 'f',
                Self::G => 'g',
            }
        )
    }
}

impl Wire {
    fn parse(char: char) -> Option<Self> {
        match char {
            'a' => Some(Self::A),
            'b' => Some(Self::B),
            'c' => Some(Self::C),
            'd' => Some(Self::D),
            'e' => Some(Self::E),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Signal {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(match self {
            Signal::Top => "T",
            Signal::TopLeft => "TL",
            Signal::TopRight => "TR",
            Signal::Middle => "M",
            Signal::BottomLeft => "BL",
            Signal::BottomRight => "BR",
            Signal::Bottom => "B",
        })
    }
}

fn digit_patterns() -> [(Digit, Vec<Signal>); 10] {
    [
        (
            Digit::Zero,
            vec![
                Signal::Top,
                Signal::TopLeft,
                Signal::TopRight,
                Signal::BottomLeft,
                Signal::BottomRight,
                Signal::Bottom,
            ],
        ),
        (Digit::One, vec![Signal::TopRight, Signal::BottomRight]),
        (
            Digit::Two,
            vec![
                Signal::Top,
                Signal::TopRight,
                Signal::Middle,
                Signal::BottomLeft,
                Signal::Bottom,
            ],
        ),
        (
            Digit::Three,
            vec![
                Signal::Top,
                Signal::TopRight,
                Signal::Middle,
                Signal::BottomRight,
                Signal::Bottom,
            ],
        ),
        (
            Digit::Four,
            vec![
                Signal::TopLeft,
                Signal::TopRight,
                Signal::Middle,
                Signal::BottomRight,
            ],
        ),
        (
            Digit::Five,
            vec![
                Signal::Top,
                Signal::TopLeft,
                Signal::Middle,
                Signal::BottomRight,
                Signal::Bottom,
            ],
        ),
        (
            Digit::Six,
            vec![
                Signal::Top,
                Signal::TopLeft,
                Signal::Middle,
                Signal::BottomLeft,
                Signal::BottomRight,
                Signal::Bottom,
            ],
        ),
        (
            Digit::Seven,
            vec![Signal::Top, Signal::TopRight, Signal::BottomRight],
        ),
        (
            Digit::Eight,
            vec![
                Signal::Top,
                Signal::TopLeft,
                Signal::TopRight,
                Signal::Middle,
                Signal::BottomLeft,
                Signal::BottomRight,
                Signal::Bottom,
            ],
        ),
        (
            Digit::Nine,
            vec![
                Signal::Top,
                Signal::TopLeft,
                Signal::TopRight,
                Signal::Middle,
                Signal::BottomRight,
                Signal::Bottom,
            ],
        ),
    ]
}

#[derive(Debug)]
struct Pattern {
    wires: Vec<Wire>,
}

impl Pattern {
    fn parse(data: &str) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let wires = data.chars().filter_map(Wire::parse).collect();

        Some(Self { wires })
    }

    fn len(&self) -> usize {
        self.wires.len()
    }

    fn decode(&self, solution: &[(Wire, Signal)]) -> usize {
        let signals: Vec<_> = self
            .wires
            .iter()
            .filter_map(|wire| solution.iter().find_map(|(w, s)| (w == wire).then_some(s)))
            .collect();

        *digit_patterns()
            .iter()
            .find_map(|(digit, pattern)| {
                signals
                    .iter()
                    .all(|s| pattern.contains(*s))
                    .then_some(digit)
            })
            .expect("The solution should work to find a digit") as usize
    }
}

/// This struct handles the main logic for figuring out which signal corresponds to which wire
#[derive(Debug, Default, PartialEq, Eq)]
struct Solver {
    changed: bool,
    guesses: Vec<(Wire, Signal)>,
    groups: Vec<(Vec<Wire>, Vec<Signal>)>,
}

const ALL_SIGNALS: [Signal; 7] = [
    Signal::Top,
    Signal::TopLeft,
    Signal::TopRight,
    Signal::Middle,
    Signal::BottomLeft,
    Signal::BottomRight,
    Signal::Bottom,
];

const ALL_WIRES: [Wire; 7] = [
    Wire::A,
    Wire::B,
    Wire::C,
    Wire::D,
    Wire::E,
    Wire::F,
    Wire::G,
];

impl Display for Solver {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "  |abcdefg")?;
        for signal in ALL_SIGNALS {
            writeln!(
                f,
                "{:>2}|{}",
                signal,
                ALL_WIRES
                    .iter()
                    .map(|wire| {
                        if self.solution().any(|(w, s)| w == *wire && s == signal) {
                            'x'
                        } else if self.guesses.contains(&(*wire, signal)) {
                            ' '
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            )?;
        }
        writeln!(f, "----------")?;
        writeln!(f, "CHANGED: {}", if self.changed { 'Y' } else { 'N' })
    }
}

impl Solver {
    /// Instantiate the Solver struct, initializing it with all possible wire/signal combinations
    fn new() -> Self {
        let guesses = ALL_WIRES
            .into_iter()
            .flat_map(|wire| ALL_SIGNALS.into_iter().map(move |signal| (wire, signal)))
            .collect();

        Self {
            guesses,
            ..Self::default()
        }
    }

    /// Figure out which signal corresponds to which wire, based on the given observed pattern samples
    fn solve(&mut self, samples: &[Pattern]) -> Option<Vec<(Wire, Signal)>> {
        eprintln!("{}", &self);
        loop {
            for sample in samples {
                self.changed = false;

                // Find all digit patterns that have the same length as this sample
                let matched_patterns: Vec<_> = digit_patterns()
                    .into_iter()
                    .filter(|(_, ps)| ps.len() == sample.len())
                    .map(|t| t.1)
                    .collect();

                let wires = &sample.wires;
                if let Some(pattern) = self.deduce_pattern(wires, matched_patterns) {
                    eprintln!(
                        "Wires   : '{}'",
                        &wires.iter().map(Wire::to_string).collect::<String>()
                    );
                    eprintln!(
                        "Pattern : '{}'",
                        &pattern
                            .iter()
                            .map(Signal::to_string)
                            .collect::<Vec<_>>()
                            .join(",")
                    );
                    self.deduce(wires, pattern);
                }

                if let Some(solution) = self.is_solved() {
                    return Some(solution.clone());
                }
            }

            if !self.changed {
                return None;
            }
        }
    }

    /// Figure out which signal pattern is correct for this wire pattern
    fn deduce_pattern(
        &self,
        wires: &[Wire],
        mut signal_patterns: Vec<Vec<Signal>>,
    ) -> Option<Vec<Signal>> {
        // If there's only one to begin with, return it
        if signal_patterns.len() == 1 {
            return signal_patterns.pop();
        }

        // If all wires in a group are found within the wire pattern, all signals must also appear
        // in the signal pattern
        signal_patterns.retain(|signals| {
            self.groups().all(|(ws, ss)| {
                if ws.iter().all(|w| wires.contains(w)) {
                    ss.iter().all(|s| signals.contains(s))
                } else {
                    true
                }
            })
        });

        // if there is exactly one pattern left, that's the correct pattern.
        if signal_patterns.len() == 1 {
            signal_patterns.pop()
        } else {
            None
        }
    }

    /// Figure out which wire belongs to which signal of the pattern
    fn deduce(&mut self, wires: &[Wire], signals: Vec<Signal>) {
        // dbg!(&wires, &signals);
        // narrow guesses
        self.narrow_guesses(wires, &signals);

        // for each signal, find all wires which still have not been solved -- if there's only
        // one, mark that as known
        for signal in signals {
            let unsolved_wires: Vec<_> = wires
                .iter()
                .filter(|wire| !self.solution().any(|(w, s)| s == signal && w == **wire))
                .collect();
            if unsolved_wires.len() == 1 {
                self.mark_known(*unsolved_wires[0], signal);
            }
        }

        eprintln!("{}", &self);
    }

    /// Add a wire/signal pair to the solution vec and remove all invalidated possibilities
    /// Also marks the Solver struct as changed for this iteration of the main solve loop
    fn mark_known(&mut self, wire: Wire, signal: Signal) {
        self.guesses
            .retain(|(w, s)| matches!((*w != wire, *s != signal), (true, true) | (false, false)));
        self.mark_changed();
    }

    /// Cross off possibilities that we know are invalid
    /// Also marks the Solver struct as changed for this iteration of the main solve loop
    fn narrow_guesses(&mut self, wires: &[Wire], signals: &[Signal]) {
        self.guesses.retain(|(w, s)| {
            matches!(
                (wires.contains(w), signals.contains(s)),
                (true, true) | (false, false)
            )
        });
        self.mark_changed();
    }

    /// Mark the struct as changed and recalculate the groups
    fn mark_changed(&mut self) {
        self.changed = true;
        self.update_groups();
    }

    /// If any possibilities are logically the only possibility
    /// for that pair, move it to the solution vec
    /// This is O(n * n)
    // fn find_known(&mut self) {
    //     let (mut move_to_solution, other_possibilities): (Vec<_>, Vec<_>) =
    //         self.guesses.iter().partition(|(wire, signal)| {
    //             // are there no other possibilities that have the same wire or the same signal
    //             self.guesses
    //                 .iter()
    //                 .all(|(w, s)| matches!((w == wire, s == signal), (true, true) | (false, false)))
    //         });

    //     self.solution.append(&mut move_to_solution);
    //     self.guesses = other_possibilities;
    // }

    /// Get all logical "groups" of wire/signal pairs (one or more wires sharing the same guesses,
    /// the number of which is the same as the number of wires themselves)
    fn update_groups(&mut self) {
        // Group the signals by wire
        let mut by_wire = self.guesses.iter().fold(
            Vec::with_capacity(ALL_WIRES.len()),
            |mut acc: Vec<(Wire, Vec<Signal>)>, (wire, signal)| {
                if let Some(pos) = acc.iter().position(|(w, _)| w == wire) {
                    acc[pos].1.push(*signal);
                } else {
                    acc.push((*wire, vec![*signal]));
                }
                acc
            },
        );

        let mut groups = Vec::new();

        for wire in ALL_WIRES {
            // for each entry, pull all the others that have the same guesses
            let pos = by_wire.iter().position(|(w, _)| *w == wire);
            if pos.is_none() {
                continue;
            }
            let pos = pos.unwrap();
            let entry = by_wire.swap_remove(pos);
            let (same_guesses, other): (Vec<_>, Vec<_>) = by_wire
                .iter()
                .partition(|(_, signals)| signals.iter().all(|s| entry.1.contains(s)));
            // Add one for the original entry
            let number_of_wires = same_guesses.len() + 1;
            // if the number of wires matches the number of guesses, that is a group
            if number_of_wires == entry.1.len() {
                let mut wires = vec![entry.0];
                let signals = entry.1;
                wires.extend(same_guesses.into_iter().map(|(w, _)| w));
                groups.push((wires, signals));
                let new_by_wire: Vec<_> = other.into_iter().cloned().collect();
                by_wire = new_by_wire;
            }
        }

        self.groups = groups;
    }

    /// Get an iterator over the groups (see above)
    fn groups(&self) -> impl Iterator<Item = (&Vec<Wire>, &Vec<Signal>)> {
        // The purpose of this map is so that the vectors are passed by reference, rather than the
        // tuple itself
        self.groups.iter().map(|(wires, signals)| (wires, signals))
    }

    /// Get all groups that have only one possibility. This is the solution, provided that all
    /// wires and signals are contained in this group
    fn solution(&self) -> impl Iterator<Item = (Wire, Signal)> + '_ {
        self.groups()
            .filter(|(wires, _signals)| wires.len() == 1)
            .map(|(wires, signals)| (wires[0], signals[0]))
    }

    /// Get the solution, if known
    fn is_solved(&self) -> Option<Vec<(Wire, Signal)>> {
        let solution: Vec<_> = self.solution().collect();
        if solution.len() == 7 {
            return Some(solution);
        }

        None
    }
}

macro_rules! samples_signals {
    ($data:ident) => {
        $data.lines().filter_map(|l| l.split_once(" | "))
    };
}

/// Get the total number of signal patterns with unique sizes in the data
pub fn unique_segment_total(data: &str) -> usize {
    samples_signals!(data)
        .map(|ps| {
            ps.1.split_ascii_whitespace()
                .filter(|p| DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS.contains(&p.len()))
                .count()
        })
        .sum()
}

/// Figure out what each wire corresponds to and decode the scrambled digits
pub fn solve_segments(data: &str) -> usize {
    samples_signals!(data)
        .filter_map(|(samples, signals)| {
            let mut samples: Vec<_> = samples
                .split_ascii_whitespace()
                .filter_map(Pattern::parse)
                .collect();

            // Sort the segments, first by uniqueness of length (unique first), then by length
            samples.sort_unstable_by_key(|s| {
                (
                    Reverse(DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS.contains(&s.len())),
                    s.len(),
                )
            });

            let mut solver = Solver::new();

            if let Some(solution) = solver.solve(&samples) {
                return signals
                    .split_ascii_whitespace()
                    .filter_map(Pattern::parse)
                    .map(|p| p.decode(&solution))
                    .rev()
                    .fold((0, 1), |(acc, column), digit| {
                        (acc + (digit * column), column * 10)
                    })
                    .0
                    .into();
            }

            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_pattern_using_solution() {
        let pattern = Pattern::parse("cefbgd").unwrap();

        assert_eq!(
            9,
            pattern.decode(&[
                (Wire::A, Signal::BottomLeft),
                (Wire::B, Signal::TopRight),
                (Wire::C, Signal::Middle),
                (Wire::D, Signal::Top),
                (Wire::E, Signal::BottomRight),
                (Wire::F, Signal::Bottom),
                (Wire::G, Signal::TopLeft)
            ])
        );
    }

    #[test]
    fn mark_solved() {
        let mut solver = Solver {
            guesses: vec![
                (Wire::A, Signal::Top),
                (Wire::A, Signal::Middle),
                (Wire::A, Signal::Bottom),
                (Wire::B, Signal::Top),
                (Wire::B, Signal::Middle),
                (Wire::B, Signal::Bottom),
                (Wire::C, Signal::Top),
                (Wire::C, Signal::Middle),
                (Wire::C, Signal::Bottom),
            ],
            ..Default::default()
        };

        solver.mark_known(Wire::A, Signal::Top);

        assert_eq!(
            Solver {
                guesses: vec![
                    (Wire::A, Signal::Top),
                    (Wire::B, Signal::Middle),
                    (Wire::B, Signal::Bottom),
                    (Wire::C, Signal::Middle),
                    (Wire::C, Signal::Bottom),
                ],
                changed: true,
                groups: vec![
                    (vec![Wire::A], vec![Signal::Top]),
                    (vec![Wire::B, Wire::C], vec![Signal::Middle, Signal::Bottom])
                ],
            },
            solver
        );
    }

    #[test]
    fn narrow_guesses() {
        let mut solver = Solver {
            guesses: vec![
                (Wire::A, Signal::Top),
                (Wire::A, Signal::Middle),
                (Wire::A, Signal::Bottom),
                (Wire::B, Signal::Top),
                (Wire::B, Signal::Middle),
                (Wire::B, Signal::Bottom),
                (Wire::C, Signal::Top),
                (Wire::C, Signal::Middle),
                (Wire::C, Signal::Bottom),
            ],
            ..Default::default()
        };

        solver.narrow_guesses(&[Wire::A, Wire::B], &[Signal::Top, Signal::Bottom]);

        assert_eq!(
            Solver {
                guesses: vec![
                    (Wire::A, Signal::Top),
                    (Wire::A, Signal::Bottom),
                    (Wire::B, Signal::Top),
                    (Wire::B, Signal::Bottom),
                    (Wire::C, Signal::Middle),
                ],
                changed: true,
                groups: vec![
                    (vec![Wire::A, Wire::B], vec![Signal::Top, Signal::Bottom]),
                    (vec![Wire::C], vec![Signal::Middle])
                ]
            },
            solver
        );
    }

    #[test]
    fn groups() {
        let mut solver = Solver {
            guesses: vec![
                (Wire::A, Signal::Top),
                (Wire::A, Signal::Middle),
                (Wire::A, Signal::Bottom),
                (Wire::B, Signal::Top),
                (Wire::B, Signal::Middle),
                (Wire::B, Signal::Bottom),
                (Wire::C, Signal::Top),
                (Wire::C, Signal::Middle),
                (Wire::C, Signal::Bottom),
            ],
            ..Default::default()
        };

        solver.narrow_guesses(&[Wire::A, Wire::B], &[Signal::Top, Signal::Bottom]);

        assert_eq!(
            vec![
                (&vec![Wire::A, Wire::B], &vec![Signal::Top, Signal::Bottom]),
                (&vec![Wire::C], &vec![Signal::Middle])
            ],
            solver.groups().collect::<Vec<_>>()
        );
    }

    const SMALL_DATA: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const DATA: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_unique_segment_total() {
        assert_eq!(26, unique_segment_total(DATA));
    }

    #[test]
    fn test_solve_segments() {
        assert_eq!(5353, solve_segments(SMALL_DATA));
    }
}
