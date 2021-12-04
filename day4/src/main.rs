use nom::{
    bytes::complete::tag,
    character::{
        complete::multispace1,
        complete::{newline, space0},
    },
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, u32> {
    delimited(space0, nom::character::complete::u32, space0)(input)
}

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<u32>>,
}

impl Grid {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, values) = many1(terminated(many1(parse_number), newline))(input)?;

        Ok((input, Self { values }))
    }
}

#[derive(Debug)]
struct BingoFile {
    order: Vec<u32>,
    grids: Vec<Grid>,
}

impl BingoFile {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Parse out the order
        let (input, order) = terminated(
            separated_list1(tag(","), nom::character::complete::u32),
            multispace1,
        )(input)?;

        let (input, grids) = separated_list1(newline, Grid::parse)(input)?;

        Ok((input, Self { order, grids }))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../sample.txt");
    let (_, bingo_file) = BingoFile::parse(input)?;
    dbg!(&bingo_file);

    Ok(())
}
