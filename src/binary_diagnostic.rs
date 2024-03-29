#![allow(clippy::cast_possible_truncation)]

use std::cmp::Ordering;

use crate::utils::div_ceil;

type BitList = Vec<u32>;

fn strings_to_bit_lists<'a, I>(data: I) -> Vec<BitList>
where
    I: Iterator<Item = &'a str>,
{
    data.map(|s| {
        s.chars()
            .map(|c| c.to_digit(2).expect("Couldn't parse digit"))
            .collect()
    })
    .collect()
}

fn most_common_bits(data: &Vec<BitList>) -> BitList {
    (0..data[0].len()).fold(Vec::with_capacity(data[0].len()), |mut result, i| {
        result.push(most_common_bit(data, i));
        result
    })
}

fn most_common_bit(data: &Vec<BitList>, position: usize) -> u32 {
    let digits_sum: u32 = data.iter().map(|bs| bs[position]).sum();

    match digits_sum.cmp(&div_ceil(data.len() as u32, 2)) {
        Ordering::Greater | Ordering::Equal => 1,
        Ordering::Less => 0,
    }
}

fn single_most_common_bitlist(data: Vec<BitList>, position: Option<usize>) -> BitList {
    let p = position.unwrap_or(0);

    if data.len() == 1 {
        return data[0].clone();
    }

    let most_common_bit = most_common_bit(&data, p);

    single_most_common_bitlist(
        data.into_iter()
            .filter_map(|b| (b[p] == most_common_bit).then_some(b))
            .collect(),
        Some(p + 1),
    )
}

fn least_common_bits(data: &Vec<BitList>) -> BitList {
    (0..data[0].len()).fold(Vec::with_capacity(data[0].len()), |mut result, i| {
        result.push(least_common_bit(data, i));
        result
    })
}

fn least_common_bit(data: &Vec<BitList>, position: usize) -> u32 {
    let digits_sum: u32 = data.iter().map(|bs| bs[position]).sum();

    match digits_sum.cmp(&div_ceil(data.len() as u32, 2)) {
        Ordering::Greater | Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}

fn single_least_common_bitlist(data: Vec<BitList>, position: Option<usize>) -> BitList {
    let p = position.unwrap_or(0);

    if data.len() == 1 {
        return data[0].clone();
    }

    let least_common_bit = least_common_bit(&data, p);

    single_least_common_bitlist(
        data.into_iter()
            .filter_map(|b| (b[p] == least_common_bit).then_some(b))
            .collect(),
        Some(p + 1),
    )
}

fn bin_to_dec(bin: &BitList) -> u32 {
    if bin.is_empty() {
        return 0;
    }

    bin.iter().copied().reduce(|res, b| (res * 2) + b).unwrap()
}

pub fn diagnose_power_consumption<'a, I>(data: I) -> u32
where
    I: Iterator<Item = &'a str>,
{
    let data = strings_to_bit_lists(data);
    let gamma = most_common_bits(&data);
    let epsilon = least_common_bits(&data);

    bin_to_dec(&gamma) * bin_to_dec(&epsilon)
}

pub fn diagnose_life_support<'a, I>(data: I) -> u32
where
    I: Iterator<Item = &'a str>,
{
    let data = strings_to_bit_lists(data);
    let oxygen_gen_rating = single_most_common_bitlist(data.clone(), None);
    let co2_scrubber_rating = single_least_common_bitlist(data, None);

    bin_to_dec(&oxygen_gen_rating) * bin_to_dec(&co2_scrubber_rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn it_diagnoses_power_consumption() {
        assert_eq!(
            198,
            diagnose_power_consumption(DATA.lines().map(Into::into))
        );
    }

    #[test]
    fn it_diagnoses_life_support() {
        assert_eq!(230, diagnose_life_support(DATA.lines().map(Into::into)));
    }
}
