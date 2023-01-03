use std::cmp::Ordering;


#[derive(Copy, Clone, Debug)]
pub enum Owner {
    Me,
    Opponent,
    Neutral,
}

impl Owner {
    pub fn from_num(num: i32) -> Self {
        match num {
            -1 => Owner::Neutral,
            0 => Owner::Opponent,
            1 => Owner::Me,
            _ => panic!("unknown owner num"),
        }
    }
}

impl Default for Owner {
    fn default() -> Self {
        Owner::Neutral
    }
}

impl Eq for Owner {}

impl PartialEq<Self> for Owner {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Owner::Me, Owner::Me) => true,
            (Owner::Neutral, Owner::Neutral) => true,
            (Owner::Opponent, Owner::Opponent) => true,
            _ => false,
        }
    }
}

impl PartialOrd<Self> for Owner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Owner {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Me, Self::Me) => Ordering::Equal,
            (Self::Me, _) => Ordering::Greater,
            (Self::Neutral, Self::Me) => Ordering::Less,
            (Self::Neutral, Self::Neutral) => Ordering::Equal,
            (Self::Neutral, _) => Ordering::Greater,
            (Self::Opponent, Self::Opponent) => Ordering::Equal,
            (Self::Opponent, _) => Ordering::Less,
        }
    }
}

