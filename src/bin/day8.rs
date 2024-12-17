use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    iter,
    ops::Index,
};

use adventofcode::solve_day;
use anyhow::ensure;
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

struct City {
    antenna_positions: OnceCell<HashMap<Antenna, Vec<IVec2>>>,
    col_count: i32,
    rows: Vec<Vec<Option<Antenna>>>,
    row_count: i32,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Antenna(char);

impl Index<usize> for City {
    type Output = Vec<Option<Antenna>>;

    fn index(&self, index: usize) -> &Self::Output {
        self.rows.index(index)
    }
}

impl<I2: IntoIterator<Item = char>> FromIterator<I2> for City {
    fn from_iter<I1>(iter: I1) -> Self
    where
        I1: IntoIterator<Item = I2>,
    {
        let rows: Vec<Vec<Option<Antenna>>> = iter
            .into_iter()
            .map(|char_iter| {
                char_iter
                    .into_iter()
                    .map(|char| Antenna::try_from(char).ok())
                    .collect()
            })
            .collect();

        assert!(!rows.is_empty(), "No rows in city.");

        let row_count = rows.len();
        let col_count = rows[0].len();

        assert!(
            rows[1..].iter().all(|row| row.len() == col_count),
            "City rows are not all of equal length."
        );

        Self {
            antenna_positions: OnceCell::new(),
            col_count: col_count as i32,
            rows,
            row_count: row_count as i32,
        }
    }
}

impl City {
    fn antenna_positions(&self) -> &HashMap<Antenna, Vec<IVec2>> {
        self.antenna_positions.get_or_init(|| {
            let mut antenna_positions: HashMap<Antenna, Vec<IVec2>> = HashMap::new();

            for (y, row) in self.rows.iter().enumerate() {
                let y = y as i32;

                for (x, &antenna) in row.iter().enumerate() {
                    let x = x as i32;

                    let Some(antenna) = antenna else {
                        continue;
                    };

                    antenna_positions
                        .entry(antenna)
                        .or_default()
                        .push(IVec2 { x, y });
                }
            }

            antenna_positions
        })
    }

    fn in_bounds(&self, position: IVec2) -> bool {
        position.min_element() >= 0 && position.x < self.col_count && position.y < self.row_count
    }
}

impl TryFrom<char> for Antenna {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        ensure!(
            value.is_ascii_alphanumeric(),
            "An antenna must be tuned to an alphanumeric frequency!"
        );

        Ok(Self(value))
    }
}
fn part1(input: &str) -> anyhow::Result<u64> {
    let city: City = input.lines().map(|row| row.chars()).collect();

    let mut antinode_positions: HashSet<IVec2> = HashSet::new();

    for positions in city
        .antenna_positions()
        .values()
        .filter(|positions| positions.len() > 1)
    {
        for (position_a_index, position_a) in positions[..positions.len()].iter().enumerate() {
            for position_b in &positions[(position_a_index + 1)..] {
                let diff = position_a - position_b;

                antinode_positions.extend(
                    [position_a + diff, position_b - diff]
                        .into_iter()
                        .filter(|&position| city.in_bounds(position)),
                );
            }
        }
    }

    Ok(antinode_positions.len() as u64)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let city: City = input.lines().map(|row| row.chars()).collect();

    let mut antinode_positions: HashSet<IVec2> = HashSet::new();

    for positions in city
        .antenna_positions()
        .values()
        .filter(|positions| positions.len() > 1)
    {
        for (position_a_index, &position_a) in positions[..positions.len()].iter().enumerate() {
            for &position_b in &positions[(position_a_index + 1)..] {
                let diff = position_a - position_b;

                antinode_positions.extend(iter::successors(Some(position_a), |&position| {
                    let next_position = position + diff;
                    city.in_bounds(next_position).then_some(next_position)
                }));
                antinode_positions.extend(iter::successors(Some(position_b), |&position| {
                    let next_position = position - diff;
                    city.in_bounds(next_position).then_some(next_position)
                }));
            }
        }
    }

    Ok(antinode_positions.len() as u64)
}
