use aoc::input::read_input;
use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let mut dot_grid = DotGrid::from(input);

    // dot_grid.fold(1);
    // println!("{}", dot_grid.count_visible());

    dot_grid.fold_all();
    println!("{}", dot_grid);

    Ok(())
}

struct DotGrid {
    grid: HashMap<u32, HashSet<u32>>,
    instructions: VecDeque<Fold>,
}

impl fmt::Display for DotGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_x = self.grid.keys().max().cloned().unwrap();
        let max_y = self
            .grid
            .values()
            .flat_map(|v| v.iter().max())
            .max()
            .cloned()
            .unwrap();

        for x in 0..=max_x {
            if let Some(row) = self.grid.get(&x) {
                for y in 0..=max_y {
                    match row.get(&y) {
                        Some(_) => write!(f, "#")?,
                        None => write!(f, " ")?,
                    }
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "")
    }
}

impl DotGrid {
    fn count_visible(&self) -> usize {
        self.grid.values().fold(0, |total, cols| total + cols.len())
    }

    fn fold(&mut self, n: usize) {
        for _ in 0..n {
            let instruction = self.instructions.pop_front();
            match instruction {
                Some(Fold::X(fold_point)) => {
                    for x_coords in self.grid.values_mut() {
                        let x_to_move: Vec<_> = x_coords
                            .iter()
                            .filter(|&x| *x > fold_point)
                            .cloned()
                            .collect();
                        for x in x_to_move {
                            x_coords.remove(&x);
                            x_coords.insert(x - ((x - fold_point) * 2));
                        }
                    }
                }
                Some(Fold::Y(fold_point)) => {
                    let y_to_move: Vec<_> = self
                        .grid
                        .keys()
                        .filter(|&x| *x > fold_point)
                        .cloned()
                        .collect();

                    for y in y_to_move {
                        if let Some(x_coords) = self.grid.remove(&y) {
                            let new_x_coords = self
                                .grid
                                .entry(y - ((y - fold_point) * 2))
                                .or_insert(HashSet::new());
                            new_x_coords.extend(x_coords);
                        }
                    }
                }
                None => break,
            }
        }
    }

    fn fold_all(&mut self) {
        self.fold(self.instructions.len())
    }
}

impl<T: AsRef<str>> From<T> for DotGrid {
    fn from(input: T) -> Self {
        let mut input = input.as_ref().split("\n\n");
        let raw_coordinates = input.next().unwrap();

        let mut grid = HashMap::new();
        for raw_pair in raw_coordinates.lines() {
            let mut pair = raw_pair.trim().split(',');
            let x = pair.next().unwrap().parse::<u32>().unwrap();
            let y = pair.next().unwrap().parse::<u32>().unwrap();

            let y_coords = grid.entry(y).or_insert(HashSet::new());
            y_coords.insert(x);
        }

        let raw_instructions = input.next().unwrap();
        let instructions = raw_instructions.lines().map(Fold::from).collect();

        DotGrid { grid, instructions }
    }
}

#[derive(Debug, PartialEq)]
enum Fold {
    X(u32),
    Y(u32),
}

impl<T: AsRef<str>> From<T> for Fold {
    fn from(input: T) -> Self {
        let (_, axis_point) = input.as_ref().trim().rsplit_once(' ').unwrap();
        let (axis, point) = axis_point.split_once('=').unwrap();

        let point = point.parse::<u32>().unwrap();

        match axis {
            "x" => Self::X(point),
            "y" => Self::Y(point),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5";

    #[test]
    fn it_parses_the_input() {
        let dot_grid = DotGrid::from(TEST_INPUT);

        assert_eq!(dot_grid.instructions, vec![Fold::Y(7), Fold::X(5)]);
    }

    #[test]
    fn it_counts_visible_points() {
        let mut dot_grid = DotGrid::from(TEST_INPUT);
        assert_eq!(dot_grid.count_visible(), 18);

        dot_grid.fold(1);
        assert_eq!(dot_grid.count_visible(), 17);
    }
}
