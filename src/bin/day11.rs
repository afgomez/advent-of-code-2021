use std::collections::{HashSet, VecDeque};
use std::fmt;

use aoc::input::read_input;

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let mut octos = OctoLights::from(&input);

    println!("{}", octos.step(100));

    let mut octos = OctoLights::from(&input);
    let octi_count = octos.grid.len() * octos.grid[0].len();
    let mut step_no = 1;

    while octos.step(1) != octi_count {
        step_no += 1;
    }

    println!("{}", step_no);

    Ok(())
}

struct OctoLights {
    grid: Vec<Vec<u8>>,
}

impl OctoLights {
    fn step(&mut self, steps: usize) -> usize {
        let mut flash_count = 0;

        for _ in 0..steps {
            let mut must_radiate: VecDeque<(usize, usize)> = VecDeque::new();
            let mut has_flashed: HashSet<(usize, usize)> = HashSet::new();

            for (r, row) in self.grid.iter_mut().enumerate() {
                for (c, val) in row.iter_mut().enumerate() {
                    *val += 1;
                    if *val > 9 {
                        must_radiate.push_back((r, c));
                        has_flashed.insert((r, c));
                    }
                }
            }

            while let Some((row, col)) = must_radiate.pop_front() {
                let prev_r = row.saturating_sub(1);
                let next_r = (row + 1).min(self.grid.len() - 1);
                let prev_c = col.saturating_sub(1);
                let next_c = (col + 1).min(self.grid[row].len() - 1);

                for r in prev_r..=next_r {
                    for c in prev_c..=next_c {
                        // Don't radiate a point onto itself
                        if (r == row && c == col) || has_flashed.contains(&(r, c)) {
                            self.grid[r][c] = 0;
                            continue;
                        }

                        let next_val = self.grid[r][c] + 1;
                        if next_val > 9 {
                            must_radiate.push_back((r, c));
                            has_flashed.insert((r, c));
                        }
                        self.grid[r][c] = next_val;
                    }
                }
            }

            flash_count += has_flashed.len();
        }

        flash_count
    }
}

impl<T: AsRef<str>> From<T> for OctoLights {
    fn from(input: T) -> Self {
        let grid: Vec<Vec<u8>> = input
            .as_ref()
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();

        OctoLights { grid }
    }
}

impl fmt::Display for OctoLights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for val in row {
                write!(f, "{}", val)?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_propagates_flashes() {
        let mut octos = OctoLights::from("11111\n19991\n19191\n19991\n11111");
        octos.step(1);

        assert_eq!(format!("{}", octos), "34543\n40004\n50005\n40004\n34543\n");
    }

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn it_counts_flashes() {
        let mut octos = OctoLights::from(TEST_INPUT);

        let mut flash_count = octos.step(10);
        assert_eq!(flash_count, 204);

        flash_count += octos.step(90); // For a total of 100
        assert_eq!(flash_count, 1656);
    }
}
