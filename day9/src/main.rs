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

    fn no_point_lower(&self, x: usize, y: usize) -> bool {
        // Check surroundings if they exist
        let points = vec![
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
        ];

        let centre = self.values[x][y];

        for p in points {
            let (x, y) = match p {
                (Some(_), None) | (None, Some(_)) => continue,
                (Some(x), Some(y)) => (x, y),
                _ => unreachable!(),
            };

            if self.values.len() <= x {
                continue;
            }

            if self.values[x].len() <= y {
                continue;
            }

            // Access should be fine here
            if self.values[x][y] <= centre {
                return false;
            }
        }

        true
    }

    fn lowest_points(&self) -> Vec<u32> {
        let mut lowest = Vec::new();

        for x in 0..self.values.len() {
            for y in 0..self.values[x].len() {
                let v = self.values[x][y];
                let is_lowest = self.no_point_lower(x, y);

                if is_lowest {
                    lowest.push(v);
                }
            }
        }

        lowest
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    let depth_map = DepthMap::parse(input)?.1;

    let lowest_points = depth_map.lowest_points();
    let answer: u32 = lowest_points.iter().map(|p| p + 1).sum();
    dbg!(&answer);

    Ok(())
}
