use aoc::input::read_input;

#[derive(Debug, PartialEq)]
enum Instruction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Instruction {
        if let Some((direction, amount)) = input.trim().split_once(' ') {
            let amount = amount.parse().unwrap();
            match direction {
                "forward" => Self::Forward(amount),
                "up" => Self::Up(amount),
                "down" => Self::Down(amount),
                _ => unreachable!(),
            }
        } else {
            panic!("Unable to parse instruction")
        }
    }
}

struct Submarine {
    horizontal_pos: u32,
    depth: u32,
    aim: u32,
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            horizontal_pos: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn mv(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(amount) => {
                self.horizontal_pos += amount;
                self.depth += amount * self.aim;
            }
            Instruction::Down(amount) => self.aim += amount,
            Instruction::Up(amount) => self.aim -= amount,
        }
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Instruction> {
    input
        .as_ref()
        .lines()
        .map(|line| Instruction::from(line))
        .collect()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let instructions: Vec<Instruction> = parse_input(input);

    let mut sub = Submarine::new();

    for instruction in instructions {
        sub.mv(instruction)
    }

    println!("{}", sub.horizontal_pos * sub.depth);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_from_string() {
        assert_eq!(Instruction::from("up 5"), Instruction::Up(5));
        assert_eq!(Instruction::from("down 2"), Instruction::Down(2));
        assert_eq!(Instruction::from("forward 3"), Instruction::Forward(3));
    }

    #[test]
    #[should_panic]
    fn direction_from_random_string_fails() {
        let _ = Instruction::from("foobar");
    }

    const TEST_INPUT: &str = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";

    #[test]
    fn parses_input() {
        let instructions = parse_input(TEST_INPUT);
        assert_eq!(
            instructions,
            vec![
                Instruction::Forward(5),
                Instruction::Down(5),
                Instruction::Forward(8),
                Instruction::Up(3),
                Instruction::Down(8),
                Instruction::Forward(2),
            ]
        );
    }

    #[test]
    fn moves_the_submarine() {
        let instructions = parse_input(TEST_INPUT);
        let mut submarine = Submarine::new();

        for instruction in instructions {
            submarine.mv(instruction);
        }

        assert_eq!(submarine.horizontal_pos, 15);
        assert_eq!(submarine.depth, 60);
    }
}
