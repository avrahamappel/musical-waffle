use itertools::Itertools;

pub fn sweep_increases<I: Iterator<Item = String>>(data: I) -> usize {
    lines_to_ints(data)
        .tuple_windows()
        .filter_map(|(s, n)| (n > s).then(|| s))
        .count()
}

fn lines_to_ints<I>(data: I) -> impl Iterator<Item = u32>
where
    I: Iterator<Item = String>,
{
    data.filter_map(|l| l.trim().parse::<u32>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"199
200
208
210
200
207
240
269
260
263"#;

    #[test]
    fn it_calculates_sweep_increases() {
        assert_eq!(7, sweep_increases(DATA.lines().map(|l| l.into())))
    }
}
