use super::Owner;


#[derive(Copy, Clone, Debug)]
pub struct Field {
    pub x: u32,
    pub y: u32,
    pub scrap_amount: u32,
    pub owner: Owner,
    pub num_units: u32,
    pub has_recycler: bool,
    pub can_build: bool,
    pub can_spawn: bool,
    pub in_recycler_range: bool,
}

impl Field {
    pub fn is_grass(&self) -> bool {
        self.scrap_amount == 0
    }

    pub fn from_input_line(input_line: &[&str], x: u32, y: u32) -> Self {
        let scrap_amount = parse_input!(input_line[0], i32);
        let owner = parse_input!(input_line[1], i32); // 1 = me, 0 = foe, -1 = neutral
        let units = parse_input!(input_line[2], i32);
        let recycler = parse_input!(input_line[3], i32);
        let can_build = parse_input!(input_line[4], i32);
        let can_spawn = parse_input!(input_line[5], i32);
        let in_range_of_recycler = parse_input!(input_line[6], i32);

        Self {
            x,
            y,
            scrap_amount: scrap_amount as u32,
            owner: Owner::from_num(owner),
            num_units: units as u32,
            has_recycler: recycler == 1,
            can_build: can_build == 1,
            can_spawn: can_spawn == 1,
            in_recycler_range: in_range_of_recycler == 1,
        }
    }
}
