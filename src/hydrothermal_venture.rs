use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending};
use nom::combinator::eof;
use nom::multi::many0;
use nom::number::complete::u32;
use nom::number::Endianness;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
struct Point(u32, u32);

impl Point {
    fn parse<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        separated_pair(u32(Endianness::Native), char(','), u32(Endianness::Native))(input)
            .map(|(inp, (x, y))| (inp, Self(x, y)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line(Point, Point);

impl Line {
    fn parse<'a>(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        separated_pair(Point::parse, tag(" -> "), Point::parse)(input)
            .map(|(inp, (p1, p2))| (inp, Line(p1, p2)))
    }
}

fn vent_parser<'a>(input: &'a str) -> IResult<&'a [u8], Vec<Line>> {
    many0(terminated(Line::parse, alt((line_ending, eof))))(input.as_bytes())
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
    fn it_parses_lines() {
        assert_eq!(
            Ok((
                "".as_bytes(),
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
