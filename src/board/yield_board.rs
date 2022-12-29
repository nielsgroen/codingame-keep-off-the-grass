use std::cmp::min;
use super::Board;

/// Provides the yields for mining on a position, assuming no other recyclers are nearby
#[derive(Clone, Debug)]
pub struct YieldBoard {
    pub width: u32,
    pub height: u32,
    pub prospective_scrap: Vec<u32>,
}

impl YieldBoard {

    pub fn new(board: &Board) -> Self {
        let prospective_scrap = board.fields
            .iter()
            .map(|x| (x, board.get_adjacent_fields(x.x, x.y).into_iter().flatten()))
            .map(|(x, adj)| {
                x.scrap_amount + adj
                    .map(|a| min(x.scrap_amount, a.scrap_amount))
                    .sum::<u32>()
            })
            .collect();

        Self {
            width: board.width,
            height: board.height,
            prospective_scrap
        }
    }

}


