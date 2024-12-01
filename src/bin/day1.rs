use std::collections::HashMap;

use adventofcode::solve_day;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let mut group1_ids = Vec::with_capacity(1_000);
    let mut group2_ids = Vec::with_capacity(1_000);

    for line in input.lines() {
        let mut location_ids = line.split_whitespace();

        let group1_id: u64 = location_ids
            .next()
            .ok_or(anyhow!("Line was missing group 1 location ID."))?
            .parse()?;
        let group2_id: u64 = location_ids
            .next()
            .ok_or(anyhow!("Line was missing group 2 location ID."))?
            .parse()?;

        group1_ids.push(group1_id);
        group2_ids.push(group2_id);
    }

    group1_ids.sort_unstable();
    group2_ids.sort_unstable();

    let distance_sum = group1_ids
        .into_iter()
        .zip(group2_ids)
        .map(|(id1, id2)| id1.abs_diff(id2))
        .sum();

    Ok(distance_sum)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let mut group1_ids = Vec::with_capacity(1_000);
    let mut group2_ids = Vec::with_capacity(1_000);

    for line in input.lines() {
        let mut location_ids = line.split_whitespace();

        let group1_id: u64 = location_ids
            .next()
            .ok_or(anyhow!("Line was missing group 1 location ID."))?
            .parse()?;
        let group2_id: u64 = location_ids
            .next()
            .ok_or(anyhow!("Line was missing group 2 location ID."))?
            .parse()?;

        group1_ids.push(group1_id);
        group2_ids.push(group2_id);
    }

    let mut group2_id_sums: HashMap<u64, u64> = HashMap::new();

    for location_id in group2_ids {
        *group2_id_sums.entry(location_id).or_default() += location_id;
    }

    let similarity_score = group1_ids
        .into_iter()
        .map(|location_id| {
            group2_id_sums
                .get(&location_id)
                .copied()
                .unwrap_or_default()
        })
        .sum();

    Ok(similarity_score)
}
