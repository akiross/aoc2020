fn count_nbors(seats: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut occ = 0;

    let h = seats.len() as i32;
    let w = seats[i].len() as i32;

    for &y in &[-1, 0, 1] {
        for &x in &[-1, 0, 1] {
            // Skip seat itself
            if x == 0 && y == 0 {
                continue
            }
            let i = i as i32 + y;
            let j = j as i32 + x;
            // Ignore out of bounds
            if i < 0 || j < 0 || i >= h || j >= w {
                continue
            }
            match seats[i as usize][j as usize] {
                '#' => { occ += 1; },
                _ => {}
            }
            
        }
    }
    return occ;
}

fn count_line(seats: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut occ = 0;

    let h = seats.len() as i32;
    let w = seats[i].len() as i32;

    for &y in &[-1, 0, 1] {
        for &x in &[-1, 0, 1] {
            if x == 0 && y == 0 {
                continue
            }
            let mut i = i as i32 + y;
            let mut j = j as i32 + x;
            while i >= 0 && j >= 0 && i < h && j < w {
                match seats[i as usize][j as usize] {
                    '#' => {
                        occ += 1;
                        break
                    }
                    'L' => { break }
                    _ => { }
                }
                i += y;
                j += x;
            }
        }
    }
    occ
}

fn step<F>(seats: &Vec<Vec<char>>, thresh: usize, counter: F) -> Vec<Vec<char>> 
where F: Fn(&Vec<Vec<char>>, usize, usize) -> usize {
    let mut nseats = seats.clone();
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            let seat = seats[i][j];
            match seat {
                'L' => {
                    if counter(seats, i, j) == 0 {
                        nseats[i][j] = '#';
                    }
                }
                '#' => {
                    if counter(seats, i, j) >= thresh {
                        nseats[i][j] = 'L';
                    }
                }
                _ => {}
            }
        }
    }
    nseats
}

fn main() {
    let grid = include_str!("input")
        .trim()
        .split("\n")
        .map(|r| {
            r.chars()
                .map(|c| if c == 'L' { 'L' } else { '.' })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    let mut seats = grid.clone();
    let mut steps = 0;
    loop {
        steps += 1;
        let s = step(&seats, 4, count_nbors);
        if s == seats {
            let occup: usize = s.iter().map(|r| r.iter().filter(|&&s| s == '#' ).count() ).sum();
            println!("Stable after {} steps, {} occupied", steps, occup);
            break;
        } else {
            seats = s;
        }
    }

    let mut seats = grid.clone();
    let mut steps = 0;
    loop {
        steps += 1;
        let s = step(&seats, 5, count_line);
        if s == seats {
            let occup: usize = s.iter().map(|r| r.iter().filter(|&&s| s == '#' ).count() ).sum();
            println!("Stable after {} steps, {} occupied", steps, occup);
            break;
        } else {
            seats = s;
        }
    }
}
