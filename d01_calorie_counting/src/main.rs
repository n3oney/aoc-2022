fn main() {
    let input_bytes = include_bytes!("../input");
    let input = String::from_utf8_lossy(input_bytes);

    let mut top = [0u32; 3];

    let mut acc = 0;

    for line in input.split("\n") {
        match line.parse::<u32>() {
            Ok(value) => acc += value,
            Err(_) => {
                if acc > top[2] {
                    if acc > top[1] {
                        if acc > top[0] {
                            top.rotate_right(1);
                            top[0] = acc;
                        } else {
                            top[2] = top[1];
                            top[1] = acc;
                        }
                    } else {
                        top[2] = acc;
                    }
                }

                acc = 0;
            }
        }
    }

    println!("Most Calories carried by one Elf: {}", top[0]);
    println!(
        "Sum of Calories carried by all Elves: {}",
        top.iter().sum::<u32>()
    );
}
