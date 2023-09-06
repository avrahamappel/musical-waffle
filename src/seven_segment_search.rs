use std::cmp::Reverse;

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

    fn decode(&self, solution: Vec<(Wire, Signal)>) -> usize {
        let signals: Vec<_> = self
            .wires
            .iter()
            .filter_map(|wire| solution.iter().find_map(|(w, s)| (w == wire).then_some(s)))
            .collect();

        digit_patterns()
            .iter()
            .find_map(|(digit, pattern)| (pattern == signals).then_some(digit))
            .expect("The solution should work to find a digit")
            .clone() as usize
    }
}

#[derive(Default)]
struct Guesses {
    changed: bool,
    guesses: Vec<(Wire, Signal)>,
    solution: Vec<(Wire, Signal)>,
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

impl Guesses {
    fn new() -> Self {
        let guesses = ALL_SIGNALS
            .into_iter()
            .flat_map(|signal| ALL_WIRES.into_iter().map(|wire| (wire, signal)))
            .collect();

        Self {
            guesses,
            ..Self::default()
        }
    }

    // TODO remove
    fn start_round(&self) {
        self.changed = false
    }

    // TODO remove
    fn hasnt_changed(&self) -> bool {
        !self.changed
    }

    /// Do some logic to figure out which wires correspond to which signals
    fn narrow(&self, wires: Box<dyn Iterator<Item = Wire>>, signals: Vec<Signal>) {
        // Remove signals that we already know
        let signals: Vec<_> = signals
            .iter()
            .copied()
            .filter(|s| self.solution.iter().find(|ws| *ws.1 == s).is_none())
            .collect();

        // take the first wire from the pattern
        let wire = wires.next();
        if wire.is_none() {
            return;
        }
        let wire = wire.unwrap();

        // if it's known, remove it and the corresponding signal
        // call this method again with the rest
        if let Some((_, signal)) = self.solution.iter().find(|(w, _)| *w == wire) {
            self.narrow(
                Box::new(wires.filter(|w| *w != wire)),
                signals.into_iter().filter(|s| s != signal).collect(),
            );
            return;
        }

        // if there is only one wire and one signal, we've established that it corresponds to that one
        // mark it as known
        if wires.done() {
            if signals.len() == 1 {
                self.mark_known(wire, signals[0]);
            }
            return;
        }

        let possible_signals_for_wire: Vec<_> = self
            .guesses
            .iter()
            .filter_map(|(w, s)| (*w == wire).then_some(*s))
            .collect();

        // if only one of the possible signals is present in the signals pattern, we've established that it corresponds to that one
        // mark this wire as known
        // remove and call this method with the rest
        let possibilites_in_signals_vec = signals
            .iter()
            .filter(|s| possible_signals_for_wire.contains(s));
        if possibilites_in_signals_vec.count() == 1 {
            let signal = possibilites_in_signals_vec.next().unwrap();
            self.mark_known(wire, *signal);
            self.narrow(
                Box::new(wires.filter(|w| *w != wire)),
                signals.into_iter().filter(|s| s != signal).collect(),
            );
            return;
        }

        if possible_signals_for_wire.len() != 2 {
            return;
        }

        // if it has 2 possible signals, and there is another wire with the same 2 possibilities
        // remove both and continue
        if let Some((wire2, _)) = self
            .guesses
            .iter()
            .find(|(w, s)| *w != wire && possible_signals_for_wire.contains(s))
        {
            self.narrow(
                Box::new(wires.filter(|w| ![wire, *wire2].contains(w))),
                signals
                    .into_iter()
                    .filter(|s| !possible_signals_for_wire.contains(s))
                    .collect(),
            );
            return;
        }
    }

    fn mark_known(&self, wire: Wire, signal: Signal) {
        todo!()
    }

    fn solved(&self) -> Option<Vec<(Wire, Signal)>> {
        if self.solution.len() == 7 {
            return Some(self.solution);
        }

        None
    }
}

macro_rules! samples_signals {
    ($data:ident) => {
        $data.lines().filter_map(|l| l.split_once(" | "))
    };
}

pub fn unique_segment_total(data: &str) -> usize {
    samples_signals!(data)
        .map(|ps| {
            ps.1.split_ascii_whitespace()
                .filter(|p| DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS.contains(&p.len()))
                .count()
        })
        .sum()
}

pub fn solve_segments(data: &str) -> usize {
    samples_signals!(data)
        .filter_map(|(samples, signals)| {
            let mut samples: Vec<_> = samples
                .split_ascii_whitespace()
                .filter_map(Pattern::parse)
                .collect();

            samples.sort_unstable_by_key(|s| {
                (
                    DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS.contains(&s.len()),
                    Reverse(s.len()),
                )
            });

            let guesses = Guesses::new();

            let solve = || loop {
                for sample in samples {
                    guesses.start_round();

                    let matched_patterns: Vec<_> = digit_patterns()
                        .into_iter()
                        .filter(|(d, ps)| ps.len() == sample.len())
                        .collect();

                    for (digit, pattern) in matched_patterns {
                        guesses.narrow(Box::new(sample.wires.into_iter()), pattern);

                        if let Some(solution) = guesses.solved() {
                            return Some(solution);
                        }
                    }

                    if guesses.hasnt_changed() {
                        return None;
                    }
                }
            };

            if let Some(solution) = solve() {
                return signals
                    .split_ascii_whitespace()
                    .filter_map(Pattern::parse)
                    .map(|p| p.decode(solution))
                    .rev()
                    .fold((1, 0), |(column, acc), digit| {
                        (column * 10, acc + digit + column)
                    })
                    .1
                    .into();
            }

            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(16, solve_segments(SMALL_DATA));
    }
}
