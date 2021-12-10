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

fn find_incorrect_closing_character(input: &str) -> Option<char> {
    let mut stack = Vec::new();

    for c in input.chars() {
        if is_opening_char(c) {
            stack.push(c);
        } else {
            let opening_char = get_matching_open_char(c);

            // Get the top of the stack
            let top = stack.pop();

            match top {
                Some(t) if t == opening_char => (),
                _ => return Some(c),
            }
        }
    }

    None
}

fn character_to_score(c: char) -> u64 {
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

fn find_completion_string(input: &str) -> String {
    let mut stack = Vec::new();

    for c in input.chars() {
        if is_opening_char(c) {
            stack.push(c);
        } else {
            let opening_char = get_matching_open_char(c);

            // Get the top of the stack
            let top = stack.pop();

            match top {
                Some(t) if t == opening_char => (),
                _ => panic!("String should not be invalid"),
            }
        }
    }

    stack
        .into_iter()
        .rev()
        .map(get_matching_closing_char)
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");

    let score: u64 = input
        .lines()
        .filter_map(find_incorrect_closing_character)
        .map(character_to_score)
        .sum();

    println!("Part 1: {}", score);

    // Score each string
    let mut scores: Vec<_> = input
        .lines()
        .filter(|l| find_incorrect_closing_character(l).is_none())
        .map(find_completion_string)
        .map(compute_autocomplete_score)
        .collect();

    // Sort the values and get the middle one
    scores.sort();
    let middle_score = scores[scores.len() / 2];

    println!("Part 2: {}", middle_score);
}
