use std::{error::Error, str::Chars};

fn parse_stacks(input: &mut Chars) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut last_char: Option<char> = None;

    let mut stack_index = 0usize;

    while let Some(char) = input.next() {
        match char {
            '[' => (),
            ' ' => {
                if last_char == Some(' ') {
                    input.nth(1);
                    stack_index += 1;
                    last_char = None;
                    continue;
                }
            }
            '\n' => {
                if last_char == Some('\n') {
                    for stack in &mut stacks {
                        stack.reverse();
                    }
                    return Ok(stacks);
                }

                stack_index = 0;
            }
            ']' => (),
            ch if ch.is_digit(10) => {
                for stack in &mut stacks {
                    stack.reverse();
                }

                return Ok(stacks);
            }
            id => {
                let stack = match stacks.get_mut(stack_index) {
                    Some(v) => v,
                    None => {
                        let offset = stack_index - stacks.len() + 1;
                        for _ in 0..offset {
                            stacks.push(Vec::new());
                        }
                        stacks.last_mut().unwrap()
                    }
                };

                stack.push(id);

                stack_index += 1;
            }
        }

        last_char = Some(char);
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    Ok(stacks)
}

#[derive(Clone, Copy)]
enum Reading {
    Nothing,
    Count,
    FromStack,
    ToStack,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input");

    let raw_output = match std::env::var("ENV") {
        Ok(v) if v == "test" => true,
        _ => false,
    };

    let mut chars_iter = input.chars();

    let mut stacks = parse_stacks(&mut chars_iter)?;
    let mut part2_stacks = stacks.clone();

    let commands_vec = chars_iter.skip_while(|c| c != &'m').collect::<Vec<_>>();

    let mut reading = Reading::Nothing;

    let mut reading_string = "".to_owned();
    let mut count = 0u32;
    let mut from_stack = 0usize;

    let mut chars_iter = commands_vec.iter();

    while let Some(char) = chars_iter.next() {
        match char {
            'e' => {
                chars_iter.next();
                reading = Reading::Count;
            }
            'r' => {
                chars_iter.nth(2);
                reading = Reading::FromStack;
            }
            't' => {
                chars_iter.nth(1);
                reading = Reading::ToStack;
            }
            ' ' => match reading {
                Reading::Count => {
                    count = reading_string.parse()?;
                    reading_string.clear();
                }
                Reading::FromStack => {
                    from_stack = reading_string.parse()?;
                    reading_string.clear();
                }
                _ => (),
            },
            digit if digit.is_digit(10) => {
                reading_string.push(*digit);
            }
            '\n' => {
                let to_stack: usize = reading_string.parse()?;
                reading_string.clear();

                let mut moved_values = Vec::new();

                for _ in 0..count {
                    let val = stacks[from_stack - 1].pop().unwrap();
                    moved_values.push(part2_stacks[from_stack - 1].pop().unwrap());

                    stacks[to_stack - 1].push(val);
                }

                moved_values.reverse();
                part2_stacks[to_stack - 1].append(&mut moved_values);
            }
            _ => (),
        }
    }

    let p1_result = stacks
        .iter()
        .fold("".to_owned(), |acc, x| acc + &x.last().unwrap().to_string());

    let p2_result = part2_stacks
        .iter()
        .fold("".to_owned(), |acc, x| acc + &x.last().unwrap().to_string());

    if raw_output {
        println!("{p1_result}");
        println!("{p2_result}");
    } else {
        println!("The final arrangement of the CrateMover 9000 is {p1_result}");
        println!("The final arrangement of the CrateMover 9001 is {p2_result}");
    }

    Ok(())
}
