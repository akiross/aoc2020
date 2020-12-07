fn get_id(s: &str) -> usize {
    usize::from_str_radix(&s[..7], 2).unwrap() * 8 + usize::from_str_radix(&s[7..], 2).unwrap()
}

fn main() {
    let mut rows = include_str!("input")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|r| {
            r.replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1")
        })
        .collect::<Vec<_>>();
    rows.sort();

    let first = rows.first().unwrap();
    println!("First {} {}", first, get_id(first));

    let last = rows.last().unwrap();
    println!("Last {} {}", last, get_id(last));

    rows.iter().enumerate().find(|(i, r)| {
        if get_id(r) != i + get_id(first) {
            println!("Seat {}", i);
            true
        } else {
            false
        }
    });
}
