fn is_passport(ppt: &Vec<String>) -> bool {
    let mut fields = 0;
    let mut valids = 0;
    let mut has_cid = false;

    for f in ppt.iter() {
        let fs = f.split(':').collect::<Vec<_>>();

        fields += 1;
        // Short version
        if match fs[0] {
            "byr" => fs[1]
                .parse::<i32>()
                .ok()
                .map_or(false, |y| 1920 <= y && y <= 2002),
            "iyr" => fs[1]
                .parse::<i32>()
                .ok()
                .map_or(false, |y| 2010 <= y && y <= 2020),
            "eyr" => fs[1]
                .parse::<i32>()
                .ok()
                .map_or(false, |y| 2020 <= y && y <= 2030),
            "hgt" => {
                fs[1]
                    .strip_suffix("cm")
                    .and_then(|s| s.parse::<i32>().ok())
                    .map_or(false, |n| 150 <= n && n <= 193)
                    || fs[1]
                        .strip_suffix("in")
                        .and_then(|s| s.parse::<i32>().ok())
                        .map_or(false, |n| 59 <= n && n <= 76)
            }
            "hcl" => fs[1].starts_with("#") && fs[1][1..].chars().all(|c| c.is_ascii_hexdigit()),
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&fs[1]),
            "pid" => fs[1].len() == 9 && fs[1].chars().all(|c| c.is_digit(10)),
            "cid" => {
                has_cid = true;
                true
            }
            _ => false,
        } {
            valids += 1;
        }
    }

    if fields == 7 {
        return !has_cid && valids == 7;
    } else if fields == 8 {
        return valids == 8;
    } else {
        return false;
    }
}

fn main() {
    let mut passports = vec![vec![]];

    for line in include_str!("input")
        .trim()
        .split('\n')
        .map(|r| r.trim().to_string())
    {
        if line.is_empty() {
            passports.push(vec![]);
        } else {
            let l = passports.len() - 1;
            passports[l].extend(line.split(" ").map(|f| f.trim().to_string()));
        }
    }

    println!(
        "Valids: {}",
        passports.iter().map(is_passport).filter(|x| *x).count()
    );
}
