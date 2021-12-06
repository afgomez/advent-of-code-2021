use aoc::input::read_input;

fn main() -> Result<(), std::io::Error> {
    let input = read_input()?;
    let mut simulator = FishSimulator::from(input);
    simulator.advance(80);

    println!("{}", simulator.count());

    Ok(())
}

struct FishSimulator {
    population: [u64; 9],
}

impl FishSimulator {
    fn advance(&mut self, days: u64) {
        for _ in 0..days {
            let new_fish = self.population[0];
            self.population.copy_within(1.., 0);
            self.population[8] = new_fish;
            self.population[6] += new_fish;
        }
    }

    fn count(&self) -> u64 {
        self.population.iter().sum()
    }
}

impl<T: AsRef<str>> From<T> for FishSimulator {
    fn from(input: T) -> Self {
        let mut population = [0; 9];

        for day_count in input.as_ref().split(',') {
            if let Ok(day_count) = day_count.trim().parse::<usize>() {
                population[day_count] += 1;
            }
        }

        FishSimulator { population }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn it_counts_the_initial_population() {
        let simulator = FishSimulator::from(TEST_INPUT);
        assert_eq!(simulator.count(), 5);
    }

    #[test]
    fn it_counts_the_population_after_some_days_pass() {
        let mut simulator = FishSimulator::from(TEST_INPUT);

        simulator.advance(18);
        assert_eq!(simulator.count(), 26);

        simulator.advance(80 - 18);
        assert_eq!(simulator.count(), 5934);
    }
}
