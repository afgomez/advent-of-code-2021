use aoc::input::read_input;

fn parse_input(input: String) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

// Part 1
fn count_increments(numbers: &Vec<u32>) -> usize {
    numbers.windows(2).filter(|pair| pair[1] > pair[0]).count()
}

// Part 2
fn count_increments_triplets(numbers: &Vec<u32>) -> usize {
    let triplets: Vec<u32> = numbers.windows(3).map(|w| w.iter().sum()).collect();
    count_increments(&triplets)
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let numbers = parse_input(input);

    // Part 1
    // let increments = count_increments(&numbers);

    // Part 2
    let increments = count_increments_triplets(&numbers);

    println!("{}", increments);
    Ok(())
}
