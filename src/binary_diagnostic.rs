pub fn diagnose<I>(data: I) -> u32
where
    I: Iterator<Item = String>,
{
    let data: Vec<Vec<_>> = data
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(2).expect("Couldn't parse digit"))
                .collect()
        })
        .collect();
    let mut gamma = Vec::with_capacity(data[0].len());

    for i in 0..data[0].len() {
        let digits_sum: u32 = data.iter().map(|d| d[i]).sum();

        let gamma_digit = if digits_sum > data.len() as u32 / 2 {
            1
        } else {
            0
        };

        gamma.push(gamma_digit);
    }

    let epsilon: Vec<_> = gamma.iter().map(|b| if *b == 1 { 0 } else { 1 }).collect();

    bin_to_dec(&gamma) * bin_to_dec(&epsilon)
}

fn bin_to_dec(bin: &[u8]) -> u32 {
    if bin.is_empty() {
        return 0;
    }

    bin.into_iter()
        .map(|u| *u as u32)
        .reduce(|res, b| (res * 2) + b)
        .unwrap()
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
