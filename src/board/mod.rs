pub mod owner;
pub mod boardbuilder;
pub mod field;
pub mod distance_board;

use std::cmp::{max, min};
pub use owner::*;
pub use field::*;

pub struct Board {
    width: u32,
    height: u32,
    my_matter: u32,
    opponent_matter: u32,
    fields: Vec<Field>,
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




