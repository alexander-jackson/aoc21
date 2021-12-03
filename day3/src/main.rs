use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Default)]
struct IndexCount {
    zeros: usize,
    ones: usize,
}

impl IndexCount {
    fn update(&mut self, value: char) {
        match value {
            '0' => self.zeros += 1,
            _ => self.ones += 1,
        }
    }

    fn resolve(self) -> usize {
        (self.zeros < self.ones).then(|| 1).unwrap_or_default()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let values: Vec<_> = input.lines().collect();

    // Find the most common in each position
    let mut counts: HashMap<usize, IndexCount> = HashMap::new();

    values.iter().for_each(|v| {
        // Update the mapping counts
        for (i, c) in v.chars().enumerate() {
            counts.entry(i).or_default().update(c);
        }
    });

    let width = values[0].len();

    let gamma: usize = (0..width)
        .map(|i| counts[&i].resolve() << (width - i - 1))
        .sum();

    let epsilon = (1 << width) - gamma - 1;

    println!("Part 1: {}", gamma * epsilon);
}
