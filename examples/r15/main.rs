use std::collections::HashMap;

type Number = usize;
type Turn = usize;

fn main() {
    let input = &[16, 11, 15, 0, 1, 7];
    //let input = &[0, 3, 6];

    let mut memory: HashMap<Number, (Turn, Option<Turn>)> = HashMap::new();

    let mut turn = 1;
    let mut number = 0;
    while turn <= input.len() {
        number = input[turn - 1];
        //println!("Turn {} Number {}", turn, number);
        memory.insert(number, (turn, None));
        turn += 1;
    }

    while turn <= 30000000 {
        match memory.get(&number) {
            // This is the memory of spoken numbers: can't recall a number never spoken
            None => unreachable!(),
            // First time the number was spoken
            Some((_, None)) => number = 0,
            // Number was spoken before
            Some((turn_last_spoken, Some(prev_turn_spoken))) => {
                number = turn_last_spoken - prev_turn_spoken
            }
        }

        if turn == 30000000 || turn == 2020 {
            println!("Turn {} Number {}", turn, number);
        }

        // Update memory according to last spoken number
        if let Some(&(turn_last_spoken, _)) = memory.get(&number) {
            memory.insert(number, (turn, Some(turn_last_spoken)));
        } else {
            memory.insert(number, (turn, None));
        }

        turn += 1;
    }

    println!("{:?} {}", memory.keys().max(), memory.len());
}
