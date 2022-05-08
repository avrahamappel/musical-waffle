use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{eof, map, map_res};
use nom::multi::many0;
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};

fn separated_twins<F, G, H, I, O1, O2>(f: F, sep: H) -> impl FnMut(I) -> IResult<I, (O1, O1)>
where
    F: Fn() -> G,
    G: Parser<I, O1, nom::error::Error<I>>,
    H: Parser<I, O2, nom::error::Error<I>>,
{
    separated_pair(f(), sep, f())
}

#[derive(Debug, PartialEq, Eq)]
struct Point(u32, u32);

impl Point {
    fn parse<'a>(input: &'a str) -> IResult<&'a str, Self> {
        map(
            separated_twins(|| map_res(digit1, str::parse), char(',')),
            |(x, y)| Self(x, y),
        )(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line(Point, Point);

impl Line {
    fn parse<'a>(input: &'a str) -> IResult<&'a str, Self> {
        map(separated_twins(|| Point::parse, tag(" -> ")), |(p1, p2)| {
            Line(p1, p2)
        })(input)
    }
}

fn vent_parser<'a>(input: &'a str) -> IResult<&'a str, Vec<Line>> {
    many0(terminated(Line::parse, alt((line_ending, eof))))(input)
}

// pub fn dangerous_points(data: &str) -> u32 {
//     let parsed = vent_parser(data);

//     dbg!(parsed)
// }

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn it_parses_a_point_from_a_string_slice() {
        assert_eq!(Ok(("", Point(8, 0))), Point::parse("8,0"))
    }

    #[test]
    fn it_parses_a_line_from_a_string_slice() {
        assert_eq!(
            Ok(("", Line(Point(8, 0), Point(0, 8)))),
            Line::parse("8,0 -> 0,8")
        )
    }

    #[test]
    fn it_parses_lines() {
        assert_eq!(
            Ok((
                "",
                vec![
                    Line(Point(0, 9), Point(5, 9)),
                    Line(Point(8, 0), Point(0, 8)),
                    Line(Point(9, 4), Point(3, 4)),
                    Line(Point(2, 2), Point(2, 1)),
                    Line(Point(7, 0), Point(7, 4)),
                    Line(Point(6, 4), Point(2, 0)),
                    Line(Point(0, 9), Point(2, 9)),
                    Line(Point(3, 4), Point(1, 4)),
                    Line(Point(0, 0), Point(8, 8)),
                    Line(Point(5, 5), Point(8, 2)),
                ]
            )),
            vent_parser(DATA)
        )
    }

    // #[test]
    // fn it_calculates_dangerous_points() {
    //     // assert_eq!(5, dangerous_points(DATA));
    // }
}
