use std::collections::HashSet;

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    fn from(start: Point, end: Point) -> Direction {
        if start.x != end.x {
            Direction::Horizontal
        } else {
            Direction::Vertical
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (x, y)) = separated_pair(
            nom::character::complete::u32,
            tag(","),
            nom::character::complete::u32,
        )(input)?;

        Ok((input, Self { x, y }))
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (start, end)) = separated_pair(Point::parse, tag(" -> "), Point::parse)(input)?;

        Ok((input, Self { start, end }))
    }

    fn points(&self) -> Vec<Point> {
        let direction = Direction::from(self.start, self.end);

        // Swap if needed
        let (start, end) = match direction {
            Direction::Horizontal => {
                if self.start.x > self.end.x {
                    (self.end, self.start)
                } else {
                    (self.start, self.end)
                }
            }
            Direction::Vertical => {
                if self.start.y > self.end.y {
                    (self.end, self.start)
                } else {
                    (self.start, self.end)
                }
            }
        };

        // Iterate based on the direction
        match direction {
            Direction::Horizontal => {
                let diff = end.x - start.x;
                (0..=diff)
                    .map(|i| Point {
                        x: start.x + i,
                        y: start.y,
                    })
                    .collect()
            }
            Direction::Vertical => {
                let diff = end.y - start.y;
                (0..=diff)
                    .map(|i| Point {
                        x: start.x,
                        y: start.y + i,
                    })
                    .collect()
            }
        }
    }
}

#[derive(Debug)]
struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, lines) = separated_list1(newline, Line::parse)(input)?;

        Ok((input, Self { lines }))
    }

    fn calculate_dangerous_point_count(&self) -> usize {
        let mut covered_points = HashSet::new();
        let mut dangerous_points = HashSet::new();

        self.lines.iter().for_each(|line| {
            for point in line.points() {
                println!("Line {:?} produced point {:?}", line, point);

                if !covered_points.insert(point) {
                    dangerous_points.insert(point);
                }
            }
        });

        dbg!(&dangerous_points);

        dangerous_points.len()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_, input) = Input::parse(include_str!("../sample.txt"))?;
    let dangerous_count = input.calculate_dangerous_point_count();
    dbg!(&dangerous_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_are_as_expected() {
        let line = Line {
            start: Point { x: 5, y: 5 },
            end: Point { x: 5, y: 8 },
        };
        let expected = vec![
            Point { x: 5, y: 5 },
            Point { x: 5, y: 6 },
            Point { x: 5, y: 7 },
            Point { x: 5, y: 8 },
        ];

        assert_eq!(line.points(), expected);

        let line = Line {
            start: Point { x: 5, y: 8 },
            end: Point { x: 5, y: 5 },
        };
        let expected = vec![
            Point { x: 5, y: 5 },
            Point { x: 5, y: 6 },
            Point { x: 5, y: 7 },
            Point { x: 5, y: 8 },
        ];

        assert_eq!(line.points(), expected);

        let line = Line {
            start: Point { x: 5, y: 5 },
            end: Point { x: 8, y: 5 },
        };
        let expected = vec![
            Point { x: 5, y: 5 },
            Point { x: 6, y: 5 },
            Point { x: 7, y: 5 },
            Point { x: 8, y: 5 },
        ];

        assert_eq!(line.points(), expected);

        let line = Line {
            start: Point { x: 8, y: 5 },
            end: Point { x: 5, y: 5 },
        };
        let expected = vec![
            Point { x: 5, y: 5 },
            Point { x: 6, y: 5 },
            Point { x: 7, y: 5 },
            Point { x: 8, y: 5 },
        ];

        assert_eq!(line.points(), expected);
    }
}
