use aoc::input::read_input;

struct Telemetry {
    entries: Vec<String>,
    entry_len: usize,
}

impl Telemetry {
    fn new() -> Self {
        Telemetry {
            entries: vec![],
            entry_len: 0,
        }
    }

    fn from<T: AsRef<str>>(input: T) -> Self {
        let mut telemetry = Telemetry::new();
        telemetry.parse(input);
        telemetry
    }

    fn parse<T: AsRef<str>>(&mut self, input: T) {
        let entries: Vec<String> = input.as_ref().lines().map(str::to_owned).collect();
        let entry_len = if entries.len() == 0 {
            0
        } else {
            entries.get(0).unwrap().len()
        };

        self.entries = entries;
        self.entry_len = entry_len;
    }

    fn consumption(&self) -> usize {
        let len = self.entries.len();

        let mut ones_per_position: Vec<usize> = vec![0; self.entry_len];
        for entry in &self.entries {
            for (pos, bit) in entry.chars().enumerate() {
                if bit == '1' {
                    *(ones_per_position.get_mut(pos).unwrap()) += 1;
                }
            }
        }

        let most_common_bit_per_position: String = ones_per_position
            .into_iter()
            .map(|no_ones| if no_ones > len / 2 { '1' } else { '0' })
            .collect();

        // `gamma` is the most common bit per position
        let gamma_rate = usize::from_str_radix(&most_common_bit_per_position, 2).unwrap();

        // `epsilon` is the least common bit per position, that is, the bit inverse of gamma
        let mask = 2_usize.pow(self.entry_len.try_into().unwrap()) - 1; // 0b1000 - 1 = 0b111, for a `entry_len` of 3
        let epsilon_rate = gamma_rate ^ mask;

        gamma_rate * epsilon_rate
    }

    fn life_support(&self) -> usize {
        let mut entries = self.entries.to_vec();
        for pos in 0..self.entry_len {
            let (entries_with_zero, entries_with_one) = partition_by_bit_at_pos(&entries, pos);

            entries = if entries_with_one.len() >= entries_with_zero.len() {
                entries_with_one
            } else {
                entries_with_zero
            };

            if entries.len() == 1 {
                break;
            }
        }
        let o2 = usize::from_str_radix(&entries[0], 2).unwrap();

        let mut entries = self.entries.to_vec();
        for pos in 0..self.entry_len {
            let (entries_with_zero, entries_with_one) = partition_by_bit_at_pos(&entries, pos);

            entries = if entries_with_one.len() < entries_with_zero.len() {
                entries_with_one
            } else {
                entries_with_zero
            };

            if entries.len() == 1 {
                break;
            }
        }

        let co2 = usize::from_str_radix(&entries[0], 2).unwrap();

        o2 * co2
    }
}

fn partition_by_bit_at_pos(entries: &Vec<String>, pos: usize) -> (Vec<String>, Vec<String>) {
    entries
        .to_vec()
        .into_iter()
        .partition(|entry| entry.chars().nth(pos).unwrap() == '0')
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let telemetry = Telemetry::from(&input);

    let consumption = telemetry.consumption();
    let life_support = telemetry.life_support();

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
        let telemetry = Telemetry::from(TEST_INPUT);

        assert_eq!(telemetry.entry_len, 5);
        assert_eq!(
            telemetry.entries,
            vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010"
            ]
        )
    }

    #[test]
    fn it_calcualtes_consumption() {
        let telemetry = Telemetry::from(TEST_INPUT);
        assert_eq!(telemetry.consumption(), 198);
    }

    #[test]
    fn it_calculates_life_support() {
        let telemetry = Telemetry::from(TEST_INPUT);
        assert_eq!(telemetry.life_support(), 230);
    }
}
