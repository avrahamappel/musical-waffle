pub fn diagnose<I>(data: I) -> u32
where
    I: Iterator<Item = String>,
{
    let data: Vec<Vec<_>> = data.map(|s| s.chars().collect()).collect();
    let mut gamma = String::with_capacity(data[0].len());

    for i in 0..data[0].len() {
        let digits_sum: u32 = data
            .iter()
            .map(|c| c[i].to_digit(2).expect("Couldn't parse digit"))
            .sum();

        let gamma_digit = if digits_sum > data.len() as u32 / 2 {
            '1'
        } else {
            '0'
        };

        gamma.push(gamma_digit);
    }

    let err = "Bin to dec parsing failed";
    let gamma_dec = u32::from_str_radix(&gamma, 2).expect(err);
    let epsilon: String = gamma
        .chars()
        .map(|b| if b == '1' { '0' } else { '1' })
        .collect();

    gamma_dec * u32::from_str_radix(&epsilon, 2).expect(err)
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
    fn it_diagnoses_binary_codes() {
        assert_eq!(198, diagnose(DATA.lines().map(Into::into)));
    }
}
