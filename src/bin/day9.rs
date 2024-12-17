use std::{cmp::Ordering, fmt};

use adventofcode::solve_day;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

#[derive(Clone, Copy)]
struct Chunk {
    block: Block,
    length: u64,
}

#[derive(Clone, Copy)]
struct Block {
    block_type: BlockType,
    id: u64,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum BlockType {
    File,
    Free,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self.block.block_type {
            BlockType::File => self.block.id.to_string(),
            BlockType::Free => ".".to_string(),
        };

        write!(f, "{}", char.repeat(self.length as usize))
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            block_type: BlockType::File,
            id: 0,
        }
    }
}

impl Block {
    fn next(self) -> Self {
        match self.block_type {
            BlockType::File => Self {
                block_type: BlockType::Free,
                id: self.id,
            },
            BlockType::Free => Self {
                block_type: BlockType::File,
                id: self.id + 1,
            },
        }
    }
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let mut current_block = Block::default();

    let mut disk: Vec<Chunk> = Vec::with_capacity(input.len());

    for char in input.trim_ascii_end().chars() {
        let block_size = u64::from(
            char.to_digit(10)
                .ok_or(anyhow!("Attempted to parse char outside the range 0-9"))?,
        );

        disk.push(Chunk {
            block: current_block,
            length: block_size,
        });

        current_block = current_block.next();
    }

    while let Some(first_free_chunk_index) = disk
        .iter()
        .position(|chunk| chunk.block.block_type == BlockType::Free)
    {
        // The above while condition can only return `true` if there is at least one element.
        let &last_chunk = disk.last().unwrap();

        if let BlockType::Free = last_chunk.block.block_type {
            disk.pop();
            continue;
        }

        let first_free_chunk = &mut disk[first_free_chunk_index];

        match first_free_chunk.length.cmp(&last_chunk.length) {
            Ordering::Less => {
                first_free_chunk.block = last_chunk.block;
                disk.last_mut().unwrap().length -= first_free_chunk.length;
            }
            Ordering::Equal => {
                first_free_chunk.block = last_chunk.block;
                disk.pop();
            }
            Ordering::Greater => {
                first_free_chunk.length -= last_chunk.length;
                disk.insert(first_free_chunk_index, last_chunk);
                disk.pop();
            }
        }
    }

    let mut checksum: u64 = 0;
    let mut chunk_position = 0;

    for chunk in disk {
        checksum += (chunk_position..(chunk_position + chunk.length))
            .map(|position| position * chunk.block.id)
            .sum::<u64>();

        chunk_position += chunk.length;
    }

    Ok(checksum)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let mut current_block = Block::default();

    let mut disk: Vec<Chunk> = Vec::with_capacity(input.len());

    for char in input.trim_ascii_end().chars() {
        let block_size = u64::from(
            char.to_digit(10)
                .ok_or(anyhow!("Attempted to parse char outside the range 0-9"))?,
        );

        disk.push(Chunk {
            block: current_block,
            length: block_size,
        });

        current_block = current_block.next();
    }

    for file_id in (1..=current_block.id).rev() {
        // The chunk with the given `file_id` is guaranteed to exist in `disk`.
        // It is also guaranteed to come before the empty chunk with the same ID.
        let (current_chunk_index, &current_chunk) = disk
            .iter()
            .enumerate()
            .find(|(_, chunk)| chunk.block.id == file_id)
            .unwrap();

        let Some((free_chunk_index, free_chunk)) = disk[..current_chunk_index]
            .iter_mut()
            .enumerate()
            .find(|(_, chunk)| {
                chunk.block.block_type == BlockType::Free && chunk.length >= current_chunk.length
            })
        else {
            continue;
        };

        free_chunk.length -= current_chunk.length;
        disk[current_chunk_index] = Chunk {
            block: Block {
                block_type: BlockType::Free,
                id: 0,
            },
            ..current_chunk
        };
        disk.insert(free_chunk_index, current_chunk);
    }

    let mut checksum: u64 = 0;
    let mut chunk_position = 0;

    for chunk in disk {
        if chunk.block.block_type == BlockType::File {
            checksum += (chunk_position..(chunk_position + chunk.length))
                .map(|position| position * chunk.block.id)
                .sum::<u64>();
        }

        chunk_position += chunk.length;
    }

    Ok(checksum)
}
