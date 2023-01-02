use std::cmp::Ordering;
use std::iter::zip;
use super::super::board::recycler_range_board::RecyclerRangeBoard;
use super::super::board::yield_board::YieldBoard;
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

pub struct SimpleEconomyAgent {
    pub min_scrap_lead: i32,
    pub recycler_min_yield: u32,
    pub expected_mining_discount: f32,
}

impl Agent for SimpleEconomyAgent {
    fn generate_actions(&mut self, board: Board) -> Vec<Action> {
        let expected_mining = YieldBoard::without_recycling(&board); // yields, ignoring all recyclers
        let prospective_mining = YieldBoard::with_recycling(&board); // yields, accounting for current recyclers

        let my_robot_count = board.robot_count(Owner::Me);
        let opponent_robot_count = board.robot_count(Owner::Opponent);

        let my_yield = expected_yield(&board, &expected_mining, Owner::Me);
        let my_matter_robot_score = board.my_matter + 10 * my_robot_count + (my_yield as f32 * self.expected_mining_discount) as u32;
        let opponent_yield = expected_yield(&board, &expected_mining, Owner::Opponent);
        let opponent_matter_robot_score = board.opponent_matter + 10 * opponent_robot_count + (opponent_yield as f32 * self.expected_mining_discount) as u32;

        let opponent_distance_board = DistanceBoard::from_owner(&board, Owner::Opponent);

        let mut result: Vec<Action> = Vec::new();
        result.extend(self.move_robots(&board, &opponent_distance_board));

        let mut scrap_to_spend = board.my_matter;
        if (my_matter_robot_score as i32) < (opponent_matter_robot_score as i32 + self.min_scrap_lead) {
            let build_commands = self.build_recyclers(&board, &prospective_mining,  scrap_to_spend / 10);
            scrap_to_spend -= build_commands.len() as u32 * 10;
            result.extend(build_commands);
        }
        result.extend(self.spawn_robots(&board, &opponent_distance_board, scrap_to_spend / 10));

        result
    }
}

impl SimpleEconomyAgent {

    fn build_recyclers(&mut self, board: &Board, yield_board: &YieldBoard, amount: u32) -> Vec<Action> {
        let mut result: Vec<Action> = Vec::new();
        let recycler_range_board = RecyclerRangeBoard::from_board(board);

        let mut field_yield =  board.fields
            .iter()
            .filter(|x| x.owner == Owner::Me)
            .filter(|x| x.is_traversible())
            .filter(|x| x.num_units == 0)
            .map(|x| (x, yield_board.prospective_scrap[(x.x + x.y * board.width) as usize]))
            .collect::<Vec<_>>();

        field_yield.sort_by(|(_, a), (_, b)| (*a).cmp(b).reverse());

        // TODO: check if recycler is in range of another => then skip
        for (field, prospective_yield) in field_yield.into_iter().take((board.my_matter / 10) as usize) {
            if prospective_yield < self.recycler_min_yield {
                break;
            }

            if !recycler_range_board.get_field(field.x, field.y).unwrap() {
                eprintln!("x {} y {} prospective yield {}", field.x, field.y, prospective_yield);
                result.push(Action::Build(field.x, field.y))
            }
        }

        result
    }

    fn spawn_robots(&mut self, board: &Board, opponent_distance_board: &DistanceBoard, amount: u32) -> Vec<Action> {
        let mut result: Vec<Action> = Vec::new();

        let field_dist = board.fields
            .iter()
            .filter(|x| x.owner == Owner::Me)
            .filter(|x| x.is_traversible())
            .map(|x| (x, opponent_distance_board.get_field(x.x, x.y).unwrap()));

        let shortest_dist = *field_dist.clone()
            .map(|(_, dist)| dist)
            .min().unwrap();

        let mut field_dist = field_dist
            .filter(|(_, dist)| **dist == shortest_dist)
            .map(|(x, _)| x)
            .collect::<Vec<_>>();

        let num_placeable_fields = field_dist.len() as u32;
        let mut robot_count_to_spawn = amount;
        for (i, x) in field_dist.into_iter().enumerate() {
            let mut spawn_count = robot_count_to_spawn / (num_placeable_fields - i as u32);
            spawn_count += if robot_count_to_spawn % (num_placeable_fields - i as u32) > 0 { 1 } else { 0 };

            result.push(Action::Spawn(spawn_count, x.x, x.y));
            robot_count_to_spawn -= spawn_count;
        }

        result
    }

    fn move_robots(&self, board: &Board, opponent_distance_board: &DistanceBoard) -> Vec<Action> {
        let mut result = Vec::new();
        // keeps track of how many robots will be on a given field at the end of a turn
        let count_to_move_to_board: Vec<u32> = Vec::with_capacity(board.fields.len());

        let my_robot_coords = board.fields.iter()
            .filter(|x| x.owner == Owner::Me && x.num_units > 0)
            .map(|x| (x.x, x.y, x.num_units));

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

        result
    }
}

fn expected_yield(board: &Board, yield_board: &YieldBoard, owner: Owner) -> u32 {
    let fields = board.fields.iter();
    let yields = yield_board.prospective_scrap.iter();

    let result = zip(fields, yields)
        .filter(|(f, _)| f.has_recycler)
        .filter(|(f, _)| f.owner == owner)
        .map(|(_, y)| *y)
        .sum::<u32>();

    result
}
