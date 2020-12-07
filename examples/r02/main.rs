fn main() {
    let rows = include_str!("input")
        .split('\n')
        .filter(|r| !r.is_empty())
        .map(|r| r.trim())
        .collect::<Vec<_>>();

    let mut valid = 0;
    let mut counter = 0;
    let mut lines = 0;
    for row in rows.iter() {
        println!("Input row: {}", row);
        // min-max c: somestring
        let n_c_s = row.split(' ').collect::<Vec<_>>();
        let n = n_c_s[0].split('-').collect::<Vec<_>>();
        let (min, max): (usize, usize) = (n[0].parse().unwrap(), n[1].parse().unwrap()); // (min, max)
        let c = n_c_s[1].chars().nth(0).unwrap(); // 'c'
        let s = n_c_s[2];
        let count = s.chars().filter(|&x| x == c).count();
        println!(
            "String {} has char {} for {} times (min {} max {})",
            s, c, count, min, max
        );
        if count < min || count > max {
            println!("Invalid!");
        } else {
            println!("Valid!");
            counter += 1;
        }
        lines += 1;

        let c1 = s.chars().nth(min - 1).unwrap();
        let c2 = s.chars().nth(max - 1).unwrap();
        if (c1 == c) ^ (c2 == c) {
            valid += 1;
        }
    }
    println!("{} valid passwords out of {} lines", counter, lines);
    println!("{} valid passwords for right policies", valid);
}
