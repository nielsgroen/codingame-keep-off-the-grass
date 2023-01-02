use std::iter::zip;
use super::Board;

/// Contains how many turns until each field is mined empty
#[derive(Clone, Debug)]
pub struct MineDurationBoard {
    width: u32,
    height: u32,
    mine_durations: Vec<MineDuration>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum MineDuration {
    Duration(u32),
    Unending,
}

impl MineDurationBoard {

    pub fn new(board: &Board) -> Self {
        todo!();
        // TODO: maybe deprecate => recyclers can be depleted faster than
        // the scrap amount of the field they're on

        let mut mine_durations = vec![MineDuration::Unending; board.fields.len()];

        for (mut dur, field) in zip(mine_durations, board.fields.iter()) {
            let recycler_scrap_counts = board.get_adjacent_fields(field.x, field.y)
                .into_iter()
                .flatten()
                .filter(|x| x.has_recycler)
                .map(|x| x.scrap_amount)
                .collect::<Vec<_>>();



            let mut scrap_left = field.scrap_amount;
            let mut turns_mined = 0_u32;

            while scrap_left > 0 {

            }

            // if recycler_fields.len() > 0 {
            //     let mut scrap_left = field.scrap_amount;
            //     let mut turns_mined = 0_u32;
            //
            //
            // }

        }


        // TODO: remove
        let mine_durations = Vec::new();

        Self {
            width: board.width,
            height: board.height,
            mine_durations,
        }
    }

}

