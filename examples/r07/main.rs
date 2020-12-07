use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() {
    let rows = HashMap::<String, Vec<(usize, String)>>::from_iter(
        include_str!("input").trim().split("\n").map(|row| {
            let r = row.split("contain").collect::<Vec<_>>();
            (
                r[0].split("bags").nth(0).unwrap().trim().to_owned(),
                r[1].replace("bags", "")
                    .replace("bag", "")
                    .replace(".", "")
                    .split(",")
                    .filter_map(|s| {
                        let n_c = s.trim().splitn(2, " ").collect::<Vec<_>>();
                        if let Ok(count) = n_c[0].parse() {
                            Some((count, n_c[1].to_owned()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .to_owned(),
            )
        }),
    );

    println!(
        "Bags containing at least one shiny gold: {}",
        search(&rows, "shiny gold")
    );
    println!(
        "One shiny gold will cotain {} bags",
        traverse(&rows, "shiny gold") - 1
    );
}

fn search(rows: &HashMap<String, Vec<(usize, String)>>, start: &str) -> usize {
    let mut seq = vec![start.to_owned()];
    let mut i = 0;
    let mut bags = HashSet::new();
    while i < seq.len() {
        //print!("Searching for bags with {}... ", search[i]);
        let mut found = rows
            .iter()
            .filter(|(_k, v)| v.iter().any(|(_n, c)| c == &seq[i]))
            .collect::<Vec<_>>();
        //println!("Found {} bags", found.len());
        found.drain(..).for_each(|(k, _v)| {
            bags.insert(k.to_owned());
            seq.push(k.to_owned());
        });
        i += 1;
    }
    bags.len()
}

fn traverse(rows: &HashMap<String, Vec<(usize, String)>>, root: &str) -> usize {
    let content = rows.get(root).unwrap();
    let mut n = 1;
    for (count, color) in content {
        //println!("{} contains {} {}", root, count, color);
        n += count * traverse(rows, color);
    }
    n
}
