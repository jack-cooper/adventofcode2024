use std::ops::Index;

use adventofcode::solve_day;
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    East,
    North,
    Northeast,
    Northwest,
    South,
    Southeast,
    Southwest,
    West,
}

impl Direction {
    const ALL: [Direction; 8] = [
        Direction::East,
        Direction::North,
        Direction::Northeast,
        Direction::Northwest,
        Direction::South,
        Direction::Southeast,
        Direction::Southwest,
        Direction::West,
    ];

    fn opposite(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::North => Self::South,
            Self::Northeast => Self::Southwest,
            Self::Northwest => Self::Southeast,
            Self::South => Self::North,
            Self::Southeast => Self::Northwest,
            Self::Southwest => Self::Northeast,
            Self::West => Self::East,
        }
    }

    /// Returns the amount needed to move in the x and y axes to move in this direction.
    /// Note that -y is up, because we iterate down through the wordsearch.
    fn xy(self) -> IVec2 {
        match self {
            Self::East => IVec2::X,
            Self::North => IVec2::NEG_Y,
            Self::Northeast => IVec2::new(1, -1),
            Self::Northwest => IVec2::NEG_ONE,
            Self::South => IVec2::Y,
            Self::Southeast => IVec2::ONE,
            Self::Southwest => IVec2::new(-1, 1),
            Self::West => IVec2::NEG_X,
        }
    }
}

struct WordSearch {
    col_count: i32,
    rows: Vec<Vec<char>>,
    row_count: i32,
}

impl Index<usize> for WordSearch {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        self.rows.index(index)
    }
}

impl<I2: IntoIterator<Item = char>> FromIterator<I2> for WordSearch {
    fn from_iter<I1>(iter: I1) -> Self
    where
        I1: IntoIterator<Item = I2>,
    {
        let rows: Vec<Vec<char>> = iter
            .into_iter()
            .map(|char_iter| char_iter.into_iter().collect())
            .collect();

        assert!(!rows.is_empty(), "No rows in word search.");

        let row_count = rows.len();
        let col_count = rows[0].len();

        assert!(
            rows[1..].iter().all(|row| row.len() == col_count),
            "Word search rows are not all of equal length."
        );

        Self {
            col_count: col_count as i32,
            rows,
            row_count: row_count as i32,
        }
    }
}

fn part1(input: &str) -> anyhow::Result<u64> {
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

    fn valid_directions(word_search: &WordSearch, cell: IVec2) -> Vec<Direction> {
        Direction::ALL
            .into_iter()
            .filter(|direction| {
                let offset = direction.xy() * (XMAS.len() as i32 - 1);

                let x_bounds = 0..word_search.col_count;
                let y_bounds = 0..word_search.row_count;

                let final_position = cell + offset;

                x_bounds.contains(&final_position.x) && y_bounds.contains(&final_position.y)
            })
            .collect()
    }

    let word_search: WordSearch = input.lines().map(|row| row.chars()).collect();

    let mut xmas_count = 0;

    for (y, row) in word_search.rows.iter().enumerate() {
        for x in row
            .iter()
            .enumerate()
            .filter_map(|(x, &char)| (char == 'X').then_some(x))
        {
            let cell = IVec2::new(x as i32, y as i32);

            for direction in valid_directions(&word_search, cell) {
                for (xmas_index, &xmas_char) in XMAS[1..].iter().enumerate() {
                    let xmas_index = (xmas_index + 1) as i32;

                    let next_cell = cell + direction.xy() * xmas_index;
                    let next_char = word_search[next_cell.y as usize][next_cell.x as usize];

                    if next_char != xmas_char {
                        break;
                    } else if xmas_char == 'S' {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    Ok(xmas_count)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let word_search: WordSearch = input.lines().map(|row| row.chars()).collect();

    let mut x_mas_count = 0;

    for (y, row) in word_search.rows.iter().enumerate() {
        for x in row.iter().enumerate().filter_map(|(x, &char)| {
            let x = x as i32;
            let y = y as i32;

            let in_bounds =
                || x > 0 && x < word_search.col_count - 1 && y > 0 && y < word_search.row_count - 1;

            (char == 'A' && in_bounds()).then_some(x)
        }) {
            let cell = IVec2::new(x, y as i32);

            for direction in [Direction::Northeast, Direction::Northwest] {
                let adjacent_cell = cell + direction.xy();
                let adjacent_char = word_search[adjacent_cell.y as usize][adjacent_cell.x as usize];

                let opposite_adjacent_cell = cell + direction.opposite().xy();
                let opposite_adjacent_char = word_search[opposite_adjacent_cell.y as usize]
                    [opposite_adjacent_cell.x as usize];

                if !((adjacent_char == 'M' && opposite_adjacent_char == 'S')
                    || (adjacent_char == 'S' && opposite_adjacent_char == 'M'))
                {
                    break;
                }

                if direction == Direction::Northwest {
                    x_mas_count += 1;
                }
            }
        }
    }

    Ok(x_mas_count)
}
