use itertools::Itertools;

macro_rules! lines_to_ints {
    ($data:ident) => {
        $data.filter_map(|l| l.trim().parse::<u32>().ok())
    };
}

pub fn sweep_increases<'a, I: Iterator<Item = &'a str>>(data: I) -> usize {
    lines_to_ints!(data)
        .tuple_windows()
        .filter_map(|(s, n)| (n > s).then_some(s))
        .count()
}

pub fn sweep_window_increases<'a, I: Iterator<Item = &'a str>>(data: I) -> usize {
    lines_to_ints!(data)
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter_map(|(s, n)| (n > s).then_some(s))
        .count()
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
        assert_eq!(7, sweep_increases(DATA.lines().map(Into::into)));
    }

    #[test]
    fn it_calculates_sweep_window_increases() {
        assert_eq!(5, sweep_window_increases(DATA.lines().map(Into::into)));
    }
}
