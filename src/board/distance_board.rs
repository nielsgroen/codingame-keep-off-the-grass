use std::collections::{BinaryHeap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use crate::board::{Board, Owner};

#[derive(Clone)]
pub struct DistanceBoard {
    width: u32,
    height: u32,
    distances: Vec<ManhattanDistance>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ManhattanDistance {
    Dist(u32),
    Unreachable,
}

impl DistanceBoard {

    pub fn new(board: &Board, from_owner: Owner) -> Self {
        // let mut calculated: FieldHashSet<(u32, u32)> = Default::default();
        let mut distances = vec![ManhattanDistance::Unreachable; (board.width * board.height) as usize];

        // (dist, x, y)
        let mut frontier: BinaryHeap<(u32, u32, u32)> = BinaryHeap::new();

        // initialize search frontier
        for i in 0..board.width {
            for j in 0..board.height {
                let current_field = board.get_field(i, j).unwrap();

                match (current_field.owner, current_field.num_units) {
                    (owner, units) if owner == from_owner => {
                        distances[(i + j * board.width) as usize] = ManhattanDistance::Dist(1);

                        let adjacent_coords = adjacent_in_range(i, j, board.width, board.height)
                            .into_iter()
                            .flatten()
                            .filter(|x| !board.get_field(x.0, x.1).unwrap().is_grass())
                            .filter(|x| board.get_field(x.0, x.1).unwrap().owner != from_owner);

                        frontier.extend(
                            adjacent_coords.clone()
                                .map(|x| (if units == 0 { 2 } else { 1 }, x.0, x.1))
                        );

                        for adjacent in adjacent_coords {
                            distances[(adjacent.0 + adjacent.1 * board.width) as usize] = ManhattanDistance::Dist(if units == 0 { 2 } else { 1 });
                        }
                    },
                    _ => (),
                }
            }
        }

        // Keep on rolling the frontier
        while let Some((distance, x, y)) = frontier.pop() {
            let adjacent_coords = adjacent_in_range(x, y, board.width, board.height)
                .into_iter()
                .flatten()
                .filter(|x| !board.get_field(x.0, x.1).unwrap().is_grass())
                .filter(|x| distances[(x.0 + x.1 * board.width) as usize ] == ManhattanDistance::Unreachable);

            frontier.extend(
                adjacent_coords.clone()
                    .map(|x| (distance + 1, x.0, x.1))
            );

            let adjacent_coords: Vec<_> = adjacent_coords.collect(); // end the borrow on `distances`
            for adjacent in adjacent_coords.into_iter() {
                distances[(adjacent.0 + adjacent.1 * board.width) as usize] = ManhattanDistance::Dist(distance + 1);
            }

        }

        DistanceBoard {
            width: board.width,
            height: board.height,
            distances,
        }
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

// struct FnvHasher(u64);
//
// impl Default for FnvHasher {
//
//     #[inline]
//     fn default() -> FnvHasher {
//         FnvHasher(0xcbf29ce484222325)
//     }
// }
//
// impl Hasher for FnvHasher {
//     #[inline]
//     fn finish(&self) -> u64 {
//         self.0
//     }
//
//     #[inline]
//     fn write(&mut self, bytes: &[u8]) {
//         let FnvHasher(mut hash) = *self;
//
//         for byte in bytes.iter() {
//             hash = hash ^ (*byte as u64);
//             hash = hash.wrapping_mul(0x100000001b3);
//         }
//
//         *self = FnvHasher(hash);
//     }
// }
//
// type FieldHashBuilder = BuildHasherDefault<FnvHasher>;
//
// type FieldHashSet<T> = HashSet<T, FieldHashBuilder>;
