fn main() {
    let rows = include_str!("input")
        .split('\n')
        .filter(|r| !r.is_empty())
        .map(|r| r.parse::<i32>().expect("Not an i32"))
        .collect::<Vec<_>>();

    'outer: for x in rows.iter() {
        for y in rows.iter() {
            for z in rows.iter() {
                if x + y + z == 2020 {
                    println!("{} {} {} = {}", x, y, z, x * y * z);
                    break 'outer;
                }
            }
        }
    }
}
