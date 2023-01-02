use std::cmp::min;
use super::field::Field;
use super::Board;

/// Provides the yields for mining on a position, assuming no other recyclers are nearby
#[derive(Clone, Debug)]
pub struct YieldBoard {
    pub width: u32,
    pub height: u32,
    pub prospective_scrap: Vec<u32>,
}

impl YieldBoard {

    pub fn with_recycling(board: &Board) -> Self {
        let prospective_scrap = board.fields
            .iter()
            // .map(|x| (x, board.get_adjacent_fields(x.x, x.y).into_iter().flatten()))
            .map(|x| {
                let adj = board.get_adjacent_fields(x.x, x.y);
                let total = adj.into_iter().flatten()
                    .map(|a| min(x.scrap_amount, scrap_if_not_in_harvester_range(a)))
                    .sum::<u32>();
                total + scrap_if_not_in_harvester_range(x)
            })
            .collect();

        Self {
            width: board.width,
            height: board.height,
            prospective_scrap
        }
    }

    pub fn without_recycling(board: &Board) -> Self {
        let prospective_scrap = board.fields
            .iter()
            // .map(|x| (x, board.get_adjacent_fields(x.x, x.y).into_iter().flatten()))
            .map(|x| {
                let adj = board.get_adjacent_fields(x.x, x.y);
                let total = adj.into_iter().flatten()
                    .map(|a| min(x.scrap_amount, a.scrap_amount))
                    .sum::<u32>();
                total + x.scrap_amount
            })
            .collect();

        Self {
            width: board.width,
            height: board.height,
            prospective_scrap
        }
    }

    pub fn get_field(&self, x: u32, y: u32) -> Option<&u32> {
        self.prospective_scrap.get((x + y * self.width) as usize)
    }


}

fn scrap_if_not_in_harvester_range(field: &Field) -> u32 {
    if field.in_recycler_range { 0 } else { field.scrap_amount }
}
