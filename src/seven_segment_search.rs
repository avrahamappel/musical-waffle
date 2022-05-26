use std::collections::HashMap;

use Signal::*;

macro_rules! log {
    ($($msg:tt)*) => {
        if cfg!(test) {
            println!($($msg)*);
        }
    };
}

/// Corresponds to digits 1, 4, 7, and 8
const DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS: [usize; 4] = [2, 4, 3, 7];

type DisplayChar = char;
type Pattern = Vec<DisplayChar>;

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

fn all_signals() -> [Signal; 7] {
    [
        Top,
        TopLeft,
        TopRight,
        Middle,
        BottomLeft,
        BottomRight,
        Bottom,
    ]
}

fn signal_patterns<'a>() -> HashMap<&'a str, Vec<Signal>> {
    HashMap::from([
        (
            "ZERO",
            vec![Top, TopLeft, TopRight, BottomLeft, BottomRight, Bottom],
        ),
        ("ONE", vec![TopRight, BottomRight]),
        ("TWO", vec![Top, TopRight, Middle, BottomLeft, Bottom]),
        ("THREE", vec![Top, TopRight, Middle, BottomRight, Bottom]),
        ("FOUR", vec![TopLeft, TopRight, Middle, BottomRight]),
        ("FIVE", vec![Top, TopLeft, Middle, BottomRight, Bottom]),
        (
            "SIX",
            vec![Top, TopLeft, Middle, BottomLeft, BottomRight, Bottom],
        ),
        ("SEVEN", vec![Top, TopRight, BottomRight]),
        (
            "EIGHT",
            vec![
                Top,
                TopLeft,
                TopRight,
                Middle,
                BottomLeft,
                BottomRight,
                Bottom,
            ],
        ),
        (
            "NINE",
            vec![Top, TopLeft, TopRight, Middle, BottomRight, Bottom],
        ),
    ])
}

fn solve(
    patterns: Vec<Pattern>,
    mut solved: HashMap<DisplayChar, Signal>,
    mut possibilities: Vec<(Pattern, DisplayChar, Vec<Signal>)>,
) -> HashMap<DisplayChar, Signal> {
    // If we're done, we're done
    if patterns.is_empty() {
        log!("All done! Solved {:?}", solved);
        return solved;
    }

    // get next pattern
    let pattern = patterns[0].to_owned();

    // find first signal list that matches the length
    // TODO pass this in
    let signal_patterns = signal_patterns();
    let len_matched_sig_pats = signal_patterns
        .iter()
        .map(|(_, v)| v)
        .filter(|v| v.len() == pattern.len())
        .collect::<Vec<_>>();

    // if anything is already known etc.
    let guesses = pattern.clone();

    let len = guesses.len();

    // create possibilities list
    for (i, c) in guesses.into_iter().enumerate() {
        possibilities.push((
            pattern.clone(),
            c,
            len_matched_sig_pats[0]
                .iter()
                .copied()
                .cycle()
                .skip(i)
                .take(len)
                .collect(),
        ));
    }

    solved = possibilities
        .iter()
        .fold(HashMap::with_capacity(len), |mut acc, (_, c, v)| {
            acc.insert(*c, v[0]);
            acc
        });

    solve(patterns, solved, possibilities)
}

pub fn unique_segment_total(data: &str) -> usize {
    data.lines()
        .flat_map(|l| {
            l.split('|').skip(1).take(1).map(|ps| {
                ps.split_ascii_whitespace()
                    .filter(|p| DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS.contains(&p.len()))
                    .count()
            })
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
    fn test_solve() {
        let mut possibilities = HashMap::with_capacity(7);

        for d in 'a'..='g' {
            possibilities.insert(
                d,
                vec![
                    Top,
                    TopRight,
                    TopLeft,
                    Middle,
                    BottomLeft,
                    BottomRight,
                    Bottom,
                ],
            );
        }

        let mut patterns = vec![
            vec!['a', 'c', 'e', 'd', 'g', 'f', 'b'],
            vec!['c', 'd', 'f', 'b', 'e'],
            vec!['g', 'c', 'd', 'f', 'a'],
            vec!['f', 'b', 'c', 'a', 'd'],
            vec!['d', 'a', 'b'],
            vec!['c', 'e', 'f', 'a', 'b', 'd'],
            vec!['c', 'd', 'f', 'g', 'e', 'b'],
            vec!['e', 'a', 'f', 'b'],
            vec!['c', 'a', 'g', 'e', 'd', 'b'],
            vec!['a', 'b'],
        ];
        patterns.sort();
        let len = patterns.len();

        assert_eq!(
            HashMap::from([]),
            solve(patterns, HashMap::new(), Vec::with_capacity(len))
        )
    }
}
