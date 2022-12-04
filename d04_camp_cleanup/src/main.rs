struct Elf {
    start_string: String,
    end_string: String,
}

impl Default for Elf {
    fn default() -> Self {
        Elf {
            start_string: "".to_owned(),
            end_string: "".to_owned(),
        }
    }
}

impl Elf {
    fn to_nums(&self) -> (i32, i32) {
        (
            self.start_string.parse().unwrap(),
            self.end_string.parse().unwrap(),
        )
    }

    fn reset(&mut self) {
        self.start_string.clear();
        self.end_string.clear();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input");

    let raw_output = match std::env::var("ENV") {
        Ok(v) if v == "test" => true,
        _ => false,
    };

    let mut covered_count = 0;

    let mut overlapped_count = 0;

    let mut elf1 = Elf::default();
    let mut elf2 = Elf::default();

    let mut end = false;
    let mut second = false;

    for char in input.chars() {
        if char.is_digit(10) {
            if end {
                if second {
                    elf2.end_string.push(char);
                } else {
                    elf1.end_string.push(char);
                }
            } else {
                if second {
                    elf2.start_string.push(char);
                } else {
                    elf1.start_string.push(char);
                }
            }
        } else {
            match char {
                '-' => {
                    end = true;
                }
                ',' => {
                    end = false;
                    second = true;
                }
                '\n' => {
                    end = false;
                    second = false;

                    let (elf1_start, elf1_end) = elf1.to_nums();
                    let (elf2_start, elf2_end) = elf2.to_nums();

                    if (elf1_start <= elf2_start && elf1_end >= elf2_end)
                        || (elf2_start <= elf1_start && elf2_end >= elf1_end)
                    {
                        covered_count += 1;
                    }

                    if elf2_start <= elf1_end && elf1_start <= elf2_end {
                        overlapped_count += 1;
                    }

                    elf1.reset();
                    elf2.reset();
                }
                _ => unreachable!(),
            }
        }
    }

    if raw_output {
        println!("{}", covered_count);
        println!("{}", overlapped_count);
    } else {
        println!(
            "There are {} assignments fully contained in another.",
            covered_count
        );

        println!(
            "There are {} assignments overlapping another.",
            overlapped_count
        );
    }

    Ok(())
}
