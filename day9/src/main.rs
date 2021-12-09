use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct DepthMap {
    values: Vec<Vec<u32>>,
}

impl DepthMap {
    fn new(values: Vec<&str>) -> Self {
        let values = values
            .into_iter()
            .map(|r| r.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        Self { values }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        nom::combinator::map(separated_list1(newline, digit1), Self::new)(input)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../sample.txt");
    let depth_map = DepthMap::parse(input)?.1;
    dbg!(&depth_map);

    Ok(())
}
