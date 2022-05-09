use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::{once, repeat, zip};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn parse<'a>(input: &'a str) -> IResult<&'a str, Self> {
        map(
            separated_twins(|| map_res(digit1, str::parse), char(',')),
            |(x, y)| Self { x, y },
        )(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Trajectory {
    None,
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn parse<'a>(input: &'a str) -> IResult<&'a str, Self> {
        map(
            separated_twins(|| Point::parse, tag(" -> ")),
            |(start, end)| Line { start, end },
        )(input)
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn traj(&self) -> Trajectory {
        match self.start.x.cmp(&self.end.x) {
            Ordering::Equal => match self.start.y.cmp(&self.end.y) {
                Ordering::Equal => Trajectory::None,
                Ordering::Greater => Trajectory::North,
                Ordering::Less => Trajectory::South,
            },
            Ordering::Greater => match self.start.y.cmp(&self.end.y) {
                Ordering::Equal => Trajectory::West,
                Ordering::Greater => Trajectory::Northwest,
                Ordering::Less => Trajectory::Southwest,
            },
            Ordering::Less => match self.start.y.cmp(&self.end.y) {
                Ordering::Equal => Trajectory::East,
                Ordering::Greater => Trajectory::Northeast,
                Ordering::Less => Trajectory::Southeast,
            },
        }
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point>> {
        match self.traj() {
            Trajectory::None => Box::new(once(self.start)),
            Trajectory::North => Box::new(
                zip(repeat(self.start.x), self.end.y..=self.start.y).map(|(x, y)| Point { x, y }),
            ),
            Trajectory::South => Box::new(
                zip(repeat(self.start.x), self.start.y..=self.end.y).map(|(x, y)| Point { x, y }),
            ),
            Trajectory::East => Box::new(
                zip(self.start.x..=self.end.x, repeat(self.start.y)).map(|(x, y)| Point { x, y }),
            ),
            Trajectory::West => Box::new(
                zip(self.end.x..=self.start.x, repeat(self.start.y)).map(|(x, y)| Point { x, y }),
            ),
            Trajectory::Northwest => todo!(),
            Trajectory::Northeast => todo!(),
            Trajectory::Southeast => todo!(),
            Trajectory::Southwest => todo!(),
        }
    }
}

fn vent_parser<'a>(input: &'a str) -> IResult<&'a str, Vec<Line>> {
    many0(terminated(Line::parse, alt((line_ending, eof))))(input)
}

pub fn dangerous_points(data: &str) -> Result<usize, nom::Err<nom::error::Error<&str>>> {
    let (_, lines) = vent_parser(data)?;
    let danger = lines
        .iter()
        .filter_map(|l| (!l.is_diagonal()).then(|| l.points()))
        .flatten()
        .fold(HashMap::new(), |mut danger_register, point| {
            danger_register
                .entry(point)
                .and_modify(|c| *c += 1)
                .or_insert(1);

            danger_register
        })
        .into_iter()
        .filter_map(|(_, v)| (v > 1).then(|| v))
        .count();

    Ok(danger)
}

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
        assert_eq!(Ok(("", Point { x: 8, y: 0 })), Point::parse("8,0"))
    }

    #[test]
    fn it_parses_a_line_from_a_string_slice() {
        assert_eq!(
            Ok((
                "",
                Line {
                    start: Point { x: 8, y: 0 },
                    end: Point { x: 0, y: 8 }
                }
            )),
            Line::parse("8,0 -> 0,8")
        )
    }

    #[test]
    fn it_parses_lines() {
        assert_eq!(
            Ok((
                "",
                vec![
                    Line {
                        start: Point { x: 0, y: 9 },
                        end: Point { x: 5, y: 9 }
                    },
                    Line {
                        start: Point { x: 8, y: 0 },
                        end: Point { x: 0, y: 8 }
                    },
                    Line {
                        start: Point { x: 9, y: 4 },
                        end: Point { x: 3, y: 4 }
                    },
                    Line {
                        start: Point { x: 2, y: 2 },
                        end: Point { x: 2, y: 1 }
                    },
                    Line {
                        start: Point { x: 7, y: 0 },
                        end: Point { x: 7, y: 4 }
                    },
                    Line {
                        start: Point { x: 6, y: 4 },
                        end: Point { x: 2, y: 0 }
                    },
                    Line {
                        start: Point { x: 0, y: 9 },
                        end: Point { x: 2, y: 9 }
                    },
                    Line {
                        start: Point { x: 3, y: 4 },
                        end: Point { x: 1, y: 4 }
                    },
                    Line {
                        start: Point { x: 0, y: 0 },
                        end: Point { x: 8, y: 8 }
                    },
                    Line {
                        start: Point { x: 5, y: 5 },
                        end: Point { x: 8, y: 2 }
                    },
                ]
            )),
            vent_parser(DATA)
        )
    }

    #[test]
    fn filter_out_diagonals() {
        if let Ok((_, lines)) = vent_parser(DATA) {
            assert_eq!(
                vec![
                    Line {
                        start: Point { x: 0, y: 9 },
                        end: Point { x: 5, y: 9 }
                    },
                    Line {
                        start: Point { x: 9, y: 4 },
                        end: Point { x: 3, y: 4 }
                    },
                    Line {
                        start: Point { x: 2, y: 2 },
                        end: Point { x: 2, y: 1 }
                    },
                    Line {
                        start: Point { x: 7, y: 0 },
                        end: Point { x: 7, y: 4 }
                    },
                    Line {
                        start: Point { x: 0, y: 9 },
                        end: Point { x: 2, y: 9 }
                    },
                    Line {
                        start: Point { x: 3, y: 4 },
                        end: Point { x: 1, y: 4 }
                    },
                ],
                lines
                    .into_iter()
                    .filter(|l| !l.is_diagonal())
                    .collect::<Vec<_>>()
            )
        }
    }

    #[test]
    fn it_computes_trajectory() {
        use super::Trajectory::*;

        if let Ok((_, lines)) = vent_parser(DATA) {
            assert_eq!(
                vec![East, West, North, South, East, West,],
                lines
                    .into_iter()
                    .filter_map(|l| (!l.is_diagonal()).then(|| l.traj()))
                    .collect::<Vec<_>>()
            )
        }
    }

    #[test]
    fn it_creates_vectors_of_points_from_line_definitions() {
        if let Ok((_, lines)) = vent_parser(DATA) {
            let expected = vec![
                vec![
                    Point { x: 0, y: 9 },
                    Point { x: 1, y: 9 },
                    Point { x: 2, y: 9 },
                    Point { x: 3, y: 9 },
                    Point { x: 4, y: 9 },
                    Point { x: 5, y: 9 },
                ],
                vec![
                    Point { x: 3, y: 4 },
                    Point { x: 4, y: 4 },
                    Point { x: 5, y: 4 },
                    Point { x: 6, y: 4 },
                    Point { x: 7, y: 4 },
                    Point { x: 8, y: 4 },
                    Point { x: 9, y: 4 },
                ],
                vec![Point { x: 2, y: 1 }, Point { x: 2, y: 2 }],
                vec![
                    Point { x: 7, y: 0 },
                    Point { x: 7, y: 1 },
                    Point { x: 7, y: 2 },
                    Point { x: 7, y: 3 },
                    Point { x: 7, y: 4 },
                ],
                vec![
                    Point { x: 0, y: 9 },
                    Point { x: 1, y: 9 },
                    Point { x: 2, y: 9 },
                ],
                vec![
                    Point { x: 1, y: 4 },
                    Point { x: 2, y: 4 },
                    Point { x: 3, y: 4 },
                ],
            ];

            for (exp, line) in zip(expected, lines.iter().filter(|l| !l.is_diagonal())) {
                assert_eq!(exp, line.points().collect::<Vec<_>>())
            }
        }
    }

    #[test]
    fn it_calculates_dangerous_points() {
        assert_eq!(Ok(5), dangerous_points(DATA));
    }
}
