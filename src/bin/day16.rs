use core::fmt;
use std::{
    cell::OnceCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use adventofcode::solve_day;
use anyhow::anyhow;
use glam::IVec2;
use petgraph::{
    algo,
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, VisitMap as _, Visitable as _},
    Graph,
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

fn part2(input: &str) -> anyhow::Result<u64> {
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

    #[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
    struct PathSegment {
        priority: Reverse<u64>,
        node: NodeIndex,
        edge: Option<EdgeIndex>,
    }

    let mut visited = graph.visit_map();
    let mut visit_next: BinaryHeap<PathSegment> = BinaryHeap::new();

    visit_next.push(PathSegment {
        priority: Reverse(0),
        node: start,
        edge: None,
    });

    // "Optimal path segments" is keyed by a node index, and has a value of:
    // * The optimal score we reached this node with
    // * A vector of incident edges used to reach this node with the above optimal score, along with a copy of this node's index (definitely redundant)

    // We don't really need a full map here, but it's useful to be able to index the map at `end` later on.
    let mut optimal_path_segments: HashMap<NodeIndex, (u64, Vec<(EdgeIndex, NodeIndex)>)> =
        HashMap::new();

    while let Some(PathSegment {
        priority: Reverse(node_score),
        node,
        edge: incident_edge,
    }) = visit_next.pop()
    {
        // If we've already visited this node, check to see if we've found another optimal way to visit this node.
        if visited.is_visited(&node) {
            let (optimal_node_score, optimal_incident_edges) =
                optimal_path_segments.get_mut(&node).unwrap();

            if node_score == *optimal_node_score {
                optimal_incident_edges.extend(incident_edge.map(|edge| (edge, node)));
            }

            continue;
        // If this is the first time we're seeing the node, this must be the optimal way to reach it!
        } else {
            optimal_path_segments.insert(
                node,
                (
                    node_score,
                    Vec::from_iter(incident_edge.map(|edge| (edge, node))),
                ),
            );
        }

        for edge in graph.edges(node) {
            let next = edge.target();

            // If we have already visited the next node, skip adding it to the priority queue.
            // "Visited" in this context means we've already evaluated all edges leading out from that node.
            if visited.is_visited(&next) {
                continue;
            }

            let node_cost = graph[edge.source()].cost;
            let edge_cost = edge.weight().cost;

            let next_score = node_cost + node_score + edge_cost;

            visit_next.push(PathSegment {
                priority: Reverse(next_score),
                node: next,
                edge: Some(edge.id()),
            });
        }

        visited.visit(node);
    }

    // Set of positions which lie on any optimal path between `start` and `end`
    let mut optimal_path_positions: HashSet<IVec2> = HashSet::new();

    let (_, mut optimal_incident_edges) = optimal_path_segments.remove(&end).unwrap();

    while let Some((edge, node)) = optimal_incident_edges.pop() {
        let &(endpoint_a, endpoint_b) = &graph.edge_endpoints(edge).unwrap();

        // When travelling from `start` to `end`, `source` is the first of the 2 endpoints
        // we will encounter. This allows us to define a backwards "direction" through our
        // undirected graph.
        let source = if endpoint_a == node {
            endpoint_b
        } else {
            endpoint_a
        };

        // `source_optimal_incident_edges` are incident edges for node `source` which
        // are on the optimal path to `source`.
        let (_, source_optimal_incident_edges) = &optimal_path_segments[&source];
        optimal_incident_edges.extend_from_slice(source_optimal_incident_edges);

        let (endpoint_a_pos, endpoint_b_pos) =
            (graph[endpoint_a].position, graph[endpoint_b].position);

        let (min_pos, max_pos) = (
            endpoint_a_pos.min(endpoint_b_pos),
            endpoint_a_pos.max(endpoint_b_pos),
        );

        let horizontal = min_pos.y == max_pos.y;

        let mut current_pos = min_pos;

        if horizontal {
            while current_pos.x <= max_pos.x {
                optimal_path_positions.insert(current_pos);
                current_pos += IVec2::X;
            }
        } else {
            while current_pos.y <= max_pos.y {
                optimal_path_positions.insert(current_pos);
                current_pos += IVec2::Y;
            }
        }
    }

    Ok(optimal_path_positions.len() as u64)
}
