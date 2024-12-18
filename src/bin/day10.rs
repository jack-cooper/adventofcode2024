use std::{collections::HashSet, ops::Index};

use adventofcode::solve_day;
use anyhow::bail;
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

struct TopographicMap {
    col_count: i32,
    rows: Vec<Vec<Height>>,
    row_count: i32,
}

#[derive(Clone, Copy)]
struct Height(u64);

impl Index<usize> for TopographicMap {
    type Output = Vec<Height>;

    fn index(&self, index: usize) -> &Self::Output {
        self.rows.index(index)
    }
}

impl<I2: IntoIterator<Item = Height>> FromIterator<I2> for TopographicMap {
    fn from_iter<I1>(iter: I1) -> Self
    where
        I1: IntoIterator<Item = I2>,
    {
        let rows: Vec<Vec<Height>> = iter
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

impl TopographicMap {
    fn in_bounds(&self, position: IVec2) -> bool {
        position.min_element() >= 0 && position.x < self.col_count && position.y < self.row_count
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    const ALL: [Self; 4] = [Self::East, Self::North, Self::South, Self::West];

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
    let topographic_map: TopographicMap = input
        .lines()
        .map(|line| {
            line.chars().map(|char| {
                let height = char
                    .to_digit(10)
                    .expect("All chars in input should be digits in the range 0-9.");

                Height(u64::from(height))
            })
        })
        .collect();

    let mut search_stack: Vec<IVec2> = topographic_map
        .rows
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let y = y as i32;

            row.iter()
                .enumerate()
                .filter_map(move |(x, &Height(height))| {
                    let x = x as i32;

                    (height == 0).then_some(IVec2 { x, y })
                })
        })
        .collect();

    let mut visited_positions: HashSet<IVec2> = HashSet::new();

    let mut trailhead_score_sum = 0;

    while let Some(position) = search_stack.pop() {
        let Height(height) = topographic_map[position.y as usize][position.x as usize];

        if height == 0 {
            for position in visited_positions.drain() {
                let Height(height) = topographic_map[position.y as usize][position.x as usize];

                if height == 9 {
                    trailhead_score_sum += 1;
                }
            }
        }

        for direction in Direction::ALL {
            let neighbor_position = position + direction.xy();

            if !topographic_map.in_bounds(neighbor_position)
                || visited_positions.contains(&neighbor_position)
            {
                continue;
            }

            let Height(neighbor_height) =
                topographic_map[neighbor_position.y as usize][neighbor_position.x as usize];

            if neighbor_height == height + 1 {
                if neighbor_height < 9 {
                    search_stack.push(neighbor_position);
                }
                visited_positions.insert(neighbor_position);
            }
        }
    }

    for position in visited_positions.drain() {
        let Height(height) = topographic_map[position.y as usize][position.x as usize];

        if height == 9 {
            trailhead_score_sum += 1;
        }
    }

    Ok(trailhead_score_sum)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let topographic_map: TopographicMap = input
        .lines()
        .map(|line| {
            line.chars().map(|char| {
                let height = char
                    .to_digit(10)
                    .expect("All chars in input should be digits in the range 0-9.");

                Height(u64::from(height))
            })
        })
        .collect();

    let mut search_stack: Vec<IVec2> = topographic_map
        .rows
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let y = y as i32;

            row.iter()
                .enumerate()
                .filter_map(move |(x, &Height(height))| {
                    let x = x as i32;

                    (height == 0).then_some(IVec2 { x, y })
                })
        })
        .collect();

    let mut trailhead_rating_sum = 0;

    while let Some(position) = search_stack.pop() {
        let Height(height) = topographic_map[position.y as usize][position.x as usize];

        for direction in Direction::ALL {
            let neighbor_position = position + direction.xy();

            if !topographic_map.in_bounds(neighbor_position) {
                continue;
            }

            let Height(neighbor_height) =
                topographic_map[neighbor_position.y as usize][neighbor_position.x as usize];

            if neighbor_height == height + 1 {
                if neighbor_height < 9 {
                    search_stack.push(neighbor_position);
                } else {
                    trailhead_rating_sum += 1;
                }
            }
        }
    }

    Ok(trailhead_rating_sum)
}
