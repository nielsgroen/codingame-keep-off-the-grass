/// Lists the action types

use std::fmt::{Display, Formatter};
use std::io::Take;
use std::time::{SystemTime, UNIX_EPOCH};

const TAUNTS: [&'static str; 4] = [
    "Be prepared to get scrapped!",
    "Let's see if Santa made your Robots run on coal!",
    "Ah! A good game to you... Unless your name is Jaap!",
    "Better to give in. You wouldn't want to end up on my naughty list, would you?",
];


pub enum Action {
    Move {
        amount: u32,
        from: (u32, u32),
        to: (u32, u32),
    },
    Build(u32, u32),
    Spawn(u32, u32),
    Message(String),
    Wait,
}

impl Action {
    pub fn generate_taunt() -> Self {
        // Uses the current nanos from 1970 as rng
        Self::Message(TAUNTS[(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as usize % TAUNTS.len()) as usize].to_string())
    }

    pub fn log_actions(actions: impl IntoIterator<Item=Action>) {
        for action in actions {
            print!("{};", action);
            println!();
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Move {amount, from, to} => {
                write!(f, "MOVE {} {} {} {} {}", amount, from.0, from.1, to.0, to.1)
            },
            Self::Build(x, y) => {
                write!(f, "BUILD {} {}", x, y)
            },
            Self::Spawn(x, y) => {
                write!(f, "SPAWN {} {}", x, y)
            },
            Self::Message(msg) => {
                write!(f, "MESSAGE {}", msg)
            },
            Self::Wait => {
                write!(f, "WAIT")
            }
        }
    }
}
