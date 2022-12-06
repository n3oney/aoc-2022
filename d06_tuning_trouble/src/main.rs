use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input");

    let raw_output = match std::env::var("ENV") {
        Ok(v) if v == "test" => true,
        _ => false,
    };

    let mut chars: VecDeque<char> = VecDeque::new();

    let mut index = 0;

    let mut input_chars = input.chars();

    while let Some(input_char) = input_chars.next() {
        index += 1;

        match chars.iter().position(|&v| v == input_char) {
            None => {
                if chars.len() >= 3 {
                    break;
                }

                chars.push_back(input_char);
                if chars.len() > 3 {
                    chars.pop_front();
                }
            }
            Some(duplicate_position) => {
                for _ in 0..(duplicate_position + 1) {
                    chars.pop_front();
                }

                chars.push_back(input_char);
            }
        }
    }

    if raw_output {
        println!("{index}");
    } else {
        println!("{index} characters must be processed before the first start-of-packet.");
    }

    index = 0;

    input_chars = input.chars();

    chars = VecDeque::new();

    // Kind of dirty, but I don't have enough time to try to optimize it to one loop :(
    while let Some(input_char) = input_chars.next() {
        index += 1;

        match chars.iter().position(|&v| v == input_char) {
            None => {
                if chars.len() >= 13 {
                    break;
                }

                chars.push_back(input_char);
                if chars.len() > 13 {
                    chars.pop_front();
                }
            }
            Some(duplicate_position) => {
                for _ in 0..(duplicate_position + 1) {
                    chars.pop_front();
                }

                chars.push_back(input_char);
            }
        }
    }

    if raw_output {
        println!("{index}");
    } else {
        println!("{index} characters must be processed before the first start-of-message.");
    }
}
