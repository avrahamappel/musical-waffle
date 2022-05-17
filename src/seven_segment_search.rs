/// Corresponds to digits 1, 4, 7, and 8
const DIGITS_WITH_UNIQUE_NUMBER_SEGMENTS: [usize; 4] = [2, 4, 3, 7];

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
}
