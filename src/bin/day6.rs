use std::{cell::OnceCell, collections::HashSet, ops::Index};

use adventofcode::solve_day;
use anyhow::{anyhow, bail};
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

struct Lab {
    col_count: i32,
    rows: Vec<Vec<PositionType>>,
    row_count: i32,
}

enum PositionType {
    Empty,
    Obstruction,
}

impl Index<usize> for Lab {
    type Output = Vec<PositionType>;

    fn index(&self, index: usize) -> &Self::Output {
        self.rows.index(index)
    }
}

impl<I2: IntoIterator<Item = PositionType>> FromIterator<I2> for Lab {
    fn from_iter<I1>(iter: I1) -> Self
    where
        I1: IntoIterator<Item = I2>,
    {
        let rows: Vec<Vec<PositionType>> = iter
            .into_iter()
            .map(|char_iter| char_iter.into_iter().collect())
            .collect();

        assert!(!rows.is_empty(), "No rows in Lab.");

        let row_count = rows.len();
        let col_count = rows[0].len();

        assert!(
            rows[1..].iter().all(|row| row.len() == col_count),
            "Lab rows are not all of equal length."
        );

        Self {
            col_count: col_count as i32,
            rows,
            row_count: row_count as i32,
        }
    }
}

impl Lab {
    fn in_bounds(&self, position: IVec2) -> bool {
        position.min_element() >= 0 && position.x < self.col_count && position.y < self.row_count
    }
}

impl TryFrom<char> for PositionType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | '^' => Ok(Self::Empty),
            '#' => Ok(Self::Obstruction),
            _ => Err(anyhow!(
                "Maps should only be comprised of the characters `.`, `#` and `^`."
            )),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn xy(self) -> IVec2 {
        match self {
            Self::East => IVec2::X,
            Self::North => IVec2::NEG_Y,
            Self::South => IVec2::Y,
            Self::West => IVec2::NEG_X,
        }
    }
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let start_position = OnceCell::new();

    let lab: Lab = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            let start_position = &start_position;

            row.chars().enumerate().map(move |(x, cell)| {
                if cell == '^' {
                    start_position.set(IVec2::new(x as i32, y as i32)).unwrap();
                }
                PositionType::try_from(cell).unwrap()
            })
        })
        .collect();

    let mut current_position = start_position.into_inner().unwrap();
    let mut direction = Direction::North;

    let mut visited_positions: HashSet<IVec2> = HashSet::new();

    loop {
        visited_positions.insert(current_position);

        let next_position = current_position + direction.xy();

        if !lab.in_bounds(next_position) {
            break Ok(visited_positions.len() as u64);
        }

        match lab[next_position.y as usize][next_position.x as usize] {
            PositionType::Empty => {
                current_position = next_position;
            }
            PositionType::Obstruction => {
                direction = direction.next();
            }
        }
    }
}

fn part2(_input: &str) -> anyhow::Result<u64> {
    bail!("Unimplemented");
}
