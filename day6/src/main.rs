use nom::{bytes::complete::tag, combinator::map, multi::separated_list1, IResult};

#[derive(Debug)]
struct State {
    lanternfish: Vec<u32>,
}

impl State {
    fn new(lanternfish: Vec<u32>) -> Self {
        Self { lanternfish }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tag(","), nom::character::complete::u32),
            State::new,
        )(input)
    }

    fn tick(&self) -> Self {
        let mut next = Vec::new();

        for fish in &self.lanternfish {
            if *fish == 0 {
                next.extend_from_slice(&[6, 8]);
            } else {
                next.push(fish - 1);
            }
        }

        Self::new(next)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    let state = State::parse(input)?.1;
    dbg!(&state);

    let ticked = (0..80).fold(state, |s, _| s.tick());
    println!("Length: {}", ticked.lanternfish.len());

    Ok(())
}
