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
use agent::Agent;
use agent::simple_economy::SimpleEconomyAgent;
use action::Action;

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

    let mut agent = SimpleEconomyAgent {
        min_scrap_lead: 12,
        recycler_min_score: 15,
        expected_mining_discount: 0.8,
        distance_move_weighting: 3,
        distance_mine_weighting: 2,
        recycler_robot_adjacency_weight: 3,
        movement_own_score: 4,
        movement_neutral_score: 2,
        movement_opponent_score: 0,
    };

    // game loop
    loop {
        let start = Instant::now();
        let builder = BoardBuilder::new(width, height).fields_from_stdin();
        let board = builder.build();

        let actions = agent.generate_actions(board);
        Action::log_turn(actions);

        let duration = start.elapsed();
        eprintln!("Time elapsed in micros: {}", duration.as_micros());
    }
}

