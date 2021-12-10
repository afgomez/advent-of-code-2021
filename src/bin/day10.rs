#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};

use aoc::input::read_input;

lazy_static! {
    static ref PAIR_MAP: HashMap<char, char> =
        HashMap::from([('{', '}'), ('(', ')'), ('[', ']'), ('<', '>')]);
    static ref SCORE_MAP: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let nav = NavParser::from(input);

    println!("{}", nav.corruption());

    Ok(())
}

struct NavParser {
    lines: Vec<Line>,
}

impl NavParser {
    fn corruption(&self) -> u32 {
        self.lines
            .iter()
            .filter_map(|l| {
                if l.parse_state == ParseState::Corrupted {
                    Some(SCORE_MAP[&l.last_parsed])
                } else {
                    None
                }
            })
            .sum()
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
    raw_input: String,
    last_parsed: char,
    parse_state: ParseState,
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
                        raw_input: String::from(raw_input),
                        last_parsed: chr,
                        parse_state: ParseState::Corrupted,
                    };
                }
            }
        }

        Line {
            raw_input: String::from(raw_input),
            last_parsed: raw_input.chars().last().unwrap(),
            parse_state: if stack.len() > 0 {
                ParseState::Incomplete
            } else {
                ParseState::Complete
            },
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

    fn it_calculates_corrupted_score() {
        let nav = NavParser::from(TEST_INPUT);
        assert_eq!(nav.corruption(), 26397);
    }
}
