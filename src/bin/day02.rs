use aoc::input::read_input;

#[derive(Debug)]
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

#[derive(Debug)]
struct Instruction(Direction, u32);

struct Submarine {
    x: u32,
    y: u32,
}

impl Submarine {
    fn new() -> Self {
        Submarine { x: 0, y: 0 }
    }

    fn mv(&mut self, instruction: Instruction) {
        let (direction, amount) = (instruction.0, instruction.1);
        match direction {
            Direction::Forward => self.x += amount,
            Direction::Down => self.y += amount,
            Direction::Up => self.y -= amount,
        }
    }
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
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

    println!("{}", sub.x * sub.y);

    Ok(())
}
