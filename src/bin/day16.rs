use core::fmt;
use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
};

use adventofcode::solve_day;
use anyhow::{anyhow, bail};
use glam::IVec2;
use petgraph::{algo, graph::NodeIndex, visit::EdgeRef, Graph};

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

#[derive(Clone, Copy)]
struct Node {
    cost: u64,
    position: IVec2,
}

struct Edge {
    cost: u64,
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

    let mut graph: Graph<Node, Edge, _> = Graph::new_undirected();

    for (y, row) in maze.iter().enumerate() {
        let y = y as i32;
        for (x, &position_type) in row.iter().enumerate() {
            let x = x as i32;

            if position_type == PositionType::Wall {
                continue;
            }

            let position = IVec2 { x, y };

            if position == start || position == end {
                graph.add_node(Node { cost: 0, position });
                continue;
            }

            let mut neighbor_directions: HashSet<Direction> = HashSet::new();

            for direction in Direction::ALL {
                let neighbor_candidate = position + direction.xy();

                if maze
                    .get(neighbor_candidate.y as usize)
                    .and_then(|row| row.get(neighbor_candidate.x as usize))
                    .is_some_and(|&position_type| position_type == PositionType::Empty)
                {
                    if neighbor_directions
                        .iter()
                        .any(|&neighbor_direction| neighbor_direction != direction.inverse())
                    {
                        graph.add_node(Node {
                            cost: 1000,
                            position,
                        });
                        break;
                    } else {
                        neighbor_directions.insert(direction);
                    }
                }
            }
        }
    }

    let position_to_node: HashMap<IVec2, NodeIndex> = graph
        .node_indices()
        .map(|node_index| (graph[node_index].position, node_index))
        .collect();

    for node in graph.node_indices() {
        let weight = graph[node];

        for direction in Direction::ALL {
            let mut position = weight.position;

            loop {
                position += direction.xy();

                if maze
                    .get(position.y as usize)
                    .and_then(|row| row.get(position.x as usize))
                    .is_some_and(|&position_type| position_type == PositionType::Wall)
                {
                    break;
                }

                let Some(&other_node) = position_to_node.get(&position) else {
                    continue;
                };

                let mut path_length = (weight.position - position).abs().max_element() as u64;

                if (weight.position == start && direction != Direction::East)
                    || (graph[other_node].position == start && direction != Direction::West)
                {
                    path_length += 1000;
                }

                graph.update_edge(node, other_node, Edge { cost: path_length });
            }
        }
    }

    let start = graph
        .node_indices()
        .find(|&node| graph[node].position == start)
        .unwrap();
    let end = graph
        .node_indices()
        .find(|&node| graph[node].position == end)
        .unwrap();

    let path_lengths = algo::dijkstra(&graph, start, Some(end), |edge| {
        let node_cost = graph[edge.source()].cost;
        let edge_cost = edge.weight().cost;

        node_cost + edge_cost
    });

    Ok(path_lengths[&end])
}

fn part2(_input: &str) -> anyhow::Result<u64> {
    bail!("Unimplemented");
}
