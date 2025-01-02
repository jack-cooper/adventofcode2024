use std::{
    cell::OnceCell,
    collections::HashSet,
    ops::{Index, IndexMut},
};

use adventofcode::solve_day;
use anyhow::anyhow;
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

struct WarehouseMap {
    rows: Vec<Vec<PositionType>>,
}

#[derive(Eq, PartialEq)]
enum PositionType {
    Box,
    Empty,
    Wall,
}

impl Index<usize> for WarehouseMap {
    type Output = Vec<PositionType>;

    fn index(&self, index: usize) -> &Self::Output {
        self.rows.index(index)
    }
}

impl IndexMut<usize> for WarehouseMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.rows.index_mut(index)
    }
}

impl<I2: IntoIterator<Item = PositionType>> FromIterator<I2> for WarehouseMap {
    fn from_iter<I1>(iter: I1) -> Self
    where
        I1: IntoIterator<Item = I2>,
    {
        let rows: Vec<Vec<PositionType>> = iter
            .into_iter()
            .map(|char_iter| char_iter.into_iter().collect())
            .collect();

        assert!(!rows.is_empty(), "No rows in warehouse.");

        let col_count = rows[0].len();

        assert!(
            rows[1..].iter().all(|row| row.len() == col_count),
            "Warehouse rows are not all of equal length."
        );

        Self { rows }
    }
}

impl WarehouseMap {
    fn next_empty_space(&self, mut position: IVec2, direction: Direction) -> Option<IVec2> {
        loop {
            position += direction.xy();

            match self[position.y as usize][position.x as usize] {
                PositionType::Box => continue,
                PositionType::Empty => return Some(position),
                PositionType::Wall => return None,
            }
        }
    }
}

impl TryFrom<char> for PositionType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Box),
            '@' | '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            _ => Err(anyhow!("Invalid position type char detected.")),
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

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::East),
            '^' | '.' => Ok(Self::North),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            _ => Err(anyhow!("Invalid movement char detected.")),
        }
    }
}

impl Direction {
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
    let (warehouse_map, movements) = input
        .split_once("\n\n")
        .ok_or(anyhow!("Input should contain a double newline."))?;

    let robot_position = OnceCell::new();

    let mut warehouse_map: WarehouseMap = warehouse_map
        .lines()
        .enumerate()
        .map(|(y, row)| {
            let robot_position = &robot_position;

            row.chars().enumerate().map(move |(x, char)| {
                if char == '@' {
                    robot_position.set(IVec2::new(x as i32, y as i32)).unwrap();
                }
                PositionType::try_from(char).unwrap()
            })
        })
        .collect();

    let mut robot_position = robot_position
        .into_inner()
        .ok_or(anyhow!("No robot found when parsing input!"))?;

    let movements: Vec<Direction> = movements
        .replace('\n', "")
        .chars()
        .map(|char| Direction::try_from(char).unwrap())
        .collect();

    for movement in movements {
        let Some(next_empty_space) = warehouse_map.next_empty_space(robot_position, movement)
        else {
            continue;
        };

        let next_position = robot_position + movement.xy();

        if next_empty_space != next_position {
            warehouse_map[next_empty_space.y as usize][next_empty_space.x as usize] =
                PositionType::Box;
            warehouse_map[next_position.y as usize][next_position.x as usize] = PositionType::Empty;
        }

        robot_position = next_position;
    }

    let mut gps_sum = 0;

    for (y, row) in warehouse_map.rows.into_iter().enumerate() {
        let y = y as u64;
        for (x, position_type) in row.into_iter().enumerate() {
            let x = x as u64;

            if position_type == PositionType::Box {
                gps_sum += 100 * y + x;
            }
        }
    }

    Ok(gps_sum)
}

struct WideWarehouseMap {
    rows: Vec<Vec<WidePositionType>>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum WidePositionType {
    Box(BoxSegment),
    Empty,
    Wall,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum BoxSegment {
    Left,
    Right,
}

impl BoxSegment {
    fn inverse(self) -> Self {
        match self {
            BoxSegment::Left => BoxSegment::Right,
            BoxSegment::Right => BoxSegment::Left,
        }
    }
}

impl Index<usize> for WideWarehouseMap {
    type Output = Vec<WidePositionType>;

    fn index(&self, index: usize) -> &Self::Output {
        self.rows.index(index)
    }
}

impl IndexMut<usize> for WideWarehouseMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.rows.index_mut(index)
    }
}

impl<I2: IntoIterator<Item = WidePositionType>> FromIterator<I2> for WideWarehouseMap {
    fn from_iter<I1>(iter: I1) -> Self
    where
        I1: IntoIterator<Item = I2>,
    {
        let rows: Vec<Vec<WidePositionType>> = iter
            .into_iter()
            .map(|char_iter| char_iter.into_iter().collect())
            .collect();

        assert!(!rows.is_empty(), "No rows in warehouse.");

        let col_count = rows[0].len();

        assert!(
            rows[1..].iter().all(|row| row.len() == col_count),
            "Warehouse rows are not all of equal length."
        );

        Self { rows }
    }
}

impl WideWarehouseMap {
    fn try_move(&mut self, robot_position: &mut IVec2, direction: Direction) {
        match direction {
            Direction::North | Direction::South => {
                let next_position = *robot_position + direction.xy();

                if self[next_position.y as usize][next_position.x as usize]
                    == WidePositionType::Empty
                {
                    *robot_position += direction.xy();
                    return;
                }

                if self[next_position.y as usize][next_position.x as usize]
                    == WidePositionType::Wall
                {
                    return;
                }

                let next_box_segment = self[next_position.y as usize][next_position.x as usize];

                let box_segment_positions = match next_box_segment {
                    WidePositionType::Box(box_segment) => match box_segment {
                        BoxSegment::Left => (next_position, next_position + IVec2::X),
                        BoxSegment::Right => (next_position + IVec2::NEG_X, next_position),
                    },
                    _ => unreachable!(),
                };

                let mut box_stack = vec![box_segment_positions];
                let mut visited_boxes = HashSet::new();

                while let Some((box_segment_position_left, box_segment_position_right)) =
                    box_stack.pop()
                {
                    visited_boxes.insert((box_segment_position_left, box_segment_position_right));

                    let next_position_left = box_segment_position_left + direction.xy();
                    let next_position_right = box_segment_position_right + direction.xy();

                    match self[next_position_left.y as usize][next_position_left.x as usize] {
                        WidePositionType::Box(box_segment) => match box_segment {
                            BoxSegment::Left => {
                                box_stack.push((next_position_left, next_position_right));
                                continue;
                            }
                            BoxSegment::Right => {
                                box_stack
                                    .push((next_position_left + IVec2::NEG_X, next_position_left));
                            }
                        },
                        WidePositionType::Empty => {}
                        WidePositionType::Wall => {
                            return;
                        }
                    }

                    match self[next_position_right.y as usize][next_position_right.x as usize] {
                        WidePositionType::Box(box_segment) => match box_segment {
                            BoxSegment::Left => {
                                box_stack
                                    .push((next_position_right, next_position_right + IVec2::X));
                            }
                            BoxSegment::Right => {
                                for (y, row) in self.rows.iter().enumerate() {
                                    for (x, cell) in row.iter().enumerate() {
                                        let char = match cell {
                                            WidePositionType::Box(box_segment) => match box_segment
                                            {
                                                BoxSegment::Left => '[',
                                                BoxSegment::Right => {
                                                    let x = x as i32;
                                                    let y = y as i32;

                                                    let current_pos = IVec2 { x, y };

                                                    if next_position_right == current_pos {
                                                        'X'
                                                    } else {
                                                        ']'
                                                    }
                                                }
                                            },
                                            WidePositionType::Empty => '.',
                                            WidePositionType::Wall => '#',
                                        };

                                        print!("{char}");
                                    }
                                    println!();
                                }
                                unreachable!()
                            }
                        },
                        WidePositionType::Empty => {}
                        WidePositionType::Wall => {
                            return;
                        }
                    }
                }

                *robot_position += direction.xy();

                for &(box_position_left, box_position_right) in &visited_boxes {
                    self[box_position_left.y as usize][box_position_left.x as usize] =
                        WidePositionType::Empty;
                    self[box_position_right.y as usize][box_position_right.x as usize] =
                        WidePositionType::Empty;
                }
                for (box_position_left, box_position_right) in visited_boxes {
                    let box_position_left = box_position_left + direction.xy();
                    let box_position_right = box_position_right + direction.xy();
                    self[box_position_left.y as usize][box_position_left.x as usize] =
                        WidePositionType::Box(BoxSegment::Left);
                    self[box_position_right.y as usize][box_position_right.x as usize] =
                        WidePositionType::Box(BoxSegment::Right);
                }
            }
            Direction::East | Direction::West => {
                let mut position = *robot_position;

                let next_empty_space = loop {
                    position += direction.xy();

                    match self[position.y as usize][position.x as usize] {
                        WidePositionType::Box(_) => continue,
                        WidePositionType::Empty => break position,
                        WidePositionType::Wall => return,
                    }
                };

                let mut next_position = *robot_position + direction.xy();

                if next_empty_space != next_position {
                    let WidePositionType::Box(mut segment_type) =
                        self[next_position.y as usize][next_position.x as usize]
                    else {
                        unreachable!();
                    };

                    self[next_position.y as usize][next_position.x as usize] =
                        WidePositionType::Empty;

                    loop {
                        next_position += direction.xy();

                        self[next_position.y as usize][next_position.x as usize] =
                            WidePositionType::Box(segment_type);
                        segment_type = segment_type.inverse();

                        if next_position == next_empty_space {
                            break;
                        }
                    }
                }

                *robot_position += direction.xy();
            }
        }
    }
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let (warehouse_map, movements) = input
        .split_once("\n\n")
        .ok_or(anyhow!("Input should contain a double newline."))?;

    let robot_position = OnceCell::new();

    let mut warehouse_map: WideWarehouseMap = warehouse_map
        .lines()
        .enumerate()
        .map(|(y, row)| {
            let robot_position = &robot_position;

            row.chars()
                .enumerate()
                .flat_map(move |(x, char)| match char {
                    'O' => [
                        WidePositionType::Box(BoxSegment::Left),
                        WidePositionType::Box(BoxSegment::Right),
                    ],
                    '@' => {
                        robot_position
                            .set(IVec2::new((x * 2) as i32, y as i32))
                            .unwrap();

                        [WidePositionType::Empty, WidePositionType::Empty]
                    }
                    '.' => [WidePositionType::Empty, WidePositionType::Empty],
                    '#' => [WidePositionType::Wall, WidePositionType::Wall],
                    _ => unreachable!("Invalid position type char detected."),
                })
        })
        .collect();

    let mut robot_position = robot_position
        .into_inner()
        .ok_or(anyhow!("No robot found when parsing input!"))?;

    let movements: Vec<Direction> = movements
        .replace('\n', "")
        .chars()
        .map(|char| Direction::try_from(char).unwrap())
        .collect();

    for movement in movements {
        warehouse_map.try_move(&mut robot_position, movement);
    }

    let mut gps_sum = 0;

    for (y, row) in warehouse_map.rows.into_iter().enumerate() {
        let y = y as u64;
        for (x, position_type) in row.into_iter().enumerate() {
            let x = x as u64;

            if position_type == WidePositionType::Box(BoxSegment::Left) {
                gps_sum += 100 * y + x;
            }
        }
    }

    Ok(gps_sum)
}
