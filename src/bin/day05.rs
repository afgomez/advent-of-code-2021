use aoc::input::read_input;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let vent_field = VentField::from(input);

    println!("{}", vent_field.overlaps());

    Ok(())
}

type Point = (usize, usize);

struct VentField {
    lines: Vec<Line>,
}

impl<T: AsRef<str>> From<T> for VentField {
    fn from(input: T) -> Self {
        let lines: Vec<Line> = input.as_ref().lines().map(|l| l.into()).collect();
        VentField::new(lines)
    }
}

impl VentField {
    fn new(lines: Vec<Line>) -> Self {
        VentField { lines }
    }

    fn overlaps(&self) -> usize {
        let mut field: HashMap<Point, u32> = HashMap::new();

        for line in &self.lines {
            for point in line.points() {
                let count = field.entry(point).or_insert(0);
                *count += 1;
            }
        }

        field.values().filter(|&n| *n > 1).count()
    }
}

#[derive(Debug, PartialEq)]
struct Line(Point, Point);

impl Line {
    fn is_horizontal(&self) -> bool {
        self.0 .0 == self.1 .0
    }

    fn is_vertical(&self) -> bool {
        self.0 .1 == self.1 .1
    }

    fn is_diagonal(&self) -> bool {
        let x_diff = self.0 .0 as i32 - self.1 .0 as i32;
        let y_diff = self.0 .1 as i32 - self.1 .1 as i32;

        x_diff.abs() == y_diff.abs()
    }

    // TODO Make this return an iterator instead
    fn points(&self) -> Points {
        if !self.is_vertical() && !self.is_horizontal() && !self.is_diagonal() {
            return Points::empty();
        }

        Points::from(self)
    }
}

impl<T: AsRef<str> + Debug> From<T> for Line {
    fn from(line: T) -> Line {
        let pairs: Vec<&str> = line.as_ref().split(" -> ").collect();

        let (start, end) = match pairs[..] {
            [a, b] => (parse_point(a), parse_point(b)),
            _ => panic!("ParseError: Cannot parse line: {:?}", &line),
        };

        Line(start, end)
    }
}

struct Points {
    next: Option<Point>,
    end: Point,
}

impl Points {
    fn empty() -> Self {
        Points {
            next: None,
            end: (0, 0),
        }
    }
}

impl From<&Line> for Points {
    fn from(line: &Line) -> Points {
        Points {
            next: Some(line.0),
            end: line.1,
        }
    }
}

impl Iterator for Points {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next {
            if current == self.end {
                self.next.take()
            } else {
                let (mut x, mut y) = current;

                x = match x.cmp(&self.end.0) {
                    Ordering::Greater => x - 1,
                    Ordering::Less => x + 1,
                    Ordering::Equal => x,
                };

                y = match y.cmp(&self.end.1) {
                    Ordering::Greater => y - 1,
                    Ordering::Less => y + 1,
                    Ordering::Equal => y,
                };
                self.next = Some((x, y));

                Some(current)
            }
        } else {
            None
        }
    }
}

fn parse_point(point: &str) -> Point {
    let points: Vec<usize> = point
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();
    match points[..] {
        [x, y] => (x, y),
        _ => panic!("ParseError: Cannot parse point: {}", point),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_points() {
        assert_eq!(parse_point("0,1"), (0, 1))
    }

    #[test]
    fn it_parses_lines() {
        let line = Line::from("0,1 -> 0,3");
        assert_eq!(line, Line((0, 1), (0, 3)));
    }

    #[test]
    fn it_detects_horizontal_lines() {
        let line = Line((0, 0), (0, 9));
        assert!(line.is_horizontal());

        let line = Line((9, 0), (0, 0));
        assert!(!line.is_horizontal());
    }

    #[test]
    fn it_detects_vertical_lines() {
        let line = Line((0, 0), (0, 9));
        assert!(!line.is_vertical());

        let line = Line((9, 0), (0, 0));
        assert!(line.is_vertical());
    }

    #[test]
    fn it_detects_diagonal_lines() {
        let line = Line((0, 0), (0, 9));
        assert!(!line.is_diagonal());

        let line = Line((0, 0), (9, 9));
        assert!(line.is_diagonal());

        let line = Line((9, 9), (0, 0));
        assert!(line.is_diagonal());

        let line = Line((9, 9), (0, 1));
        assert!(!line.is_diagonal());
    }

    #[test]
    fn it_correctly_makes_points() {
        let line = Line((0, 1), (0, 3));
        let points: Vec<_> = line.points().collect();
        assert_eq!(points, [(0, 1), (0, 2), (0, 3)]);

        let line = Line((1, 0), (3, 0));
        let points: Vec<_> = line.points().collect();
        assert_eq!(points, [(1, 0), (2, 0), (3, 0)]);

        let line = Line((1, 0), (2, 1));
        let points: Vec<_> = line.points().collect();
        assert_eq!(points, [(1, 0), (2, 1)]);

        let line = Line((3, 1), (1, 3));
        let points: Vec<_> = line.points().collect();
        assert_eq!(points, [(3, 1), (2, 2), (1, 3)]);

        let line = Line((1, 0), (3, 4));
        let points: Vec<_> = line.points().collect();
        assert_eq!(points, []);
    }

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn it_calculates_line_overlaps() {
        let vent_field = VentField::from(TEST_INPUT);
        assert_eq!(vent_field.overlaps(), 12);
    }
}
