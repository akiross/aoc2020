use std::collections::HashMap;

fn main() {
    let mut jolts = include_str!("input")
        .trim()
        .split("\n")
        .map(|s| s.parse::<u32>().unwrap())
        .chain(0..1) // Insert starting element
        .collect::<Vec<_>>();
    jolts.sort();
    jolts.push(jolts.iter().last().unwrap() + 3); // Ending element
    let (_, n1, n3) = jolts[1..]
        .iter()
        .fold((jolts[0], 0, 0), |(cur, n1, n3), &j| match j - cur {
            1 => (j, n1 + 1, n3),
            2 => (j, n1, n3),
            3 => (j, n1, n3 + 1),
            _ => unreachable!(),
        });
    println!("num of 1-jolt * 3-jolt differences: {}", n1 * n3);

    let mut mem = HashMap::new();
    let count = countcon(&jolts[..], &mut mem);
    println!("Possible adaptor combinations: {}", count);
}

fn countcon(jolts: &[u32], mem: &mut HashMap<Vec<u32>, u64>) -> u64 {
    if jolts.len() <= 2 {
        1
    } else {
        let jv = jolts.to_vec();
        if let Some(count) = mem.get(&jv) {
            *count
        } else {
            let mut count = 0;
            for i in 1..jolts.len() {
                if jolts[i] - jolts[0] <= 3 {
                    count += 1 * countcon(&jolts[i..], mem);
                } else {
                    break;
                }
            }
            mem.insert(jv, count); // Memoize
            count
        }
    }
}
