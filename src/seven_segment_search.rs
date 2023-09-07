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
// TODO change its name to Solver
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
    /// Instantiate the Guesses struct, initializing it with all possible wire/signal combinations
    fn new() -> Self {
        let guesses = ALL_SIGNALS
            .into_iter()
            .flat_map(|signal| ALL_WIRES.into_iter().map(move |wire| (wire, signal)))
            .collect();

        Self {
            guesses,
            ..Self::default()
        }
    }

    /// Figure out which signal corresponds to which wire, based on the given observed pattern samples
    fn solve(&mut self, samples: &[Pattern]) -> Option<Vec<(Wire, Signal)>> {
        loop {
            for sample in samples {
                self.changed = false;

                let matched_patterns: Vec<_> = digit_patterns()
                    .into_iter()
                    .filter(|(_, ps)| ps.len() == sample.len())
                    .collect();

                for (_, pattern) in matched_patterns {
                    self.deduce(sample.wires.clone(), pattern);

                    if let Some(solution) = self.solved() {
                        return Some(solution.clone());
                    }
                }

                if !self.changed {
                    return None;
                }
            }
        }
    }

    /// Do some logic to narrow down the guesses
    fn deduce(&mut self, mut wires: Vec<Wire>, signals: Vec<Signal>) {
        dbg!(&wires, &signals);
        // narrow guesses
        self.narrow_guesses(&wires, &signals);

        // for each signal, find all wires which still have guesses in the table -- if there's only
        // one, mark that as known
        for signal in signals {
            let unsolved_wires: Vec<_> = wires
                .iter()
                .filter(|wire| self.guesses.iter().any(|(w, s)| *s == signal && w == *wire))
                .collect();
            if unsolved_wires.len() == 1 {
                self.mark_known(*unsolved_wires[0], signal);
            }
        }

        eprintln!("  |abcdefg");
        eprintln!("--+-------");
        for signal in ALL_SIGNALS {
            eprintln!(
                "{}{}",
                match signal {
                    Signal::Top => " T|",
                    Signal::TopLeft => "TL|",
                    Signal::TopRight => "TR|",
                    Signal::Middle => " M|",
                    Signal::BottomLeft => "BL|",
                    Signal::BottomRight => "BR|",
                    Signal::Bottom => " B|",
                },
                ALL_WIRES
                    .iter()
                    .map(
                        |wire| match self.guesses.iter().find(|(w, s)| (w == wire)) {
                            Some(_) => ' ',
                            None => '.',
                        }
                    )
                    .collect::<String>()
            );
        }
        dbg!(&self.solution);
    }

    /// Add a wire/signal pair to the solutions vec and remove all invalidated possibilities
    /// Also marks the guesses struct as changed for this iteration of the main solve loop
    fn mark_known(&mut self, wire: Wire, signal: Signal) {
        self.solution.push((wire, signal));
        self.guesses.retain(|(w, s)| *w != wire && *s != signal);
        self.changed = true;
    }

    /// Cross off guesses that we know are invalid
    fn narrow_guesses(&mut self, wires: &[Wire], signals: &[Signal]) {
        self.guesses.retain(|(w, s)| {
            if wires.contains(w) {
                return signals.contains(s);
            }

            if signals.contains(s) {
                return wires.contains(w);
            }

            true
        });
        self.changed = true;
    }

    /// Get the solution, if known
    fn solved(&self) -> Option<&Vec<(Wire, Signal)>> {
        if self.solution.len() == 7 {
            return Some(&self.solution);
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

            samples.sort_unstable_by_key(|s| {
                (
                    Reverse(DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS.contains(&s.len())),
                    s.len(),
                )
            });

            // dbg!(samples);

            // return None;

            let mut guesses = Guesses::new();

            if let Some(solution) = guesses.solve(&samples) {
                return signals
                    .split_ascii_whitespace()
                    .filter_map(Pattern::parse)
                    .map(|p| p.decode(&solution))
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
