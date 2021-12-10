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

fn character_to_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let score: u32 = input
        .lines()
        .filter_map(find_incorrect_closing_character)
        .map(character_to_score)
        .sum();

    println!("Part 1: {}", score);
}
