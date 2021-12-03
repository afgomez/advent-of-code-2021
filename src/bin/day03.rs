use aoc::input::read_input;

fn parse_input<T: AsRef<str>>(input: T) -> (Vec<String>, usize) {
    let entries: Vec<String> = input.as_ref().lines().map(str::to_owned).collect();
    let bit_count = if entries.len() == 0 {
        0
    } else {
        entries.get(0).unwrap().len()
    };

    (entries, bit_count)
}

fn calculate_consumption<T: AsRef<[String]>>(data: T, bit_count: usize) -> u32 {
    let data = data.as_ref();
    let len: u32 = data.len().try_into().unwrap();

    // Count the amount of '1' per bit
    let mut bit_counter: Vec<u32> = vec![0; bit_count];
    for entry in data.iter() {
        for (i, bit) in entry.chars().enumerate() {
            if bit == '1' {
                *(bit_counter.get_mut(i).unwrap()) += 1;
            }
        }
    }

    let gamma_rate_bits: Vec<u32> = bit_counter
        .into_iter()
        .map(|count| if count > len / 2 { 1 } else { 0 })
        .collect();

    // This can be collapsed with the previous iterator
    // Left in two for legibility
    let gamma_rate: u32 = gamma_rate_bits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |rate, (pos, bit)| rate + (bit << pos));

    let epsilon_rate = gamma_rate ^ 2_u32.pow(bit_count as u32) - 1;
    gamma_rate * epsilon_rate
}

fn calculate_life_support<T: AsRef<[String]>>(data: T, bit_count: usize) -> u32 {
    let mut filtered: Vec<String> = data.as_ref().to_vec();

    // Calculate oxigen
    for i in 0..bit_count {
        let (leading_ones, leading_zeroes): (Vec<_>, Vec<_>) =
            filtered.into_iter().partition(|entry| {
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
    let mut filtered = data.as_ref().to_vec();
    for i in 0..bit_count {
        let (leading_ones, leading_zeroes): (Vec<_>, Vec<_>) =
            filtered.into_iter().partition(|entry| {
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
    let (entries, bit_count) = parse_input(input);

    let consumption = calculate_consumption(&entries, bit_count);
    let life_support = calculate_life_support(&entries, bit_count);

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
    fn it_parses_the_input() {
        let (entries, bit_count) = parse_input(TEST_INPUT);
        assert_eq!(bit_count, 5);
        assert_eq!(
            entries,
            vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010"
            ]
        )
    }

    #[test]
    fn it_calcualtes_consumption() {
        let (entries, bit_count) = parse_input(TEST_INPUT);
        assert_eq!(calculate_consumption(entries, bit_count), 198);
    }

    #[test]
    fn it_calculates_life_support() {
        let (entries, bit_count) = parse_input(TEST_INPUT);
        assert_eq!(calculate_life_support(entries, bit_count), 230);
    }
}
