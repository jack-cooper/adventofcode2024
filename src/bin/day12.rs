use std::{
    collections::{HashSet, VecDeque},
    ops::Index,
};

use adventofcode::solve_day;
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

struct GardenMap {
    col_count: i32,
    rows: Vec<Vec<Plot>>,
    row_count: i32,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Plot(char);

impl Index<usize> for GardenMap {
    type Output = Vec<Plot>;

    fn index(&self, index: usize) -> &Self::Output {
        self.rows.index(index)
    }
}

impl<I2: IntoIterator<Item = Plot>> FromIterator<I2> for GardenMap {
    fn from_iter<I1>(iter: I1) -> Self
    where
        I1: IntoIterator<Item = I2>,
    {
        let rows: Vec<Vec<Plot>> = iter
            .into_iter()
            .map(|char_iter| char_iter.into_iter().collect())
            .collect();

        assert!(!rows.is_empty(), "No rows in garden.");

        let row_count = rows.len();
        let col_count = rows[0].len();

        assert!(row_count == col_count, "All gardens should be square.");

        assert!(
            rows[1..].iter().all(|row| row.len() == col_count),
            "Plot rows are not all of equal length."
        );

        Self {
            col_count: col_count as i32,
            rows,
            row_count: row_count as i32,
        }
    }
}

impl GardenMap {
    fn in_bounds(&self, position: IVec2) -> bool {
        position.min_element() >= 0 && position.x < self.col_count && position.y < self.row_count
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    let garden_map: GardenMap = input.lines().map(|row| row.chars().map(Plot)).collect();

    let mut visited_positions: HashSet<IVec2> =
        HashSet::with_capacity(garden_map.col_count as usize * garden_map.row_count as usize);

    let mut total_price = 0;

    for (y, row) in garden_map.rows.iter().enumerate() {
        let y = y as i32;
        for (x, &plot) in row.iter().enumerate() {
            let x = x as i32;

            if visited_positions.contains(&IVec2 { x, y }) {
                continue;
            }

            let mut region_positions: HashSet<IVec2> = HashSet::from_iter([IVec2 { x, y }]);
            let mut search_stack: Vec<IVec2> = vec![IVec2 { x, y }];

            let mut perimeter: u64 = 0;

            while let Some(position) = search_stack.pop() {
                for direction in Direction::ALL {
                    let neighbor_position = position + direction.xy();

                    if !garden_map.in_bounds(neighbor_position) {
                        perimeter += 1;
                        continue;
                    }

                    if garden_map[neighbor_position.y as usize][neighbor_position.x as usize]
                        == plot
                    {
                        if !region_positions.contains(&neighbor_position) {
                            search_stack.push(neighbor_position);
                            region_positions.insert(neighbor_position);
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }

            let area = region_positions.len() as u64;

            let region_price = perimeter * area;
            total_price += region_price;

            visited_positions.extend(region_positions);
        }
    }

    Ok(total_price)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let garden_map: GardenMap = input.lines().map(|row| row.chars().map(Plot)).collect();

    let mut visited_positions: HashSet<IVec2> =
        HashSet::with_capacity(garden_map.col_count as usize * garden_map.row_count as usize);

    let mut total_price = 0;

    for (y, row) in garden_map.rows.iter().enumerate() {
        let y = y as i32;
        for (x, &plot) in row.iter().enumerate() {
            let x = x as i32;

            if visited_positions.contains(&IVec2 { x, y }) {
                continue;
            }

            let mut region_positions: HashSet<IVec2> = HashSet::from_iter([IVec2 { x, y }]);
            let mut search_queue: VecDeque<IVec2> = VecDeque::from_iter([IVec2 { x, y }]);

            let mut perimeter_sections: HashSet<(IVec2, Direction)> = HashSet::new();

            while let Some(position) = search_queue.pop_front() {
                for direction in Direction::ALL {
                    let neighbor_position = position + direction.xy();

                    if !garden_map.in_bounds(neighbor_position) {
                        perimeter_sections.insert((position, direction));
                        continue;
                    }

                    if garden_map[neighbor_position.y as usize][neighbor_position.x as usize]
                        == plot
                    {
                        if !region_positions.contains(&neighbor_position) {
                            search_queue.push_back(neighbor_position);
                            region_positions.insert(neighbor_position);
                        }
                    } else {
                        perimeter_sections.insert((position, direction));
                    }
                }
            }

            let mut perimeter_sections: Vec<_> = perimeter_sections.into_iter().collect();
            perimeter_sections.sort_by(|(pos, dir), (pos2, dir2)| {
                pos.y
                    .cmp(&pos2.y)
                    .then(pos.x.cmp(&pos2.x))
                    .then(dir.cmp(dir2))
            });

            let mut sides = 0;

            while let Some((current_position, current_direction)) = perimeter_sections.pop() {
                sides += 1;

                let perpendicular_directions = match current_direction {
                    Direction::East | Direction::West => [Direction::North, Direction::South],
                    Direction::North | Direction::South => [Direction::East, Direction::West],
                };

                let mut left = Some((current_position, current_direction));
                let mut right = Some((current_position, current_direction));

                loop {
                    left = left.and_then(|section| {
                        let index = perimeter_sections.iter().position(|&perimeter_section| {
                            perimeter_section
                                == (
                                    section.0 + perpendicular_directions[0].xy(),
                                    current_direction,
                                )
                        });

                        index.map(|index| perimeter_sections.remove(index))
                    });

                    right = right.and_then(|section| {
                        let index = perimeter_sections.iter().position(|&perimeter_section| {
                            perimeter_section
                                == (
                                    section.0 + perpendicular_directions[1].xy(),
                                    current_direction,
                                )
                        });

                        index.map(|index| perimeter_sections.remove(index))
                    });

                    if left.or(right).is_none() {
                        break;
                    }
                }
            }

            let area = region_positions.len() as u64;

            let region_price = sides * area;
            total_price += region_price;

            visited_positions.extend(region_positions);
        }
    }

    Ok(total_price)
}
