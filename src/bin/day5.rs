use std::collections::{HashMap, HashSet, VecDeque};

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
        let pages: anyhow::Result<Vec<Page>> = update
            .split(',')
            .map(|page| Ok(Page(page.parse()?)))
            .collect();
        let pages = pages?;

        if pages.len() % 2 == 0 {
            bail!("The number of page numbers for a given update should be odd.");
        }

        let mut seen_pages: HashSet<Page> = HashSet::new();

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

fn part2(input: &str) -> anyhow::Result<u64> {
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

    for update in updates.lines() {
        let pages: anyhow::Result<Vec<Page>> = update
            .split(',')
            .map(|page| Ok(Page(page.parse()?)))
            .collect();
        let pages = pages?;

        if pages.len() % 2 == 0 {
            bail!("The number of page numbers for a given update should be odd.");
        }

        let mut seen_pages: HashSet<Page> = HashSet::new();

        let page_set: HashSet<Page> = pages.iter().copied().collect();

        let mut ordering_rules: HashMap<Page, HashSet<Page>> = ordering_rules
            .iter()
            .filter(|(earlier_page, _)| page_set.contains(earlier_page))
            .filter_map(|(&earlier_page, later_pages)| {
                let relevant_pages: HashSet<Page> =
                    page_set.intersection(later_pages).copied().collect();

                if relevant_pages.is_empty() {
                    None
                } else {
                    Some((earlier_page, relevant_pages))
                }
            })
            .collect();

        let mut incorrectly_ordered_update = false;

        for &page in &pages {
            if ordering_rules
                .get(&page)
                .is_some_and(|following_pages| !following_pages.is_disjoint(&seen_pages))
            {
                incorrectly_ordered_update = true;
                break;
            }

            seen_pages.insert(page);
        }

        if !incorrectly_ordered_update {
            continue;
        }

        let mut reverse_ordered_pages: Vec<Page> = Vec::with_capacity(pages.len());

        let mut unprocessed_pages = VecDeque::from(pages.clone());

        while let Some(page) = unprocessed_pages.pop_front() {
            if !ordering_rules.contains_key(&page) {
                reverse_ordered_pages.push(page);

                let mut rules_to_remove = Vec::new();

                for (&earlier_page, later_pages) in ordering_rules.iter_mut() {
                    later_pages.remove(&page);

                    if later_pages.is_empty() {
                        rules_to_remove.push(earlier_page);
                    }
                }

                for page in rules_to_remove {
                    ordering_rules.remove(&page);
                }
            } else {
                unprocessed_pages.push_back(page);
            }
        }

        let Page(middle_page) = reverse_ordered_pages[reverse_ordered_pages.len() / 2];
        middle_page_sum += middle_page;
    }

    Ok(middle_page_sum)
}
