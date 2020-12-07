#![feature(iterator_fold_self)]
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let groups = include_str!("input")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .split(|l| l.is_empty())
        .map(|g| g.to_vec())
        .collect::<Vec<Vec<String>>>();

    let sets = groups
        .iter()
        .map(|g| {
            g.iter().fold(HashSet::new(), |h, item| {
                h.union(&HashSet::from_iter(item.chars()))
                    .copied()
                    .collect()
            })
        })
        .collect::<Vec<HashSet<char>>>();

    println!(
        "Any answered count: {}",
        sets.iter().map(|h| h.len()).sum::<usize>()
    );

    let sets = groups
        .iter()
        .map(|g| {
            g.iter()
                .map(|r| HashSet::from_iter(r.chars()))
                .fold_first(|h, set| h.intersection(&set).copied().collect())
                .unwrap()
        })
        .collect::<Vec<HashSet<char>>>();

    println!(
        "All answered count: {}",
        sets.iter().map(|h| h.len()).sum::<usize>()
    );
}
