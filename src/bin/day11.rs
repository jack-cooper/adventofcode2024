use std::str::FromStr;

use adventofcode::solve_day;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

struct Stone(u64);

impl FromStr for Stone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse()?;
        Ok(Self(number))
    }
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let stones: anyhow::Result<Vec<Stone>> = input.split(' ').map(str::parse::<Stone>).collect();
    let mut stones = stones?;

    const BLINKS: u32 = 25;

    for temp in 0..BLINKS {
        println!("{temp} | {}", stones.len());

        let mut current_index = 0;

        while current_index < stones.len() {
            let stone = &mut stones[current_index];

            match stone {
                Stone(n @ 0) => {
                    *n = 1;
                }
                Stone(n) if n.ilog10() % 2 == 1 => {
                    let num_digits = n.ilog10() + 1;

                    let divisor = 10_u64.pow(num_digits / 2);

                    let stone_a = *n / divisor;
                    let stone_b = *n - stone_a * divisor;

                    *n = stone_a;
                    stones.insert(current_index + 1, Stone(stone_b));

                    current_index += 1;
                }
                Stone(n) => {
                    *n *= 2024;
                }
            }

            current_index += 1;
        }
    }

    Ok(stones.len() as u64)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let stones: anyhow::Result<Vec<Stone>> = input.split(' ').map(str::parse::<Stone>).collect();
    let mut stones = stones?;

    const BLINKS: u32 = 75;

    for temp in 0..BLINKS {
        println!("{temp} | {}", stones.len());

        let mut current_index = 0;

        while current_index < stones.len() {
            let stone = &mut stones[current_index];

            match stone {
                Stone(n @ 0) => {
                    *n = 1;
                }
                Stone(n) if n.ilog10() % 2 == 1 => {
                    let num_digits = n.ilog10() + 1;

                    let divisor = 10_u64.pow(num_digits / 2);

                    let stone_a = *n / divisor;
                    let stone_b = *n - stone_a * divisor;

                    *n = stone_a;
                    stones.insert(current_index + 1, Stone(stone_b));

                    current_index += 1;
                }
                Stone(n) => {
                    *n *= 2024;
                }
            }

            current_index += 1;
        }
    }

    Ok(stones.len() as u64)
}
