use std::{collections::HashMap, str::FromStr, sync::LazyLock};

use adventofcode::solve_day;
use anyhow::{anyhow, bail};
use glam::IVec2;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

const EBHQ_DIMENSIONS: IVec2 = IVec2::new(101, 103);
const MIDPOINT: IVec2 = IVec2::new(EBHQ_DIMENSIONS.x / 2, EBHQ_DIMENSIONS.y / 2);

static ROBOT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap()
});

struct Robot {
    position: IVec2,
    velocity: IVec2,
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Quadrant {
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = ROBOT_REGEX
            .captures(s)
            .ok_or(anyhow!("Unable to parse robot descriptor."))?;

        let px: i32 = captures["px"].parse()?;
        let py: i32 = captures["py"].parse()?;
        let vx: i32 = captures["vx"].parse()?;
        let vy: i32 = captures["vy"].parse()?;

        Ok(Self {
            position: IVec2::new(px, py),
            velocity: IVec2::new(vx, vy),
        })
    }
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let robots: anyhow::Result<Vec<Robot>> = input.lines().map(str::parse).collect();
    let mut robots = robots?;

    for _ in 0..100 {
        for robot in &mut robots {
            robot.position += robot.velocity;
            robot.position = robot.position.rem_euclid(EBHQ_DIMENSIONS);
        }
    }

    let mut quadrants: HashMap<Quadrant, u64> = HashMap::new();

    for robot in &robots {
        let quadrant = match robot.position {
            pos if pos.x < MIDPOINT.x && pos.y < MIDPOINT.y => Quadrant::Northwest,
            pos if pos.x < MIDPOINT.x && pos.y > MIDPOINT.y => Quadrant::Southwest,
            pos if pos.x > MIDPOINT.x && pos.y < MIDPOINT.y => Quadrant::Northeast,
            pos if pos.x > MIDPOINT.x && pos.y > MIDPOINT.y => Quadrant::Southeast,
            _ => continue,
        };

        *quadrants.entry(quadrant).or_default() += 1;
    }

    let safety_factor = quadrants.values().product();
    Ok(safety_factor)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let robots: anyhow::Result<Vec<Robot>> = input.lines().map(str::parse).collect();
    let mut robots = robots?;

    if robots.is_empty() {
        bail!("Empty input detected.");
    }

    let mut seconds_elapsed = 0;

    'outer: loop {
        for robot in &robots {
            let total_distance: i32 = robots
                .iter()
                .map(|other_robot| robot.position.distance_squared(other_robot.position))
                .sum();

            let average_distance = total_distance as usize / robots.len();

            if average_distance < 1000 {
                break 'outer;
            }
        }

        for robot in &mut robots {
            robot.position += robot.velocity;
            robot.position = robot.position.rem_euclid(EBHQ_DIMENSIONS);
        }

        seconds_elapsed += 1;
    }

    Ok(seconds_elapsed)
}
