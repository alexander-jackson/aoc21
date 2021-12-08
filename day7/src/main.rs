use nom::{bytes::complete::tag, multi::separated_list1, IResult};

fn calculate_fuel_cost_for_position(i: i32, position: i32) -> i32 {
    let distance = (i - position).abs();
    (distance * (distance + 1)) / 2
}

fn compute_fuel_cost_for_all_positions(i: i32, positions: &[i32]) -> i32 {
    positions
        .iter()
        .map(|p| calculate_fuel_cost_for_position(i, *p))
        .sum()
}

#[derive(Debug)]
struct Crabs {
    positions: Vec<i32>,
}

impl Crabs {
    fn new(mut positions: Vec<i32>) -> Self {
        positions.sort();
        Self { positions }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        nom::combinator::map(
            separated_list1(tag(","), nom::character::complete::i32),
            Self::new,
        )(input)
    }

    fn calculate_minimal_fuel(&self) -> i32 {
        let len = self.positions.len();
        let midpoint = len / 2;

        let median = match len % 2 {
            0 => (self.positions[midpoint] + self.positions[midpoint - 1]) / 2,
            _ => self.positions[midpoint],
        };

        self.positions.iter().map(|p| (p - median).abs()).sum()
    }

    fn calculate_complex_minimal_fuel(&self) -> i32 {
        // For each position, compute the fuel cost
        let costs: Vec<_> = (self.positions[0]..self.positions[self.positions.len() - 1])
            .map(|i| compute_fuel_cost_for_all_positions(i, &self.positions))
            .collect();

        *costs.iter().min().unwrap()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    let crabs = Crabs::parse(input)?.1;

    let minimal_fuel = crabs.calculate_minimal_fuel();
    dbg!(&minimal_fuel);

    let complex_minimal_fuel = crabs.calculate_complex_minimal_fuel();
    dbg!(&complex_minimal_fuel);

    Ok(())
}
