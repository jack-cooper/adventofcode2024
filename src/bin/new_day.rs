use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::anyhow;

use adventofcode::INITIALS;

fn main() -> anyhow::Result<()> {
    let day = env::args()
        .nth(1)
        .ok_or(anyhow!("Day argument not passed."))?;

    // Assumed to always be running from `CARGO_MANIFEST_DIR`
    let path = PathBuf::new();

    // e.g. `input/day1`
    let input_dir_path = path.join("input").join(&day);

    // Returns error if dir already exists
    fs::create_dir(&input_dir_path)?;

    for initials in INITIALS {
        let file_name = format!("{initials}.txt");

        // e.g. `input/day1/xmp.txt`
        let file_path = input_dir_path.join(file_name);

        File::create_new(file_path)?;
    }

    let binary_file_name = format!("{day}.rs");
    let binary_file_path = path.join("src/bin").join(binary_file_name);

    let mut binary_file = File::create_new(binary_file_path)?;
    binary_file.write_all(DEFAULT_BINARY_TEMPLATE.as_bytes())?;

    Ok(())
}

const DEFAULT_BINARY_TEMPLATE: &str = r#"
use adventofcode::solve_day;
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

fn part1(_input: &str) -> anyhow::Result<u64> {
    bail!("Unimplemented");
}

fn part2(_input: &str) -> anyhow::Result<u64> {
    bail!("Unimplemented");
}
"#
.trim_ascii_start();
