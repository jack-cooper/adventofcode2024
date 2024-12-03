use adventofcode::solve_day;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut program_output = 0;

    for (_, [multiplicand_a, multiplicand_b]) in regex
        .captures_iter(input)
        .map(|captures| captures.extract())
    {
        let multiplicand_a: u64 = multiplicand_a.parse()?;
        let multiplicand_b: u64 = multiplicand_b.parse()?;

        program_output += multiplicand_a * multiplicand_b;
    }

    Ok(program_output)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let input = input.replace('\n', "");
    let input = format!("do(){input}don't()");

    let regex = Regex::new(r"(?U)(?:do\(\).+don't\(\))+").unwrap();

    let parsed_input: String = regex
        .find_iter(&input)
        .flat_map(|r#match| r#match.as_str().chars())
        .collect();

    part1(&parsed_input)
}
