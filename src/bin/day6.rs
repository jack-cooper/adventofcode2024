use std::{
    cell::{Cell, OnceCell},
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
    ops::{Bound, Index},
};

use adventofcode::solve_day;
use anyhow::anyhow;
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

struct Lab {
    col_count: i32,
    rows: Vec<Vec<PositionType>>,
    row_count: i32,
}

#[derive(Eq, PartialEq)]
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

    fn visited_positions(&self, start_position: IVec2) -> HashSet<IVec2> {
        let mut current_position = start_position;
        let mut direction = Direction::North;

        let mut visited_positions: HashSet<IVec2> = HashSet::new();

        loop {
            visited_positions.insert(current_position);

            let next_position = current_position + direction.xy();

            if !self.in_bounds(next_position) {
                break visited_positions;
            }

            match self[next_position.y as usize][next_position.x as usize] {
                PositionType::Empty => {
                    current_position = next_position;
                }
                PositionType::Obstruction => {
                    direction = direction.next();
                }
            }
        }
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    const ALL: [Self; 4] = [Self::East, Self::North, Self::South, Self::West];

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

    let visited_positions = lab.visited_positions(start_position.into_inner().unwrap());

    Ok(visited_positions.len() as u64)
}

fn part2(input: &str) -> anyhow::Result<u64> {
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

    struct XFirstIVec2(IVec2);

    impl Eq for XFirstIVec2 {}

    impl Ord for XFirstIVec2 {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.0.x.cmp(&other.0.x) {
                ordering @ (Ordering::Less | Ordering::Greater) => ordering,
                Ordering::Equal => self.0.y.cmp(&other.0.y),
            }
        }
    }

    impl PartialEq for XFirstIVec2 {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl PartialOrd for XFirstIVec2 {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    struct YFirstIVec2(IVec2);

    impl Eq for YFirstIVec2 {}

    impl Ord for YFirstIVec2 {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.0.y.cmp(&other.0.y) {
                ordering @ (Ordering::Less | Ordering::Greater) => ordering,
                Ordering::Equal => self.0.x.cmp(&other.0.x),
            }
        }
    }

    impl PartialEq for YFirstIVec2 {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl PartialOrd for YFirstIVec2 {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut obstructions_by_col: BTreeSet<XFirstIVec2> = BTreeSet::new();
    let mut obstructions_by_row: BTreeSet<YFirstIVec2> = BTreeSet::new();

    for (y, row) in lab.rows.iter().enumerate() {
        let y = y as i32;
        for (x, cell) in row.iter().enumerate() {
            let x = x as i32;

            if *cell == PositionType::Obstruction {
                obstructions_by_col.insert(XFirstIVec2(IVec2 { x, y }));
                obstructions_by_row.insert(YFirstIVec2(IVec2 { x, y }));
            }
        }
    }

    let mut next_obstructions: HashMap<(IVec2, Direction), Option<IVec2>> =
        HashMap::with_capacity(4 * lab.row_count as usize * lab.col_count as usize);

    for (y, row) in lab.rows.iter().enumerate() {
        let y = y as i32;
        for (x, cell) in row.iter().enumerate() {
            let x = x as i32;

            let position = IVec2 { x, y };

            if *cell == PositionType::Empty {
                for direction in Direction::ALL {
                    let next_obstruction = match direction {
                        Direction::East => obstructions_by_row
                            .range((
                                Bound::Excluded(YFirstIVec2(position)),
                                Bound::Excluded(YFirstIVec2(IVec2 { x: 0, y: y + 1 })),
                            ))
                            .next()
                            .map(|&YFirstIVec2(position)| position),
                        Direction::North => obstructions_by_col
                            .range((
                                Bound::Excluded(XFirstIVec2(IVec2 {
                                    x: x - 1,
                                    y: lab.row_count - 1,
                                })),
                                Bound::Excluded(XFirstIVec2(position)),
                            ))
                            .last()
                            .map(|&XFirstIVec2(position)| position),
                        Direction::South => obstructions_by_col
                            .range((
                                Bound::Excluded(XFirstIVec2(position)),
                                Bound::Excluded(XFirstIVec2(IVec2 { x: x + 1, y: 0 })),
                            ))
                            .next()
                            .map(|&XFirstIVec2(position)| position),
                        Direction::West => obstructions_by_row
                            .range((
                                Bound::Excluded(YFirstIVec2(IVec2 {
                                    x: lab.col_count - 1,
                                    y: y - 1,
                                })),
                                Bound::Excluded(YFirstIVec2(position)),
                            ))
                            .last()
                            .map(|&YFirstIVec2(position)| position),
                    };

                    next_obstructions.insert((position, direction), next_obstruction);
                }
            }
        }
    }

    let next_obstructions = &next_obstructions;

    let start_position = start_position.into_inner().unwrap();

    let visited_positions = {
        let mut visited_positions = lab.visited_positions(start_position);
        visited_positions.remove(&start_position);
        visited_positions
    };

    let mut looped_routes = 0;

    for new_obstruction_position in visited_positions {
        let current_position = Cell::new(start_position);
        let direction = Cell::new(Direction::North);

        let mut seen_obstructions: HashSet<(IVec2, Direction)> = HashSet::new();

        let new_obstruction_position = || match direction.get() {
            Direction::East => (new_obstruction_position.x > current_position.get().x
                && new_obstruction_position.y == current_position.get().y)
                .then_some(new_obstruction_position),
            Direction::North => (new_obstruction_position.x == current_position.get().x
                && new_obstruction_position.y < current_position.get().y)
                .then_some(new_obstruction_position),
            Direction::South => (new_obstruction_position.x == current_position.get().x
                && new_obstruction_position.y > current_position.get().y)
                .then_some(new_obstruction_position),
            Direction::West => (new_obstruction_position.x < current_position.get().x
                && new_obstruction_position.y == current_position.get().y)
                .then_some(new_obstruction_position),
        };

        let next_obstruction = || match (
            new_obstruction_position(),
            next_obstructions[&(current_position.get(), direction.get())],
        ) {
            (None, None) => None,
            (None, Some(position)) | (Some(position), None) => Some(position),
            (Some(new_position), Some(next_position)) => Some(match direction.get() {
                Direction::East => {
                    if new_position.x < next_position.x {
                        new_position
                    } else {
                        next_position
                    }
                }
                Direction::North => {
                    if new_position.y > next_position.y {
                        new_position
                    } else {
                        next_position
                    }
                }
                Direction::South => {
                    if new_position.y < next_position.y {
                        new_position
                    } else {
                        next_position
                    }
                }
                Direction::West => {
                    if new_position.x > next_position.x {
                        new_position
                    } else {
                        next_position
                    }
                }
            }),
        };

        let mut n = next_obstruction();

        while let Some(obstruction_position) = n {
            if !seen_obstructions.insert((obstruction_position, direction.get())) {
                looped_routes += 1;
                break;
            }

            current_position.set(obstruction_position - direction.get().xy());
            direction.set(direction.get().next());

            n = next_obstruction();
        }
    }

    Ok(looped_routes)
}
