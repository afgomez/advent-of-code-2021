use aoc::input::read_input;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<u32> {
    input.as_ref().lines().map(|l| l.parse().unwrap()).collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "199\n\
        200\n\
        208\n\
        210\n\
        200\n\
        207\n\
        240\n\
        269\n\
        260\n\
        263";

    #[test]
    fn input_parses_correctly() {
        println!("{}", TEST_INPUT);
        let numbers = parse_input(TEST_INPUT);
        assert_eq!(
            numbers,
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        );
    }

    #[test]
    fn counts_increments() {
        let numbers = parse_input(TEST_INPUT);
        let increments = count_increments(&numbers);
        assert_eq!(increments, 7);
    }

    #[test]
    fn counts_increments_in_triplets() {
        let numbers = parse_input(TEST_INPUT);
        let increments = count_increments_triplets(&numbers);
        assert_eq!(increments, 5);
    }
}
