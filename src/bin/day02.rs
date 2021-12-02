use aoc::input::read_input;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(direction: &str) -> Direction {
        match direction {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction(Direction, u32);

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
        let (direction, amount) = (instruction.0, instruction.1);
        match direction {
            Direction::Forward => {
                self.horizontal_pos += amount;
                self.depth += amount * self.aim;
            }
            Direction::Down => self.aim += amount,
            Direction::Up => self.aim -= amount,
        }
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Instruction> {
    input
        .as_ref()
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let direction: Direction = parts.next().unwrap().into();
            let amount = parts.next().unwrap().parse().unwrap();
            Instruction(direction, amount)
        })
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
        assert_eq!(Direction::from("up"), Direction::Up);
        assert_eq!(Direction::from("down"), Direction::Down);
        assert_eq!(Direction::from("forward"), Direction::Forward);
    }

    #[test]
    #[should_panic]
    fn direction_from_random_string_fails() {
        Direction::from("foobar");
    }

    const TEST_INPUT: &str = "forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2";

    #[test]
    fn parses_input() {
        let instructions = parse_input(TEST_INPUT);
        assert_eq!(
            instructions,
            vec![
                Instruction(Direction::Forward, 5),
                Instruction(Direction::Down, 5),
                Instruction(Direction::Forward, 8),
                Instruction(Direction::Up, 3),
                Instruction(Direction::Down, 8),
                Instruction(Direction::Forward, 2),
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
