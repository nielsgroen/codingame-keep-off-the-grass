use super::super::action::Action;
use super::Agent;
use super::super::board::Board;

/// Will try to have more `scrap + 10*units` on the board than the opponent
/// While sending all units to enemy territory

struct SimpleEconomyAgent {}

impl Agent for SimpleEconomyAgent {
    fn generate_actions(board: Board) -> Vec<Action> {
        todo!()
    }
}