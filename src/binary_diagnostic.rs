use std::cmp::Ordering;

type BitList = Vec<u32>;

fn strings_to_bit_lists<I>(data: I) -> Vec<BitList>
where
    I: Iterator<Item = String>,
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
    let digits_sum: u32 = data.iter().map(|d| d[position]).sum();

    match digits_sum.cmp(&(data.len() as u32 / 2)) {
        Ordering::Greater => 1,
        Ordering::Equal => 1,
        Ordering::Less => 0,
    }
}

fn least_common_bits(data: &Vec<BitList>) -> BitList {
    (0..data[0].len()).fold(Vec::with_capacity(data[0].len()), |mut result, i| {
        result.push(least_common_bit(data, i));
        result
    })
}

fn least_common_bit(data: &Vec<BitList>, position: usize) -> u32 {
    let digits_sum: u32 = data.iter().map(|d| d[position]).sum();

    match digits_sum.cmp(&(data.len() as u32 / 2)) {
        Ordering::Greater => 0,
        Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}

fn bin_to_dec(bin: &BitList) -> u32 {
    if bin.is_empty() {
        return 0;
    }

    bin.into_iter()
        .map(|u| *u as u32)
        .reduce(|res, b| (res * 2) + b)
        .unwrap()
}

pub fn diagnose_power_consumption<I>(data: I) -> u32
where
    I: Iterator<Item = String>,
{
    let data = strings_to_bit_lists(data);
    let gamma = most_common_bits(&data);
    let epsilon = least_common_bits(&data);

    bin_to_dec(&gamma) * bin_to_dec(&epsilon)
}

pub fn diagnose_life_support<I>(data: I) -> u32
where
    I: Iterator<Item = String>,
{
    let data = strings_to_bit_lists(data);
    let mut oxygen_gen_rating = Vec::with_capacity(data[0].len());
    let mut co2_scrubber_rating = Vec::with_capacity(data[0].len());
    let mut o_g_temp = data.clone();
    let mut c_s_temp = data.clone();

    for i in 0..o_g_temp[0].len() {
        let most_common_bit = most_common_bit(&o_g_temp, i);

        o_g_temp = o_g_temp
            .into_iter()
            .filter_map(|b| (b[i] == most_common_bit).then(|| b))
            .collect();

        if o_g_temp.len() == 1 {
            oxygen_gen_rating = o_g_temp.remove(0);
            break;
        }
    }

    for i in 0..c_s_temp[0].len() {
        let least_common_bit = least_common_bit(&c_s_temp, i);
        c_s_temp = c_s_temp
            .into_iter()
            .filter_map(|b| (b[i] == least_common_bit).then(|| b))
            .collect();

        if c_s_temp.len() == 1 {
            co2_scrubber_rating = c_s_temp.remove(0);
            break;
        }
    }

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
        assert_eq!(230, diagnose_life_support(DATA.lines().map(Into::into)))
    }
}
