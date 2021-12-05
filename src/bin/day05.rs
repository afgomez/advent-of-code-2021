use aoc::input::read_input;
use std::fmt::Debug;

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let vent_field = VentField::from(input);

    println!("{}", vent_field.overlaps());

    Ok(())
}

struct VentField {
    lines: Vec<Line>,
    dimensions: (usize, usize),
}

impl<T: AsRef<str>> From<T> for VentField {
    fn from(input: T) -> Self {
        let lines: Vec<Line> = input.as_ref().lines().map(|l| l.into()).collect();
        VentField::new(lines)
    }
}

impl VentField {
    fn new(lines: Vec<Line>) -> Self {
        let (mut max_x, mut max_y) = (0, 0);

        for line in &lines {
            max_x = max_x.max(line.0 .0).max(line.1 .0);
            max_y = max_y.max(line.1 .0).max(line.1 .1);
        }

        // Line indices start at 0, so we need an extra coordinate to accomodate the right size
        max_x += 1;
        max_y += 1;

        VentField {
            lines,
            dimensions: (max_x.try_into().unwrap(), max_y.try_into().unwrap()),
        }
    }

    fn overlaps(&self) -> usize {
        let mut field = vec![vec![0_u32; self.dimensions.0]; self.dimensions.1];

        for line in &self.lines {
            if let Some(points) = line.points() {
                for (x, y) in points {
                    field[y][x] += 1;
                }
            }
        }

        field.into_iter().flatten().filter(|n| *n > 1).count()
    }
}

#[derive(Debug, PartialEq)]
struct Line((usize, usize), (usize, usize));

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
    fn points(&self) -> Option<Vec<(usize, usize)>> {
        if !self.is_vertical() && !self.is_horizontal() && !self.is_diagonal() {
            return None;
        }

        let mut points = vec![];

        let mut x = self.0 .0;
        let mut y = self.0 .1;

        while (x, y) != self.1 {
            points.push((x, y));

            x = if x > self.1 .0 {
                x - 1
            } else if x < self.1 .0 {
                x + 1
            } else {
                x
            };

            y = if y > self.1 .1 {
                y - 1
            } else if y < self.1 .1 {
                y + 1
            } else {
                y
            };
        }

        points.push(self.1);
        Some(points)
    }
}

impl<T: AsRef<str> + Debug> From<T> for Line {
    fn from(line: T) -> Line {
        let pairs: Vec<&str> = line.as_ref().split(" -> ").collect();
        if pairs.len() != 2 {
            panic!("ParseError: Cannot parse line: {:?}", &line);
        }

        let start = parse_point(pairs.get(0).unwrap());
        let end = parse_point(pairs.get(1).unwrap());

        Line(start, end)
    }
}

fn parse_point(point: &str) -> (usize, usize) {
    let points: Vec<usize> = point
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();
    match &points[..] {
        &[x, y] => (x, y),
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
    fn it_calculates_field_size() {
        let vent_field = VentField::from(TEST_INPUT);
        assert_eq!(vent_field.dimensions, (10, 10));
    }

    #[test]
    fn it_calculates_line_overlaps() {
        let vent_field = VentField::from(TEST_INPUT);
        assert_eq!(vent_field.overlaps(), 12);
    }
}
