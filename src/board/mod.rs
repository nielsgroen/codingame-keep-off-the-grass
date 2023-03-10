pub mod owner;
pub mod boardbuilder;
pub mod field;
pub mod distance_board;
pub mod yield_board;
pub mod mine_duration_board;
pub mod recycler_range_board;

use std::cmp::{max, min};
pub use owner::*;
pub use field::*;

pub struct Board {
    pub width: u32,
    pub height: u32,
    pub my_matter: u32,
    pub opponent_matter: u32,
    pub fields: Vec<Field>,
}

impl Board {

    pub fn get_field(&self, width: u32, height: u32) -> Option<&Field> {
        self.fields.get((width + height * self.width) as usize)
    }

    pub fn get_field_mut(&mut self, width: u32, height: u32) -> Option<&mut Field> {
        self.fields.get_mut((width + height * self.width) as usize)
    }

    pub fn get_adjacent_fields(&self, width: u32, height: u32) -> [Option<&Field>; 4] { // NESW
        [
            self.get_field(width, height - 1),
            self.get_field(width + 1, height),
            self.get_field(width, height + 1),
            self.get_field(width - 1, height),
        ]
    }

    pub fn get_fields_in_range(&self, width: u32, height: u32) -> [Option<&Field>; 5] { // cur + NESW
        [
            self.get_field(width, height),
            self.get_field(width, height - 1),
            self.get_field(width + 1, height),
            self.get_field(width, height + 1),
            self.get_field(width - 1, height),
        ]
    }

    pub fn robot_count(&self, owner: Owner) -> u32 {
        self.fields
            .iter()
            .map(|x| {
                if x.owner == owner {
                    x.num_units
                } else { 0 }
            })
            .sum()
    }

    pub fn adjacent_robot_count(&self, x: u32, y: u32, owner: Owner) -> u32 {
        let adj = self.get_adjacent_fields(x, y);
        let adj = adj
            .into_iter()
            .flatten();

        adj
            .filter(|f| f.owner == owner)
            .map(|f| f.num_units)
            .sum()
    }

    pub fn process_harvest_cycle(&self) -> Self {
        let new_fields = self.fields.clone();

        let mut new_board = Board {
            fields: new_fields,
            ..*self
        };

        // reduce scrap per tile amount by min(#surrounding recyclers in range, scrap)
        for i in 0..new_board.width {
            for j in 0..new_board.height {
                let mut fields_in_range = new_board.get_fields_in_range(i, j);

                let mut num_recyclers_in_range = fields_in_range
                    .into_iter()
                    .flatten()
                    .filter(|x| {x.has_recycler})
                    .count();

                let mut current_field = new_board.get_field_mut(i, j).unwrap();
                // Prevents unsigned int underflow
                current_field.scrap_amount -= min(current_field.scrap_amount, num_recyclers_in_range as u32);
            }
        }

        // Clean up recyclers
        new_board.fields = new_board.fields
            .into_iter()
            .map(|mut x| {
                x.has_recycler = x.has_recycler && x.scrap_amount != 0;
                x
            })
            .collect();

        // Update whether fields are in recycler range
        for i in 0..new_board.width {
            for j in 0..new_board.height {
                let mut fields_in_range = new_board.get_fields_in_range(i, j);
                let in_recycler_range = fields_in_range
                        .into_iter()
                        .flatten()
                        .any(|x| x.has_recycler);

                new_board.get_field_mut(i, j).unwrap().in_recycler_range = in_recycler_range;
            }
        }

        new_board
    }

    /// Returns what the board would like if all current recyclers were mined out
    pub fn mined_out(&self) -> Self {
        todo!()

        // Note: recyclers will only mine for as long as they have scrap directly underneath them
        // Tile can be in mining range of multiple recyclers
    }
}

pub fn adjacent_in_range(x: u32, y: u32, width: u32, height: u32) -> [Option<(u32, u32)>; 4] {
    // NESW
    [
        if 0 <= x && x < width && 0 <= y-1 && y-1 < height { Some((x, y-1)) } else { None },
        if 0 <= x+1 && x+1 < width && 0 <= y && y < height { Some((x+1, y)) } else { None },
        if 0 <= x && x < width && 0 <= y+1 && y+1 < height { Some((x, y+1)) } else { None },
        if 0 <= x-1 && x-1 < width && 0 <= y && y < height { Some((x-1, y)) } else { None },
    ]
}


