use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use super::{Board, Owner};

#[derive(Clone, Debug)]
pub struct DistanceBoard {
    width: u32,
    height: u32,
    distances: Vec<ManhattanDistance>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ManhattanDistance {
    Dist(u32),
    Unreachable,
}

impl DistanceBoard {

    pub fn from_owner(board: &Board, from_owner: Owner) -> Self {
        // let mut calculated: FieldHashSet<(u32, u32)> = Default::default();
        let mut distances = vec![ManhattanDistance::Unreachable; (board.width * board.height) as usize];

        // (dist, x, y)
        let mut frontier: BinaryHeap<(Reverse<u32>, u32, u32)> = BinaryHeap::new();

        // initialize search frontier
        // First only fields w/ robots
        for i in 0..board.width {
            for j in 0..board.height {
                let current_field = board.get_field(i, j).unwrap();

                match (current_field.owner, current_field.num_units) {
                    (_, 0) => (),
                    (owner, _) if owner == from_owner => {
                        distances[(i + j * board.width) as usize] = ManhattanDistance::Dist(0);

                        let adjacent_coords = adjacent_in_range(i, j, board.width, board.height);
                        let adjacent_coords = adjacent_coords
                            .into_iter()
                            .flatten()
                            .filter(|x| !board.get_field(x.0, x.1).unwrap().is_grass())
                            .filter(|x| board.get_field(x.0, x.1).unwrap().owner != from_owner)
                            .filter(|x| distances[(x.0 + x.1 * board.width) as usize ] == ManhattanDistance::Unreachable);

                        frontier.extend(
                            adjacent_coords.clone()
                                .map(|x| (Reverse(1), x.0, x.1))
                        );

                        let adjacent_coords: Vec<_> = adjacent_coords.collect(); // end the borrow on `distances`
                        for adjacent in adjacent_coords.into_iter() {
                            distances[(adjacent.0 + adjacent.1 * board.width) as usize] = ManhattanDistance::Dist(1);
                        }
                    },
                    _ => (),
                }
            }
        }

        // now initialize owned fields w/o robots
        for i in 0..board.width {
            for j in 0..board.height {
                let current_field = board.get_field(i, j).unwrap();

                match (current_field.owner, current_field.num_units) {
                    (owner, 0) if owner == from_owner => {
                        distances[(i + j * board.width) as usize] = ManhattanDistance::Dist(1);

                        let adjacent_coords = adjacent_in_range(i, j, board.width, board.height);
                        let adjacent_coords = adjacent_coords
                            .into_iter()
                            .flatten()
                            .filter(|x| !board.get_field(x.0, x.1).unwrap().is_grass())
                            .filter(|x| board.get_field(x.0, x.1).unwrap().owner != from_owner)
                            .filter(|x| distances[(x.0 + x.1 * board.width) as usize ] == ManhattanDistance::Unreachable);

                        frontier.extend(
                            adjacent_coords.clone()
                                .map(|x| (Reverse(2), x.0, x.1))
                        );

                        let adjacent_coords: Vec<_> = adjacent_coords.collect(); // end the borrow on `distances`
                        for adjacent in adjacent_coords.into_iter() {
                            distances[(adjacent.0 + adjacent.1 * board.width) as usize] = ManhattanDistance::Dist(2);
                        }
                    },
                    _ => (),
                }
            }
        }


        // Keep on rolling the frontier
        while let Some((distance, x, y)) = frontier.pop() {
            let adjacent_coords = adjacent_in_range(x, y, board.width, board.height);
            let adjacent_coords = adjacent_coords
                .into_iter()
                .flatten()
                .filter(|x| !board.get_field(x.0, x.1).unwrap().is_grass())
                .filter(|x| distances[(x.0 + x.1 * board.width) as usize ] == ManhattanDistance::Unreachable);

            frontier.extend(
                adjacent_coords.clone()
                    .map(|x| (Reverse(distance.0 + 1), x.0, x.1))
            );

            let adjacent_coords: Vec<_> = adjacent_coords.collect(); // end the borrow on `distances`
            for adjacent in adjacent_coords.into_iter() {
                distances[(adjacent.0 + adjacent.1 * board.width) as usize] = ManhattanDistance::Dist(distance.0 + 1);
            }

        }

        DistanceBoard {
            width: board.width,
            height: board.height,
            distances,
        }
    }

    /// Finds all the directions which are best for going up or down the distancefield
    pub fn towards(&self, x: u32, y: u32, ordering: Ordering) -> [bool; 4] {
        todo!()
        // note: exclude Unreachable
    }


}

fn adjacent_in_range(x: u32, y: u32, width: u32, height: u32) -> [Option<(u32, u32)>; 4] {
    // NESW
    [
        if 0 <= x && x < width && 0 <= y-1 && y-1 < height { Some((x, y-1)) } else { None },
        if 0 <= x+1 && x+1 < width && 0 <= y && y < height { Some((x+1, y)) } else { None },
        if 0 <= x && x < width && 0 <= y+1 && y+1 < height { Some((x, y+1)) } else { None },
        if 0 <= x-1 && x-1 < width && 0 <= y && y < height { Some((x-1, y)) } else { None },
    ]
}
