use std::{
    num::ParseIntError,
    ops::{AddAssign, MulAssign},
    str::FromStr,
};

use adventofcode::solve_day;
use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

struct Equation {
    operands: Vec<u64>,
    test_value: u64,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_value, operands) = s
            .split_once(':')
            .ok_or(anyhow!("Equation was missing a `:` character"))?;

        let test_value: u64 = test_value.parse()?;

        let operands: Result<Vec<u64>, ParseIntError> =
            operands.split_whitespace().map(str::parse::<u64>).collect();

        let operands = operands?;

        if operands.len() < 2 {
            bail!("An equation should have at least 2 operands.")
        }

        Ok(Self {
            operands,
            test_value,
        })
    }
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let equations: anyhow::Result<Vec<Equation>> =
        input.lines().map(str::parse::<Equation>).collect();

    let equations = equations?;

    let mut total_calibration_result = 0;

    for equation in equations {
        let operator_count = equation.operands.len() - 1;

        let operator_combinations = 2_u16.pow(operator_count as u32);

        for operator_bitset in 0..operator_combinations {
            let mut total = equation.operands[0];

            for offset in (0..operator_count).rev() {
                let next_operand = equation.operands[operator_count - offset];

                let operator_check = operator_bitset & (1 << offset);

                let operator = if operator_check == 0 {
                    <u64 as MulAssign<u64>>::mul_assign
                } else {
                    AddAssign::add_assign
                };

                operator(&mut total, next_operand);

                if total > equation.test_value {
                    break;
                }
            }

            if total == equation.test_value {
                total_calibration_result += equation.test_value;
                break;
            }
        }
    }

    Ok(total_calibration_result)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let equations: anyhow::Result<Vec<Equation>> =
        input.lines().map(str::parse::<Equation>).collect();

    let equations = equations?;

    let mut total_calibration_result = 0;

    for equation in equations {
        let operator_count = equation.operands.len() - 1; // 2

        let operator_combinations = 3_u32.pow(operator_count as u32); // 9

        for mut operator_tritset in 0..operator_combinations {
            let mut total = equation.operands[0];

            for offset in (0..operator_count).rev() {
                // 0 or 1
                let next_operand = equation.operands[operator_count - offset];

                // 000 001 002 010 011 012 020 021 022
                //  00  01  02  10  11  12  20  21  22

                let operator_check = operator_tritset / 3_u32.pow(offset as u32);
                operator_tritset %= 3_u32.pow(offset as u32);

                fn concatenate_int(concatenatee: &mut u64, concatenator: u64) {
                    let next_power_of_ten = 10_u64.pow(concatenator.ilog10() + 1);

                    *concatenatee *= next_power_of_ten;
                    *concatenatee += concatenator
                }

                let operator = if operator_check == 0 {
                    concatenate_int
                } else if operator_check == 1 {
                    MulAssign::mul_assign
                } else {
                    AddAssign::add_assign
                };

                operator(&mut total, next_operand);

                if total > equation.test_value {
                    break;
                }
            }

            if total == equation.test_value {
                total_calibration_result += equation.test_value;
                break;
            }
        }
    }

    Ok(total_calibration_result)
}
