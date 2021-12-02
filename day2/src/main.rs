use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug, Default)]
struct State {
    position: i64,
    depth: i64,
}

impl State {
    pub fn new(position: i64, depth: i64) -> Self {
        Self { position, depth }
    }

    fn apply(self, command: Command) -> Self {
        let Command {
            direction,
            magnitude,
        } = command;

        match direction {
            Direction::Forward => Self::new(self.position + magnitude, self.depth),
            Direction::Up => Self::new(self.position, self.depth - magnitude),
            Direction::Down => Self::new(self.position, self.depth + magnitude),
        }
    }

    fn compute_answer(self) -> i64 {
        self.position * self.depth
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct StateWithAim {
    state: State,
    aim: i64,
}

impl StateWithAim {
    pub fn new(position: i64, depth: i64, aim: i64) -> Self {
        Self {
            state: State::new(position, depth),
            aim,
        }
    }

    fn apply(self, command: Command) -> Self {
        let Command {
            direction,
            magnitude,
        } = command;

        match direction {
            Direction::Forward => Self::new(
                self.state.position + magnitude,
                self.state.depth + self.aim * magnitude,
                self.aim,
            ),
            Direction::Up => Self::new(self.state.position, self.state.depth, self.aim - magnitude),
            Direction::Down => {
                Self::new(self.state.position, self.state.depth, self.aim + magnitude)
            }
        }
    }

    fn compute_answer(self) -> i64 {
        self.state.compute_answer()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    let commands = many1(Command::parse)(input)?.1;

    let state = commands
        .iter()
        .fold(State::default(), |state, command| state.apply(*command));

    println!("Solution 1: {}", state.compute_answer());

    let state = commands
        .iter()
        .fold(StateWithAim::default(), |state, command| {
            state.apply(*command)
        });

    println!("Solution 2: {}", state.compute_answer());

    Ok(())
}
