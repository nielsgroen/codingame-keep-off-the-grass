use std::cmp::Ordering;
use super::super::{DistanceBoard, Owner};
use super::super::action::Action;
use super::Agent;
use super::super::board::Board;

/// Will try to have more `scrap + 10*units` on the board than the opponent
/// While sending all units to enemy territory
///
/// The philosophy is as follows:
/// Assuming the board stays a connected graph
/// (Which is a dubious assumption)
/// The person with the most scrap invested in Robots, wins
///
/// Don't mine too much than necessary to keep the board connected

pub struct SimpleEconomyAgent {}

impl Agent for SimpleEconomyAgent {
    fn generate_actions(&mut self, board: Board) -> Vec<Action> {

        let my_robot_count = board.robot_count(Owner::Me);
        let opponent_robot_count = board.robot_count(Owner::Opponent);

        let my_matter_robot_score = board.my_matter + 10 * my_robot_count;
        let opponent_matter_robot_score = board.opponent_matter + 10 * opponent_robot_count;

        // Make the Robots drive towards opponent
        let opponent_distance_board = DistanceBoard::from_owner(&board, Owner::Opponent);

        let my_robot_coords = board.fields.iter()
            .filter(|x| x.owner == Owner::Me && x.num_units > 0)
            .map(|x| (x.x, x.y, x.num_units));

        let mut result: Vec<Action> = Vec::new();
        for (x, y, num_units) in my_robot_coords {
            let direction_num = opponent_distance_board
                .towards(x, y, Ordering::Less)
                .iter()
                .position(|x| *x);
            // TODO: now they all take the first available direction: mix it up per bot

            let to_field = match direction_num {
                Some(0) => (x, y - 1),
                Some(1) => (x + 1, y),
                Some(2) => (x, y + 1),
                Some(3) => (x - 1, y),
                None => (x, y),
                _ => panic!("robot_coords should hold only 4 items"),
            };

            result.push(Action::Move {
                amount: num_units,
                from: (x, y),
                to: to_field,
            })
        }

        if my_matter_robot_score > opponent_matter_robot_score + 21 {
            result.extend(self.spawn_robots(&board, &opponent_distance_board))
        } else {
            result.extend(self.build_recyclers(&board))
        }

        result
    }
}

impl SimpleEconomyAgent {
    fn build_recyclers(&mut self, board: &Board) -> Vec<Action> {
        todo!()
    }

    fn spawn_robots(&mut self, board: &Board, opponent_distance_board: &DistanceBoard) -> Vec<Action> {
        todo!()
    }
}
