use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space0},
    combinator::map,
    multi::{many_m_n, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Entry<'a> {
    signals: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            separated_pair(
                many_m_n(10, 10, delimited(space0, alpha1, space0)),
                tag("|"),
                many_m_n(4, 4, delimited(space0, alpha1, space0)),
            ),
            |(signals, outputs)| Self { signals, outputs },
        )(input)
    }
}

#[derive(Debug)]
struct Input<'a> {
    entries: Vec<Entry<'a>>,
}

impl<'a> Input<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(separated_list1(newline, Entry::parse), |entries| Self {
            entries,
        })(input)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::parse(include_str!("../sample.txt"))?.1;
    dbg!(&input);

    Ok(())
}
