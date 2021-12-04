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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Marked,
    Unmarked,
}

#[derive(Debug)]
struct GridValue {
    value: u32,
    state: State,
}

impl GridValue {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, value) = delimited(space0, nom::character::complete::u32, space0)(input)?;

        Ok((
            input,
            Self {
                value,
                state: State::Unmarked,
            },
        ))
    }

    fn mark(&mut self, value: u32) {
        if self.value == value {
            self.state = State::Marked;
        }
    }
}

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<GridValue>>,
}

impl Grid {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, values) = many1(terminated(many1(GridValue::parse), newline))(input)?;

        Ok((input, Self { values }))
    }

    fn is_complete(&self) -> bool {
        // Check rows first
        let complete_row = self
            .values
            .iter()
            .any(|row| row.iter().all(|v| v.state == State::Marked));

        if complete_row {
            return true;
        }

        let rows = self.values.len();
        let columns = self.values[0].len();

        let complete_column = (0..columns).any(|i| {
            (0..rows)
                .map(|j| self.values[j][i].state)
                .all(|v| v == State::Marked)
        });

        complete_column
    }

    fn unmarked_sum(&self) -> u32 {
        self.values
            .iter()
            .flat_map(|v| v.iter())
            .filter_map(|v| v.state.eq(&State::Unmarked).then(|| v.value))
            .sum()
    }

    fn mark(&mut self, value: u32) -> Option<u32> {
        // Find the value and mark it
        self.values
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| v.mark(value)));

        self.is_complete().then(|| value * self.unmarked_sum())
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

    fn find_winner(&mut self) -> Option<u32> {
        for value in &self.order {
            for grid in &mut self.grids {
                if let Some(v) = grid.mark(*value) {
                    return Some(v);
                }
            }
        }

        None
    }

    fn find_loser(&mut self) -> Option<u32> {
        let mut status: Vec<bool> = (0..self.grids.len()).map(|_| false).collect();

        for value in &self.order {
            for (i, grid) in self.grids.iter_mut().enumerate() {
                if let Some(v) = grid.mark(*value) {
                    status[i] = true;

                    if status.iter().all(|v| *v) {
                        return Some(v);
                    }
                }
            }
        }

        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");
    let (_, mut bingo_file) = BingoFile::parse(input)?;

    let result = bingo_file.find_winner();
    dbg!(&result);

    let (_, mut bingo_file) = BingoFile::parse(input)?;

    let result = bingo_file.find_loser();
    dbg!(&result);

    Ok(())
}
