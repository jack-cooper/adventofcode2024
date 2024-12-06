use std::collections::{HashMap, HashSet};

use adventofcode::solve_day;
use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    solve_day(file!(), part1, part2)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Page(u64);

fn part1(input: &str) -> anyhow::Result<u64> {
    let (ordering_rules_raw, updates) = input
        .split_once("\n\n")
        .ok_or(anyhow!("Input is not comprised of 2 sections."))?;

    let mut ordering_rules: HashMap<Page, HashSet<Page>> = HashMap::new();

    for ordering_rule in ordering_rules_raw.lines() {
        let (earlier_page, later_page) = ordering_rule
            .split_once('|')
            .ok_or(anyhow!("Ordering rule does not have a separator."))?;

        let earlier_page = earlier_page.parse()?;
        let later_page = later_page.parse()?;

        ordering_rules
            .entry(Page(earlier_page))
            .or_default()
            .insert(Page(later_page));
    }

    let ordering_rules = ordering_rules;

    let mut middle_page_sum = 0;

    'updates: for update in updates.lines() {
        let mut seen_pages: HashSet<Page> = HashSet::new();

        let pages: anyhow::Result<Vec<Page>> = update
            .split(',')
            .map(|page| Ok(Page(page.parse()?)))
            .collect();
        let pages = pages?;

        if pages.len() % 2 == 0 {
            bail!("The number of page numbers for a given update should be odd.");
        }

        for &page in &pages {
            if ordering_rules
                .get(&page)
                .is_some_and(|following_pages| !following_pages.is_disjoint(&seen_pages))
            {
                continue 'updates;
            }

            seen_pages.insert(page);
        }

        let Page(middle_page) = pages[pages.len() / 2];
        middle_page_sum += middle_page;
    }

    Ok(middle_page_sum)
}

fn part2(_input: &str) -> anyhow::Result<u64> {
    bail!("Unimplemented");
}
