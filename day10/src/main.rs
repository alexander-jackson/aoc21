fn is_opening_char(c: char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn get_matching_open_char(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn get_matching_closing_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn invalid_character_to_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn autocomplete_character_to_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn compute_autocomplete_score(input: String) -> u64 {
    input
        .chars()
        .fold(0, |acc, x| acc * 5 + autocomplete_character_to_score(x))
}

fn evaluate_line(input: &str) -> Result<String, char> {
    let mut stack = Vec::new();

    for c in input.chars() {
        if is_opening_char(c) {
            stack.push(c);
        } else {
            let opening_char = get_matching_open_char(c);

            // Get the top of the stack
            let top = stack.pop();

            // If it's none or the wrong character, return an error
            if top.map(|t| t != opening_char).unwrap_or_default() {
                return Err(c);
            }
        }
    }

    let completion_string = stack
        .into_iter()
        .rev()
        .map(get_matching_closing_char)
        .collect();

    Ok(completion_string)
}

fn main() {
    let input = include_str!("../input.txt");

    let score: u64 = input
        .lines()
        .map(evaluate_line)
        .filter_map(Result::err)
        .map(invalid_character_to_score)
        .sum();

    println!("Part 1: {}", score);

    // Score each string
    let mut scores: Vec<_> = input
        .lines()
        .map(evaluate_line)
        .filter_map(Result::ok)
        .map(compute_autocomplete_score)
        .collect();

    // Sort the values and get the middle one
    scores.sort();
    let middle_score = scores[scores.len() / 2];

    println!("Part 2: {}", middle_score);
}
