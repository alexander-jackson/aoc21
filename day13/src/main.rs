use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Copy, Clone, Debug)]
enum Direction {
    X,
    Y,
}

impl Direction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(alt((tag("x"), tag("y"))), |v| {
            if v == "x" {
                Self::X
            } else {
                Self::Y
            }
        })(input)
    }
}

#[derive(Copy, Clone, Debug)]
struct Dot {
    x: i32,
    y: i32,
}

impl Dot {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                nom::character::complete::i32,
                tag(","),
                nom::character::complete::i32,
            ),
            |(x, y)| Self { x, y },
        )(input)
    }
}

#[derive(Debug)]
struct Paper {
    dots: Vec<Dot>,
}

impl Paper {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, Dot::parse), |dots| Self { dots })(input)
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    direction: Direction,
    position: i32,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(
                tag("fold along "),
                separated_pair(Direction::parse, tag("="), nom::character::complete::i32),
            ),
            |(direction, position)| Self {
                direction,
                position,
            },
        )(input)
    }
}

#[derive(Debug)]
struct Input {
    paper: Paper,
    instructions: Vec<Instruction>,
}

impl Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                Paper::parse,
                tuple((newline, newline)),
                separated_list1(newline, Instruction::parse),
            ),
            |(paper, instructions)| Self {
                paper,
                instructions,
            },
        )(input)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = include_str!("../sample.txt");
    let input = Input::parse(content)?.1;

    Ok(())
}
