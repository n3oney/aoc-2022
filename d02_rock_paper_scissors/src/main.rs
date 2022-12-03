#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn points(&self) -> u8 {
        *self as u8
    }

    fn result_points(&self, other: &Self) -> u8 {
        if self == other {
            3
        } else {
            match self {
                Shape::Rock if other == &Shape::Paper => 0,
                Shape::Rock => 6,
                Shape::Paper if other == &Shape::Scissors => 0,
                Shape::Paper => 6,
                Shape::Scissors if other == &Shape::Rock => 0,
                Shape::Scissors => 6,
            }
        }
    }

    fn from_letter(letter: char) -> Self {
        match letter {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Invalid letter provided."),
        }
    }

    fn shape_with_result(&self, result: Shape) -> Self {
        match result {
            Shape::Paper => *self,

            Shape::Scissors if self == &Shape::Scissors => Shape::Rock,
            Shape::Scissors if self == &Shape::Rock => Shape::Paper,
            Shape::Scissors => Shape::Scissors,

            Shape::Rock if self == &Shape::Scissors => Shape::Paper,
            Shape::Rock if self == &Shape::Rock => Shape::Scissors,
            Shape::Rock => Shape::Rock,
        }
    }
}

fn main() {
    let input_bytes = include_bytes!("../input");
    let input = String::from_utf8_lossy(input_bytes);

    let mut score_p1: u32 = 0;
    let mut score_p2: u32 = 0;

    let mut enemy_shape: Option<Shape> = None;
    let mut my_shape: Option<Shape> = None;

    for char in input.chars() {
        if char == ' ' {
            continue;
        }

        if char == '\n' {
            if let (Some(mut my_shape), Some(enemy_shape)) = (my_shape, enemy_shape) {
                score_p1 += my_shape.points() as u32;
                score_p1 += my_shape.result_points(&enemy_shape) as u32;

                my_shape = enemy_shape.shape_with_result(my_shape);

                score_p2 += my_shape.points() as u32;
                score_p2 += my_shape.result_points(&enemy_shape) as u32;
            }

            enemy_shape = None;
            my_shape = None;
        } else {
            let shape = Shape::from_letter(char);
            if enemy_shape.is_none() {
                enemy_shape = Some(shape);
            } else {
                my_shape = Some(shape);
            }
        }
    }

    println!("Final score for part 1: {}", score_p1);
    println!("Final score for part 2: {}", score_p2);
}
