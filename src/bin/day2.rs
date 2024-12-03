use std::cell::OnceCell;

use adventofcode::solve_day;
use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ReportDirection {
    Ascending,
    Descending,
}

impl TryFrom<i64> for ReportDirection {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            ..0 => Ok(Self::Descending),
            0 => Err(anyhow!(
                "Attempted to determine direction of 2 equal levels."
            )),
            1.. => Ok(Self::Ascending),
        }
    }
}

fn is_safe_diff(level_diff: i64) -> bool {
    matches!(level_diff, -3..=-1 | 1..=3)
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let mut safe_report_count = 0;

    for report in input.lines() {
        let levels: Result<Vec<i64>, _> = report.split(' ').map(str::parse).collect();
        let levels = levels?;

        let report_length = levels.len();

        if report_length == 1 {
            eprintln!("Very short report detected, count could be considered to be incorrect!");
        }

        let report_direction: OnceCell<ReportDirection> = OnceCell::new();
        let mut previous_level: Option<i64> = None;

        for (index, level) in levels.into_iter().enumerate() {
            if let Some(previous_level) = previous_level {
                let level_diff = level - previous_level;

                if !is_safe_diff(level_diff) {
                    break;
                }

                let direction = ReportDirection::try_from(level_diff)?;

                if let Some(report_direction) = report_direction.get() {
                    if direction != *report_direction {
                        break;
                    }

                    if index == report_length - 1 {
                        safe_report_count += 1;
                    }
                } else {
                    let _ = report_direction.set(direction);
                }
            }

            previous_level = Some(level);
        }
    }

    Ok(safe_report_count)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let mut safe_report_count = 0;

    for report in input.lines() {
        let levels: Result<Vec<i64>, _> = report.split(' ').map(str::parse).collect();
        let mut levels = levels?;

        let report_length = levels.len();

        if report_length < 4 {
            bail!("Very short report detected, direction could not be determined!");
        }

        let first_levels: Vec<ReportDirection> = levels[..4]
            .windows(2)
            .flat_map(|level_window| {
                let &[level, next_level] = level_window else {
                    unreachable!()
                };

                ReportDirection::try_from(next_level - level)
            })
            .collect();

        let report_direction = if first_levels
            .iter()
            .filter(|&&direction| direction == ReportDirection::Ascending)
            .count()
            > 1
        {
            ReportDirection::Ascending
        } else if first_levels
            .iter()
            .filter(|&&direction| direction == ReportDirection::Descending)
            .count()
            > 1
        {
            ReportDirection::Descending
        } else {
            continue;
        };

        let is_safe = |level_diff| {
            is_safe_diff(level_diff)
                && ReportDirection::try_from(level_diff)
                    .is_ok_and(|direction| direction == report_direction)
        };

        struct RemovedLevel {
            first_try: bool,
            index: usize,
            value: i64,
        }

        let mut removed_level: Option<RemovedLevel> = None;

        let mut current_index = 0;

        while current_index < levels.len() - 1 {
            let [level, next_level] = levels[current_index..(current_index + 2)] else {
                unreachable!();
            };

            if !is_safe(next_level - level) {
                match &mut removed_level {
                    Some(removed_level) if !removed_level.first_try => break,
                    Some(removed_level) => {
                        levels[removed_level.index] = removed_level.value;

                        removed_level.first_try = false;

                        current_index = 0;
                    }
                    None => {
                        levels.remove(current_index);

                        removed_level = Some(RemovedLevel {
                            first_try: true,
                            index: current_index,
                            value: level,
                        });

                        current_index = 0;
                    }
                }
            } else {
                current_index += 1;
            }

            if current_index == levels.len() - 1 {
                safe_report_count += 1;
            }
        }
    }

    Ok(safe_report_count)
}
