use std::str::FromStr;

fn calculate_larger_pairs(values: &[u64]) -> usize {
    values.windows(2).filter(|v| v[0] < v[1]).count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<_> = std::fs::read_to_string("input.txt")?
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

    Ok(())
}
