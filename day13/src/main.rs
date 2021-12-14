use std::collections::HashSet;
use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Copy, Clone, Debug)]
enum Direction {
    X,
    Y,
}

impl Direction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(alt((tag("x"), tag("y"))), |v| {
            if v == "x" {
                Self::X
            } else {
                Self::Y
            }
        })(input)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Dot {
    x: i32,
    y: i32,
}

impl Dot {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                nom::character::complete::i32,
                tag(","),
                nom::character::complete::i32,
            ),
            |(x, y)| Self { x, y },
        )(input)
    }
}

#[derive(Clone, Debug)]
struct Paper {
    dots: HashSet<Dot>,
}

impl Paper {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, Dot::parse), |dots| Self {
            dots: dots.into_iter().collect(),
        })(input)
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    direction: Direction,
    position: i32,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(
                tag("fold along "),
                separated_pair(Direction::parse, tag("="), nom::character::complete::i32),
            ),
            |(direction, position)| Self {
                direction,
                position,
            },
        )(input)
    }

    fn fold_with_horizontal_line(&self, paper: &Paper) -> Paper {
        let dots = paper
            .dots
            .iter()
            .copied()
            .map(|Dot { x, y }| {
                if y < self.position {
                    Dot { x, y }
                } else {
                    let dist = y - self.position;
                    Dot {
                        x,
                        y: self.position - dist,
                    }
                }
            })
            .collect();

        Paper { dots }
    }

    fn fold_with_vertical_line(&self, paper: &Paper) -> Paper {
        let dots = paper
            .dots
            .iter()
            .copied()
            .map(|Dot { x, y }| {
                if x < self.position {
                    Dot { x, y }
                } else {
                    let dist = x - self.position;
                    Dot {
                        x: self.position - dist,
                        y,
                    }
                }
            })
            .collect();

        Paper { dots }
    }

    fn fold(&self, paper: &Paper) -> Paper {
        match self.direction {
            Direction::X => self.fold_with_vertical_line(paper),
            Direction::Y => self.fold_with_horizontal_line(paper),
        }
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Find the max values in each dimension
        let (mx, my) = self
            .dots
            .iter()
            .fold((0, 0), |(cx, cy), Dot { x, y }| (cx.max(*x), cy.max(*y)));

        for y in 0..=my {
            for x in 0..=mx {
                let dot = Dot { x, y };
                let c = self.dots.contains(&dot).then(|| '#').unwrap_or('.');

                write!(f, "{} ", c)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Input {
    paper: Paper,
    instructions: Vec<Instruction>,
}

impl Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                Paper::parse,
                tuple((newline, newline)),
                separated_list1(newline, Instruction::parse),
            ),
            |(paper, instructions)| Self {
                paper,
                instructions,
            },
        )(input)
    }

    fn fold_first(&self) -> Paper {
        self.instructions[0].fold(&self.paper)
    }

    fn fold_all(&self) -> Paper {
        self.instructions
            .iter()
            .fold(self.paper.clone(), |p, i| i.fold(&p))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = include_str!("../input.txt");
    let input = Input::parse(content)?.1;

    let folded = input.fold_first();
    println!("Part 1 solution: {}", folded.dots.len());

    let compressed = input.fold_all();
    println!("{}", compressed);

    Ok(())
}
