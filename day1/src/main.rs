use std::str::FromStr;

fn calculate_larger_pairs(values: &[u64]) -> u64 {
    values
        .windows(2)
        .map(|v| (v[0] < v[1]).then(|| 1).unwrap_or_default())
        .sum()
}

fn main() {
    let input: Vec<_> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| u64::from_str(line).unwrap())
        .collect();

    let instances = calculate_larger_pairs(&input);

    println!("Part 1: {}", instances);

    let values: Vec<_> = input
        .windows(3)
        .map(|slice| slice.iter().sum::<u64>())
        .collect();

    let instances = calculate_larger_pairs(&values);

    println!("Part 2: {}", instances);
}
