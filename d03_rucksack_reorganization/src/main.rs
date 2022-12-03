use std::collections::{HashMap, HashSet};

fn char_to_priority(char: &char) -> u32 {
    let int = *char as u32;

    if int < 91 {
        // Uppercase letters.
        int - 38
    } else {
        // Lowercase letters
        int - 96
    }
}

fn main() {
    let input_bytes = include_bytes!("../input");
    let input = String::from_utf8_lossy(input_bytes);

    let mut p1_sum = 0;

    'rucksack_loop: for rucksack in input.split("\n") {
        let mut set = HashSet::new();

        let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

        for char in compartment1.chars() {
            set.insert(char);
        }

        for char in compartment2.chars() {
            if set.contains(&char) {
                p1_sum += char_to_priority(&char);
                continue 'rucksack_loop;
            }
        }
    }

    let mut p2_sum = 0;

    let mut map: HashMap<char, u8> = HashMap::new();

    'rucksack_loop: for (i, rucksack) in input.split("\n").enumerate() {
        if i % 3 == 0 {
            map.clear();
        }

        let mut set = HashSet::new();

        for char in rucksack.chars() {
            if set.insert(char) {
                if let Some(old_value) = map.insert(char, 1) {
                    if old_value == 2 {
                        println!(
                            "The badge is {} with prio {}",
                            char,
                            char_to_priority(&char)
                        );
                        p2_sum += char_to_priority(&char);
                        continue 'rucksack_loop;
                    } else {
                        map.insert(char, 2);
                    }
                }
            }
        }
    }

    println!("Sum of priorities for part 1: {}", p1_sum);
    println!("Sum of priorities for part 2: {}", p2_sum);
}
