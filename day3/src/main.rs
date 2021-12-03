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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BitCriteria {
    MostCommon,
    LeastCommon,
}

fn resolve_most_common(zeros: usize, ones: usize) -> char {
    match zeros.cmp(&ones) {
        std::cmp::Ordering::Greater => '0',
        _ => '1',
    }
}

fn resolve_least_common(zeros: usize, ones: usize) -> char {
    match ones.cmp(&zeros) {
        std::cmp::Ordering::Less => '1',
        _ => '0',
    }
}

impl BitCriteria {
    fn resolve(&self, zeros: usize, ones: usize) -> char {
        match *self {
            Self::MostCommon => resolve_most_common(zeros, ones),
            Self::LeastCommon => resolve_least_common(zeros, ones),
        }
    }
}

fn filter<'a, 'b>(values: Vec<&'b str>, criteria: BitCriteria) -> usize
where
    'a: 'b,
{
    usize::from_str_radix(
        (0..values[0].len()).fold(values, |v, i| filter_by_index(v, criteria, i))[0],
        2,
    )
    .unwrap()
}

fn filter_by_index<'a>(values: Vec<&'a str>, criteria: BitCriteria, index: usize) -> Vec<&'a str> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for v in &values {
        let c = v.chars().skip(index).next().unwrap();
        *counts.entry(c).or_default() += 1;
    }

    let zeros = counts.get(&'0').copied().unwrap_or_default();
    let ones = counts.get(&'1').copied().unwrap_or_default();

    let bit = criteria.resolve(zeros, ones);

    let filtered: Vec<_> = values
        .iter()
        .filter(|v| v.chars().skip(index).next().unwrap() == bit)
        .copied()
        .collect();

    match filtered.len() {
        0 => values,
        _ => filtered,
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

    let oxygen = filter(values.clone(), BitCriteria::MostCommon);
    let carbon_dioxide = filter(values.clone(), BitCriteria::LeastCommon);

    println!("Part 2: {}", oxygen * carbon_dioxide);
}
