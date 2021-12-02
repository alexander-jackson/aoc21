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
    magnitude: i64,
}

impl Command {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (direction, _, magnitude, _)) = tuple((
            Direction::parse,
            tag(" "),
            nom::character::complete::i64,
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
    let input = include_str!("../input.txt");
    let commands = many1(Command::parse)(input)?.1;

    let (pos, depth) = commands
        .iter()
        .fold((0, 0), |state, command| match command.direction {
            Direction::Forward => (state.0 + command.magnitude, state.1),
            Direction::Up => (state.0, state.1 - command.magnitude),
            Direction::Down => (state.0, state.1 + command.magnitude),
        });

    println!("Solution 1: {}", pos * depth);

    let (pos, depth, _) =
        commands
            .iter()
            .fold((0, 0, 0), |state, command| match command.direction {
                Direction::Forward => (
                    state.0 + command.magnitude,
                    state.1 + state.2 * command.magnitude,
                    state.2,
                ),
                Direction::Up => (state.0, state.1, state.2 - command.magnitude),
                Direction::Down => (state.0, state.1, state.2 + command.magnitude),
            });

    println!("Solution 2: {}", pos * depth);

    Ok(())
}
