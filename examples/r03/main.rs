fn count_trees(g: &Vec<String>, xoff: usize, yoff: usize) -> usize {
    let w = g[0].len();
    let h = g.len();

    println!("Working on {}x{} grid", h, w);

    let mut trees = 0;
    let mut x = 0;
    for y in (0..h).step_by(yoff) {
        if let Some(c) = g[y].chars().nth(x) {
            println!("{}x{}={}", y, x, c);
            if c == '#' {
                trees += 1;
            }
        }
        x = (x + xoff) % w;
    }
    println!("");
    trees
}

fn main() {
    let g = include_str!("input")
        .split('\n')
        .filter(|r| !r.trim().is_empty())
        .map(|r| r.trim().to_string())
        .collect::<Vec<_>>();

    let n11 = count_trees(&g, 1, 1);
    let n31 = count_trees(&g, 3, 1);
    let n51 = count_trees(&g, 5, 1);
    let n71 = count_trees(&g, 7, 1);
    let n12 = count_trees(&g, 1, 2);

    println!("Met {} trees", n11);
    println!("Met {} trees", n31);
    println!("Met {} trees", n51);
    println!("Met {} trees", n71);
    println!("Met {} trees", n12);
    println!("Met {} trees", n11 * n31 * n51 * n71 * n12);
}
