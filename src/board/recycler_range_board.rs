use super::Board;
use super::{Field, Owner};
use super::adjacent_in_range;

pub struct RecyclerRangeBoard {
    pub width: u32,
    pub height: u32,
    pub in_range: Vec<bool>,
}

impl RecyclerRangeBoard {

    pub fn from_board(board: &Board) -> Self {
        let in_range = board.fields
            .iter()
            .map(|x| x.in_recycler_range)
            .collect();

        Self {
            width: board.width,
            height: board.height,
            in_range,
        }
    }

    pub fn get_field(&self, width: u32, height: u32) -> Option<&bool> {
        self.in_range.get((width + height * self.width) as usize)
    }

    pub fn get_field_mut(&mut self, width: u32, height: u32) -> Option<&mut bool> {
        self.in_range.get_mut((width + height * self.width) as usize)
    }

    pub fn get_adjacent_fields(&self, width: u32, height: u32) -> [Option<&bool>; 4] { // NESW
        [
            self.get_field(width, height - 1),
            self.get_field(width + 1, height),
            self.get_field(width, height + 1),
            self.get_field(width - 1, height),
        ]
    }

    pub fn get_fields_in_range(&self, width: u32, height: u32) -> [Option<&bool>; 5] { // cur + NESW
        [
            self.get_field(width, height),
            self.get_field(width, height - 1),
            self.get_field(width + 1, height),
            self.get_field(width, height + 1),
            self.get_field(width - 1, height),
        ]
    }

    pub fn process_recycler_placement(&mut self, x: u32, y: u32) {
        self.in_range[(x + y * self.width) as usize] = true;

        for (x, y) in adjacent_in_range(x, y, self.width, self.height).iter().flatten() {
            self.in_range[(x + y * self.width) as usize] = true;
        }
    }

    pub fn process_recycler_placement_field(&mut self, field: &Field) {
        self.process_recycler_placement(field.x, field.y);
    }
}


