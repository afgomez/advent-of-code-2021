use aoc::input::read_input;

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let decoder = Decoder::from(input);

    println!("{}", decoder.count_known_numbers());
    Ok(())
}

struct Decoder {
    entries: Vec<Entry>,
}

impl Decoder {
    fn count_known_numbers(&self) -> usize {
        self.entries
            .iter()
            .flat_map(|e| e.digits.iter().filter_map(|d| d.output))
            .count()
    }
}

impl<T: AsRef<str>> From<T> for Decoder {
    fn from(input: T) -> Self {
        let entries = input
            .as_ref()
            .lines()
            .map(|l| {
                if let [signals, digits] = l.split('|').collect::<Vec<&str>>()[0..2] {
                    let signals = signals
                        .trim()
                        .split_whitespace()
                        .map(String::from)
                        .collect::<Vec<String>>();

                    let digits = digits
                        .trim()
                        .split_whitespace()
                        .map(Digit::from)
                        .collect::<Vec<Digit>>();

                    Entry { signals, digits }
                } else {
                    panic!("Parse error: could not parse line {}", l);
                }
            })
            .collect();
        Decoder { entries }
    }
}

struct Entry {
    signals: Vec<String>,
    digits: Vec<Digit>,
}

#[derive(Debug)]
struct Digit {
    raw_input: String,
    output: Option<u8>,
}

impl Digit {
    fn guess_output(input: &str) -> Option<u8> {
        let segment_count = input.len();
        match segment_count {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(7),
            _ => None,
        }
    }
}

impl<T: AsRef<str>> From<T> for Digit {
    fn from(raw_input: T) -> Self {
        let raw_input = raw_input.as_ref();
        let output = Self::guess_output(raw_input);

        Digit {
            raw_input: raw_input.into(),
            output,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn it_parses_entries() {
        let decoder = Decoder::from(TEST_INPUT);

        assert_eq!(decoder.entries.len(), 10);

        let first_entry = &decoder.entries[0];
        assert_eq!(first_entry.signals.len(), 10);
        assert_eq!(first_entry.digits.len(), 4);
    }

    #[test]
    fn it_returns_known_numbers() {
        let decoder = Decoder::from(TEST_INPUT);
        assert_eq!(decoder.count_known_numbers(), 26);
    }
}
