use std::collections::HashMap;

use itertools::Itertools;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;

type NomError<'a> = nom::Err<nom::error::Error<&'a str>>;

pub enum Error<'a, 'b> {
    ParseError(NomError<'a>),
    CalcError(&'b str),
}

pub fn crab_alignment(data: &str) -> Result<u32, Error> {
    let (_, crab_positions) = separated_list1(char(','), map_res(digit1, str::parse))(data)
        .or_else(|e| Err(Error::ParseError(e)))?;

    let crab_count = crab_positions.len();

    let map: HashMap<u32, u32> =
        crab_positions
            .into_iter()
            .fold(HashMap::with_capacity(crab_count), |mut hm, pos| {
                hm.entry(pos).and_modify(|c| *c += 1).or_insert(1);
                hm
            });

    let results = map
        .iter()
        .map(|(pos, _)| map.iter().map(|(p, c)| p.abs_diff(*pos) * c).sum::<u32>())
        .sorted()
        .collect_vec();

    if results.is_empty() {
        return Err(Error::CalcError("No results."));
    }

    Ok(results[0])
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::*;

    const DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_crab_alignment() {
        if let Ok(alignment) = crab_alignment(DATA) {
            assert_eq!(37, alignment)
        }
    }

    #[bench]
    fn bench_crab_alignment(b: &mut Bencher) {
        b.iter(|| crab_alignment(DATA))
    }
}
