#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};

use aoc::input::read_input;

lazy_static! {
    static ref PAIR_MAP: HashMap<char, char> =
        HashMap::from([('{', '}'), ('(', ')'), ('[', ']'), ('<', '>')]);
    static ref CORRUPT_SCORE_MAP: HashMap<char, u64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    static ref AUTOCOMPLETE_SCORE_MAP: HashMap<char, u64> =
        HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let nav = NavParser::from(input);

    println!("{}", nav.corruption_score());
    println!("{}", nav.completion_score());

    Ok(())
}

struct NavParser {
    lines: Vec<Line>,
}

impl NavParser {
    fn corruption_score(&self) -> u64 {
        self.lines
            .iter()
            .filter_map(|l| {
                if l.parse_state == ParseState::Corrupted {
                    Some(CORRUPT_SCORE_MAP[&l.last_parsed])
                } else {
                    None
                }
            })
            .sum()
    }

    fn completion_score(&self) -> u64 {
        let mut scores: Vec<u64> = self
            .lines
            .iter()
            .filter_map(|l| {
                if l.parse_state == ParseState::Incomplete {
                    Some(
                        l.still_open
                            .as_ref()
                            .unwrap()
                            .iter()
                            .rev()
                            .fold(0, |score, chr| score * 5 + AUTOCOMPLETE_SCORE_MAP[chr]),
                    )
                } else {
                    None
                }
            })
            .collect();

        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

impl<T: AsRef<str>> From<T> for NavParser {
    fn from(input: T) -> Self {
        let lines = input.as_ref().lines().map(Line::parse).collect();
        NavParser { lines }
    }
}

#[derive(PartialEq)]
enum ParseState {
    Complete,
    Incomplete,
    Corrupted,
}

struct Line {
    last_parsed: char,
    parse_state: ParseState,
    still_open: Option<Vec<char>>,
}

impl Line {
    fn parse<T: AsRef<str>>(raw_input: T) -> Self {
        let raw_input = raw_input.as_ref();
        let mut stack: Vec<char> = vec![];

        let opening_chars: HashSet<&char> = PAIR_MAP.keys().collect();

        for chr in raw_input.chars() {
            if opening_chars.contains(&chr) {
                stack.push(chr);
            } else if let Some(current_open) = stack.last() {
                if PAIR_MAP[current_open] == chr {
                    stack.pop();
                } else {
                    return Line {
                        last_parsed: chr,
                        parse_state: ParseState::Corrupted,
                        still_open: None, // It cannot be completed
                    };
                }
            }
        }

        let last_parsed = raw_input.chars().last().unwrap();

        if stack.len() > 0 {
            Line {
                last_parsed,
                parse_state: ParseState::Incomplete,
                still_open: Some(stack),
            }
        } else {
            Line {
                last_parsed,
                parse_state: ParseState::Complete,
                still_open: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn it_parses_lines() {
        let nav = NavParser::from(TEST_INPUT);
        assert_eq!(nav.lines.len(), 10);
    }

    #[test]
    fn it_calculates_corrupted_score() {
        let nav = NavParser::from(TEST_INPUT);
        assert_eq!(nav.corruption_score(), 26397);
    }

    #[test]
    fn it_calculates_autocomplete_score() {
        let nav = NavParser::from(TEST_INPUT);
        assert_eq!(nav.completion_score(), 288957);
    }
}
