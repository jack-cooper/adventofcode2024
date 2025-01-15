use core::fmt;
use std::{
    cell::OnceCell,
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
};

use adventofcode::solve_day;
use anyhow::{anyhow, bail};
use glam::IVec2;
use petgraph::{
    graph::NodeIndex,
    prelude::StableGraph,
    visit::{EdgeRef, VisitMap as _, Visitable},
};

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum PositionType {
    Empty,
    Wall,
}

impl TryFrom<char> for PositionType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | 'S' | 'E' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            _ => Err(anyhow!("Invalid maze character detected.")),
        }
    }
}

impl fmt::Debug for PositionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Wall => write!(f, "#"),
        }
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

    fn inverse(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
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
    let mut maze: Vec<Vec<PositionType>> = Vec::new();

    let start: OnceCell<IVec2> = OnceCell::new();
    let end: OnceCell<IVec2> = OnceCell::new();

    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        let mut row = Vec::new();

        for (x, char) in line.chars().enumerate() {
            let x = x as i32;

            if char == 'S' {
                start
                    .set(IVec2 { x, y })
                    .map_err(|_| anyhow!("Found more than one start position!"))?;
            } else if char == 'E' {
                end.set(IVec2 { x, y })
                    .map_err(|_| anyhow!("Found more than one end position!"))?;
            }

            let position_type = PositionType::try_from(char)?;
            row.push(position_type);
        }

        maze.push(row);
    }

    let start = start
        .into_inner()
        .ok_or(anyhow!("Didn't find a start position!"))?;
    let end = end
        .into_inner()
        .ok_or(anyhow!("Didn't find an end position!"))?;

    let (width, height) = (maze[0].len(), maze.len());

    for y in 0..height {
        'dead_end_search: for x in 0..width {
            let position_type = maze[y][x];

            if position_type == PositionType::Wall
                || IVec2::new(x as i32, y as i32) == start
                || IVec2::new(x as i32, y as i32) == end
            {
                continue;
            }

            let mut y = y;
            let mut x = x;

            loop {
                let dead_end_neighbor: OnceCell<IVec2> = OnceCell::new();
                let current_position = IVec2::new(x as i32, y as i32);

                if current_position == start || current_position == end {
                    continue 'dead_end_search;
                }

                for direction in Direction::ALL {
                    let neighbor_candidate = current_position + direction.xy();

                    if maze
                        .get(neighbor_candidate.y as usize)
                        .and_then(|row| row.get(neighbor_candidate.x as usize))
                        .copied()
                        == Some(PositionType::Empty)
                        && dead_end_neighbor.set(neighbor_candidate).is_err()
                    {
                        continue 'dead_end_search;
                    }
                }

                maze[y][x] = PositionType::Wall;

                let dead_end_neighbor = dead_end_neighbor.get().unwrap();
                x = dead_end_neighbor.x as usize;
                y = dead_end_neighbor.y as usize;
            }
        }
    }

    #[derive(Clone, Debug)]
    struct Edge {
        /// The cost to traverse the edge, accounting for corners between nodes and steps taken.
        cost: u64,
        /// The direction faced when going into the end node.
        direction_in: Direction,
        /// The direction faced when coming out from the start node.
        direction_out: Direction,
    }

    let mut graph: StableGraph<IVec2, Edge> = StableGraph::new();

    for (y, row) in maze.iter().enumerate() {
        'node_search: for (x, &position_type) in row.iter().enumerate() {
            if position_type == PositionType::Wall {
                continue;
            }

            let position = IVec2::new(x as i32, y as i32);

            if position == start || position == end {
                graph.add_node(position);
                continue;
            }

            let mut neighbor_count = 0;

            for direction in Direction::ALL {
                let neighbor_candidate = IVec2::new(x as i32, y as i32) + direction.xy();

                if maze
                    .get(neighbor_candidate.y as usize)
                    .and_then(|row| row.get(neighbor_candidate.x as usize))
                    .copied()
                    == Some(PositionType::Empty)
                {
                    neighbor_count += 1;
                }

                if neighbor_count > 2 {
                    graph.add_node(position);
                    continue 'node_search;
                }
            }
        }
    }

    // let mut output: Vec<Vec<&str>> = Vec::with_capacity(maze.len());

    // for (y, row) in maze.iter().enumerate() {
    //     output.push(Vec::with_capacity(maze[y].len()));
    //     let y = y as i32;

    //     for (x, cell) in row.iter().enumerate() {
    //         let x = x as i32;

    //         let pos = IVec2 { x, y };

    //         let symbol = if pos == start {
    //             "🟥"
    //         } else if pos == end {
    //             "🟩"
    //         } else if graph.node_weights().any(|&node_pos| node_pos == pos) {
    //             "🟦"
    //         } else {
    //             match cell {
    //                 PositionType::Empty => "⬜",
    //                 PositionType::Wall => "⬛",
    //             }
    //         };

    //         print!("{symbol}");
    //     }
    //     println!();
    // }

    let start = graph
        .node_indices()
        .find(|node| graph[*node] == start)
        .unwrap();

    let end = graph
        .node_indices()
        .find(|node| graph[*node] == end)
        .unwrap();

    let node_indices: Vec<_> = graph.node_indices().collect();

    for node in node_indices {
        for direction in Direction::ALL {
            let direction_out = direction;
            let mut direction_previous = direction;

            let mut cost = 0;
            let mut position = graph[node];

            'adjacency_search: loop {
                let mut search_directions = HashSet::from(Direction::ALL);
                search_directions.remove(&direction_previous.inverse());

                if position == graph[node] {
                    search_directions.retain(|&direction| direction == direction_out);
                }

                for direction_current in search_directions {
                    let neighbor_candidate = position + direction_current.xy();

                    if maze
                        .get(neighbor_candidate.y as usize)
                        .and_then(|row| row.get(neighbor_candidate.x as usize))
                        .is_some_and(|&position| position == PositionType::Empty)
                    {
                        cost += if direction_current == direction_previous {
                            1
                        } else {
                            1001
                        };

                        position = neighbor_candidate;
                        direction_previous = direction_current;

                        if let Some(terminating_node) = graph
                            .node_indices()
                            .find(|&node_index| graph[node_index] == position)
                        {
                            graph.add_edge(
                                node,
                                terminating_node,
                                Edge {
                                    cost,
                                    direction_in: direction_current,
                                    direction_out,
                                },
                            );

                            break 'adjacency_search;
                        } else {
                            break;
                        }
                    } else if position == graph[node] {
                        break 'adjacency_search;
                    }
                }
            }
        }
    }

    let mut visit_next: BinaryHeap<(Reverse<u64>, NodeIndex)> = BinaryHeap::new();

    let mut visited = graph.visit_map();
    let mut entry_dirs: HashMap<NodeIndex, Vec<Direction>> =
        HashMap::from([(start, vec![Direction::East])]);
    let mut path_lengths = HashMap::new();

    path_lengths.insert(start, 0);
    visit_next.push((Reverse(0), start));

    while let Some((Reverse(node_score), node)) = visit_next.pop() {
        if visited.is_visited(&node) {
            continue;
        }

        if node == end {
            break;
        }

        let directions = entry_dirs[&node].clone();

        for edge in graph.edges(node) {
            let next = edge.target();

            if visited.is_visited(&next) {
                continue;
            }

            // Add 1000 to cost if we couldn't enter this node facing the direction we're going to leave it facing
            let turning_cost = if directions.contains(&edge.weight().direction_out) {
                0
            } else {
                1000
            };

            let next_score = node_score + edge.weight().cost + turning_cost;

            match path_lengths.entry(next) {
                Entry::Occupied(mut ent) => {
                    let score = ent.get_mut();

                    if next_score <= *score {
                        if next_score < *score {
                            *score = next_score;
                            visit_next.push((Reverse(next_score), next));

                            entry_dirs.get_mut(&next).unwrap().clear();
                        }

                        entry_dirs
                            .entry(next)
                            .or_insert_with(|| unreachable!())
                            .push(edge.weight().direction_in);
                    }
                }
                Entry::Vacant(ent) => {
                    ent.insert(next_score);
                    visit_next.push((Reverse(next_score), next));

                    if entry_dirs
                        .insert(next, vec![edge.weight().direction_in])
                        .is_some()
                    {
                        panic!();
                    }
                }
            }
        }

        visited.visit(node);
    }

    let mut counts: HashMap<usize, u64> = HashMap::new();
    for (node_index, dirvec) in entry_dirs {
        *counts.entry(dirvec.len()).or_default() += 1;
    }

    dbg!(counts);

    let length = path_lengths[&end];

    Ok(length)
}

fn part2(_input: &str) -> anyhow::Result<u64> {
    bail!("Unimplemented");
}
