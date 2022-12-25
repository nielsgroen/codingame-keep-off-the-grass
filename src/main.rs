use std::io;
use std::time::Instant;



macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

mod board;
mod agent;
mod action;

use board::Field;
use board::boardbuilder::BoardBuilder;
use board::boardbuilder::SizeKnown;
use board::distance_board::DistanceBoard;
use board::Owner;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32) as u32;
    let height = parse_input!(inputs[1], i32) as u32;


    // game loop
    loop {
        let start = Instant::now();
        let builder = BoardBuilder::new(width, height).fields_from_stdin();
        let board = builder.build();

        let duration = start.elapsed();
        println!("WAIT");

        let distance_board = DistanceBoard::from_owner(&board, Owner::Me);



        eprintln!("Time elapsed in micros: {}", duration.as_micros());
    }
}

