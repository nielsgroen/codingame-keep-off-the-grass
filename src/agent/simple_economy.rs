use std::cmp::Ordering;
use std::iter::zip;
use super::super::board::adjacent_in_range;
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
    pub recycler_min_score: i32,
    pub expected_mining_discount: f32,
    pub distance_move_weighting: u32, // how important it is to move closer vs. spreading out
    pub distance_mine_weighting: i32, // how much recycler distance weighs vs. yield
    pub recycler_robot_adjacency_weight: u32,
    pub movement_own_score: u32,
    pub movement_neutral_score: u32,
    pub movement_opponent_score: u32,
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
            let build_commands = self.build_recyclers(&board, &prospective_mining, &opponent_distance_board,  scrap_to_spend / 10);
            scrap_to_spend -= build_commands.len() as u32 * 10;
            result.extend(build_commands);
        }
        result.extend(self.spawn_robots(&board, &opponent_distance_board, scrap_to_spend / 10));

        result
    }
}

impl SimpleEconomyAgent {

    fn build_recyclers(&mut self, board: &Board, yield_board: &YieldBoard, opponent_distance_board: &DistanceBoard, amount: u32) -> Vec<Action> {
        let mut result: Vec<Action> = Vec::new();
        let recycler_range_board = RecyclerRangeBoard::from_board(board);

        let mut field_score =  board.fields
            .iter()
            .filter(|x| x.owner == Owner::Me)
            .filter(|x| x.is_traversible())
            .filter(|x| x.num_units == 0)
            .filter(|x| !opponent_distance_board.get_field(x.x, x.y).unwrap().is_unreachable())
            .map(|x| (x, yield_board.prospective_scrap[(x.x + x.y * board.width) as usize], opponent_distance_board.distances[(x.x + x.y * board.width) as usize]))
            .map(|(a, y, dist)| {
                let enemy_robot_score = board.adjacent_robot_count(a.x, a.y, Owner::Opponent) * self.recycler_robot_adjacency_weight;
                let score = enemy_robot_score as i32 + y as i32 - self.distance_mine_weighting * dist.distance_or_panic() as i32;
                (a, y, score)
            })
            .collect::<Vec<_>>();

        field_score.sort_by(|(_, _, a), (_, _, b)| (*a).cmp(b).reverse());

        // TODO: check if recycler is in range of another => then skip
        for (field, _, score) in field_score.into_iter().take((board.my_matter / 10) as usize) {
            if score < self.recycler_min_score {
                break;
            }

            if !recycler_range_board.get_field(field.x, field.y).unwrap() {
                eprintln!("x {} y {} prospective score {}", field.x, field.y, score);
                result.push(Action::Build(field.x, field.y))
            }
        }

        result
    }

    fn spawn_robots(&mut self, board: &Board, opponent_distance_board: &DistanceBoard, amount: u32) -> Vec<Action> {
        let mut result: Vec<Action> = Vec::new();

        let neutral_distance_board = DistanceBoard::from_owner(board, Owner::Neutral);
        let mut distance_board_to_use =
            if zip(opponent_distance_board.distances.iter(), board.fields.iter())
                .all(|(dist, f)| dist.is_unreachable() || f.owner != Owner::Me)
            {
                &neutral_distance_board
            } else {
                opponent_distance_board
            };

        let field_dist = board.fields
            .iter()
            .filter(|x| x.owner == Owner::Me)
            .filter(|x| x.is_traversible())
            .map(|x| (x, distance_board_to_use.get_field(x.x, x.y).unwrap()))
            .filter(|(_, dist)| !dist.is_unreachable())
            .filter(|(f, _)| { // consider only fields that are adjacent to unowned tiles
                let adj = board.get_adjacent_fields(f.x, f.y);
                adj
                    .into_iter()
                    .flatten()
                    .any(|field| field.owner != Owner::Me && field.is_traversible())
            });

        let mut field_score = field_dist
            .map(|(f, dist)| {
                let score = f.num_units + dist.distance_or_panic();
                (f, score)
            })
            .collect::<Vec<_>>();

        field_score.sort_by(|(_, a), (_, b)| a.cmp(b));

        if field_score.len() > 0 {
            let mut amount_placed = vec![0_u32; field_score.len()];
            let mut aspiration_score = field_score[0].1;
            let mut amount_to_go = amount;

            let mut field_score_cycled = field_score
                .iter()
                .enumerate()
                .cycle();

            while amount_to_go > 0 {
                let (index, (f, score)) = field_score_cycled.next().unwrap();
                if index == 0 {
                    aspiration_score += 1;
                }

                if amount_placed[index] + score < aspiration_score {
                    amount_placed[index] += 1;
                    amount_to_go -= 1;
                }
            }

            result.extend(
                zip(amount_placed.into_iter(), field_score)
                    .map(|(a, (f, _))| Action::Spawn(a, f.x, f.y))
            );
        }

        result

        // todo!();
        // let shortest_dist = *field_dist.clone()
        //     .map(|(_, dist)| dist)
        //     .min().unwrap();
        //
        // let mut field_dist = field_dist
        //     .filter(|(_, dist)| **dist == shortest_dist)
        //     .map(|(x, _)| x)
        //     .collect::<Vec<_>>();
        //
        // let num_placeable_fields = field_dist.len() as u32;
        // let mut robot_count_to_spawn = amount;
        // for (i, x) in field_dist.into_iter().enumerate() {
        //     let mut spawn_count = robot_count_to_spawn / (num_placeable_fields - i as u32);
        //     spawn_count += if robot_count_to_spawn % (num_placeable_fields - i as u32) > 0 { 1 } else { 0 };
        //
        //     if spawn_count > 0 {
        //         result.push(Action::Spawn(spawn_count, x.x, x.y));
        //     }
        //     robot_count_to_spawn -= spawn_count;
        // }
        //
        // result
    }

    fn move_robots(&self, board: &Board, opponent_distance_board: &DistanceBoard) -> Vec<Action> {
        let mut result = Vec::new();
        // keeps track of how many robots will be on a given field at the end of a turn
        let arrival_count_board: Vec<u32> = vec![0; board.fields.len()];

        let mut my_robot_coords = zip(board.fields.iter(), opponent_distance_board.distances.iter())
            .filter(|(x, _)| x.owner == Owner::Me && x.num_units > 0)
            .map(|(a, b)| (a.x, a.y, a.num_units, *b))
            .collect::<Vec<_>>();

        my_robot_coords.sort_by(|(_, _, _, a), (_, _, _, b)| a.cmp(b));

        let neutral_distance_board = DistanceBoard::from_owner(board, Owner::Neutral);
        let mut owner_score = board.fields
            .iter()
            .map(|f| match f.owner {
                Owner::Me => 5,
                Owner::Neutral => 1,
                Owner::Opponent => 0,
            })
            .collect::<Vec<_>>();

        for (x, y, num_units, _) in my_robot_coords.into_iter() {
            let adjacent_locations = adjacent_in_range(x, y, board.width, board.height);
            let adjacent_locations = adjacent_locations
                .into_iter()
                .flatten()
                .filter(|(x, y)| {
                    let field = board.get_field(*x, *y).unwrap();
                    !(field.scrap_amount <= 1 && field.in_recycler_range)
                });

            let adjacent_distances = adjacent_locations
                .clone()
                .map(|(x, y)| *opponent_distance_board.get_field(x, y).unwrap())
                .collect::<Vec<_>>();

            let adjacent_arrival_count = adjacent_locations
                .map(|(x, y)| (x, y, arrival_count_board[(x + board.width * y) as usize]));

            // (x, y, score)
            let current_adjacent_score = zip(adjacent_distances, adjacent_arrival_count)
                .filter(|(a, _)| !a.is_unreachable())
                .map(|(a, b)| {
                    // let owner_score = match board.get_field(b.0, b.1).unwrap().owner {
                    //     Owner::Me => 5,
                    //     Owner::Neutral => 2,
                    //     Owner::Opponent => 0,
                    // };
                    let score = a.distance_or_panic() * self.distance_move_weighting + b.2;
                    (b.0, b.1, score)
                })
                .collect::<Vec<_>>();

            if current_adjacent_score.len() > 0 {
                // enemy reachable
                let mut current_aspiration_score = current_adjacent_score
                    .iter()
                    .map(|(_, _, x)| *x)
                    .min()
                    .unwrap_or(0u32);

                let mut num_units_left = num_units;
                let mut move_amounts = vec![0_u32; current_adjacent_score.len()];
                let mut location_scores = current_adjacent_score
                    .clone()
                    .into_iter()
                    .enumerate()
                    .cycle();
                let mut locations = current_adjacent_score
                    .iter()
                    .map(|(x, y, _)| (x, y));

                while num_units_left > 0 {
                    let (index, (x, y, score)) = location_scores.next().unwrap();
                    if index == 0 {
                        current_aspiration_score += 1;
                    }

                    if move_amounts[index] + score + owner_score[(x + y * board.width) as usize] < current_aspiration_score {
                        move_amounts[index] += 1;
                        owner_score[(x + y * board.width) as usize] = self.movement_own_score;
                        num_units_left -= 1;
                    }
                }

                result.extend(
                    zip(locations, move_amounts.into_iter())
                        .map(|((to_x, to_y), amount)| Action::Move {
                            amount,
                            from: (x, y),
                            to: (*to_x, *to_y)
                        })
                )
            } else {
                // enemy unreachable
                let direction_num = neutral_distance_board
                    .towards(x, y, Ordering::Less)
                    .iter()
                    .position(|x| *x);

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


            // let direction_num = opponent_distance_board
            //     .towards(x, y, Ordering::Less)
            //     .iter()
            //     .position(|x| *x);
            // // TODO: now they all take the first available direction: mix it up per bot
            //
            // let to_field = match direction_num {
            //     Some(0) => (x, y - 1),
            //     Some(1) => (x + 1, y),
            //     Some(2) => (x, y + 1),
            //     Some(3) => (x - 1, y),
            //     None => (x, y),
            //     _ => panic!("robot_coords should hold only 4 items"),
            // };
            //
            // result.push(Action::Move {
            //     amount: num_units,
            //     from: (x, y),
            //     to: to_field,
            // })
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
