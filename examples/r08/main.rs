use std::collections::HashSet;

fn run_looper(program: &Vec<(String, i32)>) -> (Vec<(i32, i32)>, bool) {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let mut pcacc: Vec<(i32, i32)> = vec![];
    let mut visited = HashSet::new();
    loop {
        // Save current state
        pcacc.push((pc, acc));

        // Determine if in an infinite loop
        if visited.contains(&pc) {
            println!("Stopping at pc={} acc={}", pc, acc);
            break (pcacc, false);
        } else {
            visited.insert(pc);
        }

        // Check if program terminates
        if pc < 0 {
            panic!("No negative instruction!");
        } else if pc == program.len() as i32 {
            println!("Program terminated with acc={}", acc);
            break (pcacc, true);
        }

        // Run instruction
        let (cmd, val) = &program[pc as usize];
        match cmd.as_str() {
            "jmp" => {
                pc += val;
            }
            "acc" => {
                acc += val;
                pc += 1;
            }
            _ => {
                pc += 1;
            }
        }
    }
}

fn patch(program: &Vec<(String, i32)>, i: usize) -> Vec<(String, i32)> {
    match program[i].0.as_str() {
        "jmp" => {
            let mut p2 = program.clone();
            p2[i].0 = "nop".to_owned();
            p2
        }
        "nop" => {
            let mut p2 = program.clone();
            p2[i].0 = "jmp".to_owned();
            p2
        }
        _ => program.clone(),
    }
}

fn main() {
    let program = include_str!("input")
        .trim()
        .split("\n")
        .map(|s| {
            let v = s.trim().split(" ").collect::<Vec<_>>();
            (v[0].to_owned(), v[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<(String, i32)>>();

    for i in 0..program.len() {
        let p = patch(&program, i);
        let (hist, term) = run_looper(&p);
        if term {
            println!("Found correction {:?}", hist);
            break;
        }
    }
}
