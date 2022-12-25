use crate::Owner;
use super::super::action::Action;
use super::Agent;
use super::super::board::Board;

/// Will try to have more `scrap + 10*units` on the board than the opponent
/// While sending all units to enemy territory

struct SimpleEconomyAgent {}

impl Agent for SimpleEconomyAgent {
    fn generate_actions(board: Board) -> Vec<Action> {

        let my_robot_count = board.robot_count(Owner::Me);
        let opponent_robot_count = board.robot_count(Owner::Opponent);

        let my_matter_robot_score = board.my_matter + 10 * my_robot_count;
        let opponent_matter_robot_score = board.opponent_matter + 10 * opponent_robot_count;

        todo!()
    }
}