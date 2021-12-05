use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (x, y)) = separated_pair(
            nom::character::complete::u32,
            tag(","),
            nom::character::complete::u32,
        )(input)?;

        Ok((input, Self { x, y }))
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (start, end)) = separated_pair(Point::parse, tag(" -> "), Point::parse)(input)?;

        Ok((input, Self { start, end }))
    }
}

#[derive(Debug)]
struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, lines) = separated_list1(newline, Line::parse)(input)?;

        Ok((input, Self { lines }))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_, input) = Input::parse(include_str!("../sample.txt"))?;
    dbg!(&input);

    Ok(())
}
