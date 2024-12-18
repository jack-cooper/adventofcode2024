use std::{borrow::Cow, fs, path::Path};

use anyhow::anyhow;

pub const INITIALS: [&str; 5] = ["xmp", "jwc", "scb", "slh", "tmf"];
const COLUMN_PADDING: usize = 3;

fn day(source_file: &str) -> anyhow::Result<Cow<'_, str>> {
    Ok(Path::new(source_file)
        .file_stem()
        .ok_or(anyhow!("Empty source file name"))?
        .to_string_lossy())
}

pub fn solve_day(
    source_file: &str,
    part1: fn(&str) -> anyhow::Result<u64>,
    part2: fn(&str) -> anyhow::Result<u64>,
) -> anyhow::Result<()> {
    let day = day(source_file)?;

    let day_number: String = day.chars().skip(3).collect();
    let day_header = format!("Day {day_number}");

    let headers = ["Initials", "Part 1", "Part 2", "Error"];
    let mut column_widths = headers.map(str::len);

    let mut answers = Vec::with_capacity(5);

    for initials in INITIALS {
        let input = fs::read_to_string(format!("input/{day}/{initials}.txt"))?;

        if input.is_empty() {
            answers.push((Err(anyhow!("Empty input")), Err(anyhow!("Empty input"))));
        } else {
            answers.push((part1(&input), part2(&input)));
        }
    }

    for (part1_answer, part2_answer) in &answers {
        match part1_answer {
            &Ok(answer) => {
                column_widths[1] = column_widths[1].max(number_len(answer));

                if let Err(err) = part2_answer {
                    column_widths[3] = column_widths[3].max(err.to_string().len());
                }
            }
            Err(err) => column_widths[3] = column_widths[3].max(err.to_string().len()),
        };

        if let &Ok(answer) = part2_answer {
            column_widths[2] = column_widths[2].max(number_len(answer));
        };
    }

    let table_width =
        column_widths.into_iter().sum::<usize>() + COLUMN_PADDING * column_widths.len() - 1;

    let initials_column_width = column_widths[0];
    let part1_column_width = column_widths[1];
    let part2_column_width = column_widths[2];
    let error_column_width = column_widths[3];

    println!(
        "╔═{:═^initials_column_width$}═══{:═^part1_column_width$}═══{:═^part2_column_width$}═══{:═^error_column_width$}═╗",
        "", "", "", ""
    );

    println!("║{:^table_width$}║", day_header);

    println!(
        "╠═{:═^initials_column_width$}═╦═{:═^part1_column_width$}═╦═{:═^part2_column_width$}═╦═{:═^error_column_width$}═╣",
        "", "", "", "",
    );

    println!(
        "║ {} ║ {:^part1_column_width$} ║ {:^part2_column_width$} ║ {:^error_column_width$} ║",
        headers[0], headers[1], headers[2], headers[3],
    );

    println!(
        "╠═{:═^initials_column_width$}═╬═{:═^part1_column_width$}═╬═{:═^part2_column_width$}═╬═{:═^error_column_width$}═╣",
        "", "", "", ""
    );

    for (index, (answer1, answer2)) in answers.into_iter().enumerate() {
        let initials = INITIALS[index].to_uppercase();

        let mut part1_column_width = part1_column_width;

        let part1 = match answer1 {
            Ok(answer) => answer.to_string(),
            Err(_) => {
                part1_column_width += 11;
                String::from("\x1b[1;31m[ERR]\x1b[0m")
            }
        };

        let mut part2_column_width = part2_column_width;

        let part2 = match answer2 {
            Ok(answer) => answer.to_string(),
            Err(_) => {
                part2_column_width += 11;

                String::from("\x1b[1;31m[ERR]\x1b[0m")
            }
        };

        let mut error_column_width = error_column_width;
        let err = match answer1.and(answer2) {
            Ok(_) => String::from("-"),
            Err(err) => {
                error_column_width += 11;
                format!("\x1b[0;31m{}\x1b[0m", err)
            }
        };

        println!(
            "║ {:^initials_column_width$} ║ {:^part1_column_width$} ║ {:^part2_column_width$} ║ {:^error_column_width$} ║",
            initials, part1, part2, err
        );
    }

    println!(
        "╚═{:═^initials_column_width$}═╩═{:═^part1_column_width$}═╩═{:═^part2_column_width$}═╩═{:═^error_column_width$}═╝",
        "", "", "", ""
    );

    Ok(())
}

fn number_len(number: u64) -> usize {
    match number {
        0 => 1,
        1.. => (number.ilog10() + 1) as usize,
    }
}
