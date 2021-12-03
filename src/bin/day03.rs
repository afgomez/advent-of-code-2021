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

fn calculate_life_support<T: AsRef<str>>(data: T) -> u32 {
    let data = data.as_ref();
    // AKA entry_length
    let bits_per_entry = data.find('\n').unwrap_or(0);

    let mut filtered: Vec<_> = data.lines().collect();

    // Calculate oxigen
    for i in 0..bits_per_entry {
        let (leading_ones, leading_zeroes): (Vec<_>, Vec<_>) =
            filtered.iter().partition(|&entry| {
                let bit = entry.chars().nth(i).unwrap();
                bit == '1'
            });

        if leading_ones.len() >= leading_zeroes.len() {
            filtered = leading_ones;
        } else {
            filtered = leading_zeroes;
        }

        if filtered.len() == 1 {
            break;
        }
    }

    let oxigen = u32::from_str_radix(filtered.get(0).unwrap(), 2).unwrap();

    // Calculate co2
    let mut filtered: Vec<_> = data.lines().collect();
    for i in 0..bits_per_entry {
        let (leading_ones, leading_zeroes): (Vec<_>, Vec<_>) =
            filtered.iter().partition(|&entry| {
                let bit = entry.chars().nth(i).unwrap();
                bit == '1'
            });

        if leading_ones.len() < leading_zeroes.len() {
            filtered = leading_ones;
        } else {
            filtered = leading_zeroes;
        }

        if filtered.len() == 1 {
            break;
        }
    }

    let co2 = u32::from_str_radix(filtered.get(0).unwrap(), 2).unwrap();

    oxigen * co2
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let consumption = calculate_consumption(&input);

    let life_support = calculate_life_support(&input);

    println!("{}", consumption);
    println!("{}", life_support);
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

    #[test]
    fn it_calculates_life_support() {
        assert_eq!(calculate_life_support(TEST_INPUT), 230);
    }
}
