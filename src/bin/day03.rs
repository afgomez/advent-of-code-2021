use aoc::input::read_input;

fn calculate_consumption<T: AsRef<str>>(data: T) -> u32 {
    let mut entry_count = 0;

    // AKA entry_length
    let bits_per_entry = data.as_ref().find('\n').unwrap_or(0);

    // Count the amount of '1' per bit
    let mut bit_counter: Vec<u32> = vec![0; bits_per_entry];
    for entry in data.as_ref().lines() {
        for (i, bit) in entry.chars().enumerate() {
            if bit == '1' {
                *(bit_counter.get_mut(i).unwrap()) += 1;
            }
        }
        entry_count += 1;
    }

    let gamma_rate_bits: Vec<u32> = bit_counter
        .into_iter()
        .map(|count| if count > entry_count / 2 { 1 } else { 0 })
        .collect();

    // This can be collapsed with the previous iterator
    // Left in two for legibility
    let gamma_rate: u32 = gamma_rate_bits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |rate, (pos, bit)| rate + (bit << pos));

    let epsilon_rate = gamma_rate ^ 2_u32.pow(bits_per_entry as u32) - 1;
    gamma_rate * epsilon_rate
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let consumption = calculate_consumption(input);

    println!("{}", consumption);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";

    #[test]
    fn it_calcualtes_consumption() {
        assert_eq!(calculate_consumption(TEST_INPUT), 198);
    }
}
