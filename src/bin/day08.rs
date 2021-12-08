use std::collections::{HashMap, HashSet};

use aoc::input::read_input;

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let mut decoder = Decoder::from(input);

    println!("{}", decoder.count_known_numbers());

    decoder.decode();
    println!("{}", decoder.sum_values());

    Ok(())
}

struct Decoder {
    entries: Vec<Entry>,
}

impl Decoder {
    fn decode(&mut self) {
        for entry in &mut self.entries {
            (*entry).decode();
        }
    }

    fn count_known_numbers(&self) -> usize {
        self.entries
            .iter()
            .flat_map(|e| e.digits.iter().filter_map(|d| d.output))
            .count()
    }

    fn sum_values(&self) -> u64 {
        self.entries.iter().map(|e| e.value().unwrap()).sum()
    }
}

impl<T: AsRef<str>> From<T> for Decoder {
    fn from(input: T) -> Self {
        let entries = input.as_ref().lines().map(Entry::from).collect();
        Decoder { entries }
    }
}

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/

struct Entry {
    signals: Vec<String>,
    digits: Vec<Digit>,
    decode_map: HashMap<char, char>,
}

impl Entry {
    fn decode(&mut self) {
        let mut sorted_signals: Vec<HashSet<char>> = self
            .signals
            .iter()
            .map(|s| HashSet::from_iter(s.chars()))
            .collect();

        // Sort signals by their length. That would put in place the signals whose lenghs are known
        sorted_signals.sort_by_key(|s| s.len());

        if let [s1, s7, s4, rest @ .., _s8] = &sorted_signals[0..=9] {
            let with_five = &rest[0..=2];

            // The 'a' segment has to be the difference between s1 and s7
            let a = s7.difference(s1).nth(0).unwrap();
            self.decode_map.insert(*a, 'a');

            // s3 has 5 segments, and it's the only one whose difference with 7 leaves two segments
            let s3 = with_five
                .iter()
                .filter(|s| s.difference(s7).count() == 2)
                .nth(0)
                .unwrap();

            // Given s3 and s4
            // -> `b` is in 4, but not in 3
            // -> `g` is in 3, but not in 4, and it's not 'a'
            let b = s4.difference(s3).nth(0).unwrap();
            self.decode_map.insert(*b, 'b');

            let g = s3.difference(s4).filter(|&chr| *chr != *a).nth(0).unwrap();
            self.decode_map.insert(*g, 'g');

            // Given s3 and s7
            // -> `d` is in 3, but not in 7, and it's not `g`
            let d = s3.difference(s7).filter(|&chr| *chr != *g).nth(0).unwrap();
            self.decode_map.insert(*d, 'd');

            // s2, s3 and s5 have 5 segments.
            // -> we know s3
            // -> s2 has no `b`
            // -> s5 has to be the other one
            let s2 = with_five
                .iter()
                .filter(|&s| s != s3 && !s.contains(b))
                .nth(0)
                .unwrap();
            // Given s2 and s3,
            // -> `f` is in s3 but not in s2
            // -> `e` is in s2 but not in s3
            let f = s3.difference(s2).nth(0).unwrap();
            self.decode_map.insert(*f, 'f');

            let e = s2.difference(s3).nth(0).unwrap();
            self.decode_map.insert(*e, 'e');

            // `c` is segment in s1 that is not `f`
            let c = s1.iter().filter(|&chr| *chr != *f).nth(0).unwrap();
            self.decode_map.insert(*c, 'c');
        } else {
            panic!("Not enough signals")
        }

        for digit in &mut self.digits {
            (*digit).decode(&self.decode_map);
        }
    }

    fn value(&self) -> Option<u64> {
        let mut n: u64 = 0;
        for (exp, digit) in self.digits.iter().rev().enumerate() {
            if let Some(output) = digit.output {
                n += output as u64 * 10_u64.pow(exp as u32);
            } else {
                return None;
            }
        }

        if n == 0 {
            None
        } else {
            Some(n)
        }
    }
}

impl<T: AsRef<str>> From<T> for Entry {
    fn from(input: T) -> Self {
        let input = input.as_ref();
        if let [signals, digits] = input.split('|').collect::<Vec<&str>>()[0..2] {
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

            Entry {
                signals,
                digits,
                decode_map: HashMap::new(),
            }
        } else {
            panic!("Parse error: could not parse Entry {}", input);
        }
    }
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
            7 => Some(8),
            _ => None,
        }
    }

    fn parse_segments(segments: &str) -> Option<u8> {
        match segments {
            "abcefg" => Some(0),
            "cf" => Some(1),
            "acdeg" => Some(2),
            "acdfg" => Some(3),
            "bcdf" => Some(4),
            "abdfg" => Some(5),
            "abdefg" => Some(6),
            "acf" => Some(7),
            "abcdefg" => Some(8),
            "abcdfg" => Some(9),
            _ => None,
        }
    }

    fn decode(&mut self, decode_map: &HashMap<char, char>) {
        let mut segments: Vec<char> = self.raw_input.chars().map(|c| decode_map[&c]).collect();
        segments.sort_unstable();

        let segments = String::from_iter(segments);
        self.output = Self::parse_segments(&segments);
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

    #[test]
    fn it_returns_entry_value_after_decoding() {
        let mut entry = Entry::from(
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        );
        assert_eq!(entry.value(), None);

        entry.decode();
        assert_eq!(entry.value(), Some(1197));
    }

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

    fn it_sums_outputs() {
        let mut decoder = Decoder::from(TEST_INPUT);
        decoder.decode();

        assert_eq!(decoder.sum_values(), 61229);
    }
}
