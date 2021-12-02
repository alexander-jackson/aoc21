use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, slice) = alt((tag("forward"), tag("up"), tag("down")))(input)?;

        let variant = match slice {
            "forward" => Self::Forward,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => unreachable!(),
        };

        Ok((input, variant))
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    magnitude: u64,
}

impl Command {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (direction, _, magnitude, _)) = tuple((
            Direction::parse,
            tag(" "),
            nom::character::complete::u64,
            tag("\n"),
        ))(input)?;

        Ok((
            input,
            Command {
                direction,
                magnitude,
            },
        ))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../sample.txt");
    let commands = many1(Command::parse)(input)?.1;
    dbg!(&commands);

    Ok(())
}
