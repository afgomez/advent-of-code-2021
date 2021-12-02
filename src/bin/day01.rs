use aoc::input::read_input;

fn parse_input(input: String) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let numbers = parse_input(input);

    // part 2
    let numbers: Vec<u32> = numbers.windows(3).map(|w| w.iter().sum()).collect();

    let increments = numbers.windows(2).filter(|pair| pair[1] > pair[0]).count();

    println!("{}", increments);
    Ok(())
}
