use std::sync::LazyLock;

use adventofcode::solve_day;
use anyhow::bail;
use glam::{DMat2, DVec2, Mat2, Vec2};
use regex::Regex;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

static BUTTON_A_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Button A: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap());

static BUTTON_B_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Button B: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap());

static PRIZE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap());

fn part1(input: &str) -> anyhow::Result<u64> {
    const TOLERANCE: f32 = 0.001;

    struct ClawMachine {
        movement_a: Vec2,
        movement_b: Vec2,
        prize: Vec2,
    }

    impl ClawMachine {
        const A_COST: u64 = 3;
        const B_COST: u64 = 1;
    }

    let mut total_tokens = 0;

    for raw_machine in input.split("\n\n") {
        let Some(captures) = BUTTON_A_REGEX.captures(raw_machine) else {
            bail!("Button A requires an X and Y value.");
        };

        let ax: f32 = captures["x"].parse()?;
        let ay: f32 = captures["y"].parse()?;

        let Some(captures) = BUTTON_B_REGEX.captures(raw_machine) else {
            bail!("Button B requires an X and Y value.");
        };

        let bx: f32 = captures["x"].parse()?;
        let by: f32 = captures["y"].parse()?;

        let Some(captures) = PRIZE_REGEX.captures(raw_machine) else {
            bail!("The prize requiers an X and Y value.");
        };

        let prize_x: f32 = captures["x"].parse()?;
        let prize_y: f32 = captures["y"].parse()?;

        let claw_machine = ClawMachine {
            movement_a: Vec2::new(ax, ay),
            movement_b: Vec2::new(bx, by),
            prize: Vec2::new(prize_x, prize_y),
        };

        let mat = Mat2::from_cols(claw_machine.movement_a, claw_machine.movement_b);

        let button_presses = mat.inverse() * claw_machine.prize;

        if button_presses.max_element() > 100.0 + TOLERANCE
            || (button_presses.x - button_presses.x.round()).abs() > TOLERANCE
            || (button_presses.y - button_presses.y.round()).abs() > TOLERANCE
        {
            continue;
        }

        let a_tokens = button_presses.x.round() as u64;
        let b_tokens = button_presses.y.round() as u64;

        total_tokens += ClawMachine::A_COST * a_tokens + ClawMachine::B_COST * b_tokens;
    }

    Ok(total_tokens)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    const TOLERANCE: f64 = 0.001;

    struct ClawMachine {
        movement_a: DVec2,
        movement_b: DVec2,
        prize: DVec2,
    }

    impl ClawMachine {
        const A_COST: u64 = 3;
        const B_COST: u64 = 1;
    }

    let mut total_tokens = 0;

    for raw_machine in input.split("\n\n") {
        let Some(captures) = BUTTON_A_REGEX.captures(raw_machine) else {
            bail!("Button A requires an X and Y value.");
        };

        let ax: f64 = captures["x"].parse()?;
        let ay: f64 = captures["y"].parse()?;

        let Some(captures) = BUTTON_B_REGEX.captures(raw_machine) else {
            bail!("Button B requires an X and Y value.");
        };

        let bx: f64 = captures["x"].parse()?;
        let by: f64 = captures["y"].parse()?;

        let Some(captures) = PRIZE_REGEX.captures(raw_machine) else {
            bail!("The prize requiers an X and Y value.");
        };

        let prize_x: f64 = captures["x"].parse()?;
        let prize_y: f64 = captures["y"].parse()?;

        let claw_machine = ClawMachine {
            movement_a: DVec2::new(ax, ay),
            movement_b: DVec2::new(bx, by),
            prize: DVec2::new(prize_x, prize_y) + DVec2::splat(10_000_000_000_000.0),
        };

        let mat = DMat2::from_cols(claw_machine.movement_a, claw_machine.movement_b);

        let button_presses = mat.inverse() * claw_machine.prize;

        if (button_presses.x - button_presses.x.round()).abs() > TOLERANCE
            || (button_presses.y - button_presses.y.round()).abs() > TOLERANCE
        {
            continue;
        }

        let a_tokens = button_presses.x.round() as u64;
        let b_tokens = button_presses.y.round() as u64;

        total_tokens += ClawMachine::A_COST * a_tokens + ClawMachine::B_COST * b_tokens;
    }

    Ok(total_tokens)
}
