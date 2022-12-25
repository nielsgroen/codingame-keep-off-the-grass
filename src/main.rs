use std::io;



macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

mod board;

use board::Field;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_matter = parse_input!(inputs[0], i32);
        let opp_matter = parse_input!(inputs[1], i32);
        for i in 0..height as usize {
            for j in 0..width as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.split(" ").collect::<Vec<_>>();
                let scrap_amount = parse_input!(inputs[0], i32);
                let owner = parse_input!(inputs[1], i32); // 1 = me, 0 = foe, -1 = neutral
                let units = parse_input!(inputs[2], i32);
                let recycler = parse_input!(inputs[3], i32);
                let can_build = parse_input!(inputs[4], i32);
                let can_spawn = parse_input!(inputs[5], i32);
                let in_range_of_recycler = parse_input!(inputs[6], i32);
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("WAIT");
    }
}

