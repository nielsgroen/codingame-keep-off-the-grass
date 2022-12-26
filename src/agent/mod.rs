use super::board::Board;
use super::action::Action;

/// Every submodule is an agent implementation

pub mod simple_economy;


pub trait Agent {
    fn generate_actions(&mut self, board: Board) -> Vec<Action>;
}
