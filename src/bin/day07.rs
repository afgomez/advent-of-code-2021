use std::cmp::{max, min};
use std::collections::HashMap;

use aoc::input::read_input;

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let army = CrabArmy::from(input);

    // println!("{:?}", army.positions);

    println!("{}", army.sim_align());

    Ok(())
}

struct CrabArmy {
    positions: Vec<u64>,
}

impl CrabArmy {
    fn sim_align(&self) -> u64 {
        let mid_point = &self.positions[self.positions.len() / 2];

        self.positions
            .iter()
            .map(|p| max(p, mid_point) - min(p, mid_point))
            .sum()
    }
}

impl<T: AsRef<str>> From<T> for CrabArmy {
    fn from(input: T) -> Self {
        let mut positions: Vec<u64> = input
            .as_ref()
            .split(',')
            .filter_map(|p| p.trim().parse::<u64>().ok())
            .collect();

        positions.sort_unstable();
        CrabArmy { positions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn it_calculates_fuel_to_align() {
        let army = CrabArmy::from(TEST_INPUT);
        assert_eq!(army.sim_align(), 37);
    }
}
