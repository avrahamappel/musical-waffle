#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use std::collections::HashMap;

use itertools::Itertools;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;

type NomError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, PartialEq)]
pub enum Error<'a, 'b> {
    ParseError(NomError<'a>),
    CalcError(&'b str),
}

fn sum_numbers_until(num: u32) -> u32 {
    let float_num: f64 = num.into();
    let avg = (1.0 + float_num) / 2.0;
    // This should be the same as adding all numbers until u32, so we shouldn't get a decimal here
    // and casting back should be fine
    (avg * float_num) as u32
}

fn crab_alignment<F, G>(data: &str, fuel_calc: F) -> Result<u32, Error>
where
    F: Fn(u32) -> G,
    G: Fn((&u32, &u32)) -> u32,
{
    let (_, crab_positions) =
        separated_list1(char(','), map_res(digit1, str::parse))(data).map_err(Error::ParseError)?;

    let crab_count = crab_positions.len();

    let mapped_crab_positions: HashMap<u32, u32> =
        crab_positions
            .into_iter()
            .fold(HashMap::with_capacity(crab_count), |mut hm, pos| {
                let init = 1;
                hm.entry(pos).and_modify(|c| *c += init).or_insert(init);
                hm
            });

    let possible_alignments = mapped_crab_positions.keys().min().copied().unwrap_or(0)
        ..=mapped_crab_positions.keys().max().copied().unwrap_or(0);

    let results = possible_alignments
        .map(|pos| mapped_crab_positions.iter().map(fuel_calc(pos)).sum())
        .sorted()
        .collect_vec();

    if results.is_empty() {
        return Err(Error::CalcError("No results."));
    }

    Ok(results[0])
}

pub fn crab_alignment_constant(data: &str) -> Result<u32, Error> {
    crab_alignment(data, |pos| move |(p, c)| p.abs_diff(pos) * c)
}

pub fn crab_alignment_increasing(data: &str) -> Result<u32, Error> {
    crab_alignment(data, |pos| {
        move |(p, c)| sum_numbers_until(p.abs_diff(pos)) * c
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_crab_alignment_constant() {
        if let Ok(alignment) = crab_alignment_constant(DATA) {
            assert_eq!(37, alignment);
        }
    }

    #[test]
    fn test_crab_alignment_increasing() {
        if let Ok(alignment) = crab_alignment_increasing(DATA) {
            assert_eq!(168, alignment);
        }
    }
}
