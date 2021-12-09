use aoc::input::read_input;

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let map = HeightMap::from(input);

    let risk_level_sum = map.risk_level_sum();
    println!("{}", risk_level_sum);

    let biggest_basins_product = map.biggest_basins_product();
    println!("{}", biggest_basins_product);

    Ok(())
}

struct HeightMap {
    grid: Vec<Vec<u32>>,
}

impl HeightMap {
    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut points = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, measurement) in row.iter().enumerate() {
                let (prev_x, prev_y, next_x, next_y) =
                    adjacent_to(x, y, row.len() - 1, self.grid.len() - 1);
                if (prev_x.is_some() && self.grid[y][prev_x.unwrap()] <= *measurement)
                    || (next_x.is_some() && self.grid[y][next_x.unwrap()] <= *measurement)
                    || (prev_y.is_some() && self.grid[prev_y.unwrap()][x] <= *measurement)
                    || (next_y.is_some() && self.grid[next_y.unwrap()][x] <= *measurement)
                {
                    continue;
                };
                points.push((x, y));
            }
        }

        points
    }

    fn biggest_basins_product(&self) -> u32 {
        let mut exploration_map = vec![vec![false; self.grid[0].len()]; self.grid.len()];

        let mut basin_sizes: Vec<u32> = self
            .low_points()
            .into_iter()
            .map(|p| self.basin_size(p, &mut exploration_map))
            .collect();

        basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
        basin_sizes.into_iter().take(3).product()
    }

    fn basin_size(&self, (x, y): (usize, usize), exploration_map: &mut Vec<Vec<bool>>) -> u32 {
        // start from the point, and recursively sum the sizes of the unexplored points
        if exploration_map[y][x] {
            return 0;
        }

        exploration_map[y][x] = true;

        if self.grid[y][x] == 9 {
            return 0;
        }

        let mut sum = 1;
        let (prev_x, prev_y, next_x, next_y) =
            adjacent_to(x, y, self.grid[0].len() - 1, self.grid.len() - 1);

        if let Some(prev_x) = prev_x {
            sum += self.basin_size((prev_x, y), exploration_map);
        }
        if let Some(prev_y) = prev_y {
            sum += self.basin_size((x, prev_y), exploration_map);
        }
        if let Some(next_x) = next_x {
            sum += self.basin_size((next_x, y), exploration_map);
        }
        if let Some(next_y) = next_y {
            sum += self.basin_size((x, next_y), exploration_map);
        }

        sum
    }

    fn risk_level_sum(&self) -> u32 {
        self.low_points()
            .into_iter()
            .map(|(x, y)| self.grid[y][x] + 1)
            .sum()
    }
}

fn adjacent_to(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
    let prev_x = x.saturating_sub(1);
    let prev_y = y.saturating_sub(1);
    let next_x = (x + 1).min(max_x);
    let next_y = (y + 1).min(max_y);

    (
        if prev_x == x { None } else { Some(prev_x) },
        if prev_y == y { None } else { Some(prev_y) },
        if next_x == x { None } else { Some(next_x) },
        if next_y == y { None } else { Some(next_y) },
    )
}

impl<T: AsRef<str>> From<T> for HeightMap {
    fn from(input: T) -> Self {
        let grid = input
            .as_ref()
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();

        HeightMap { grid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn it_parses_input() {
        let map = HeightMap::from(TEST_INPUT);
        assert_eq!(map.grid.len(), 5);
    }

    #[test]
    fn it_finds_low_points() {
        let map = HeightMap::from(TEST_INPUT);
        assert_eq!(map.low_points(), vec![(1, 0), (9, 0), (2, 2), (6, 4)])
    }

    #[test]
    fn it_sums_risk_levels() {
        let map = HeightMap::from(TEST_INPUT);
        assert_eq!(map.risk_level_sum(), 15);
    }

    #[test]
    fn it_finds_basin_sizes() {
        let map = HeightMap::from(TEST_INPUT);

        assert_eq!(map.biggest_basins_product(), 1134);
    }
}
