use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::DieNumber::*;

// just store the full cube and then simulate the rotation

#[derive(Debug, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum DieNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Die {
    front: DieNumber,
    back: DieNumber,
    top: DieNumber,
    bottom: DieNumber,
    left: DieNumber,
    right: DieNumber,
}

// x_position y_position top_dice_number front_dice_number
fn move_dice(x: u8, y: u8, die: Die, direction: Direction) -> Option<(u8, u8, Die)> {
    match direction {
        Direction::Up => {
            if y == 0 {
                None
            } else {
                Some((
                    x,
                    y - 1,
                    Die {
                        front: die.bottom,
                        back: die.top,
                        top: die.front,
                        bottom: die.back,
                        left: die.left,
                        right: die.right,
                    },
                ))
            }
        }
        Direction::Right => {
            if x == 7 {
                None
            } else {
                Some((
                    x + 1,
                    y,
                    Die {
                        front: die.left,
                        back: die.right,
                        top: die.top,
                        bottom: die.bottom,
                        left: die.back,
                        right: die.front,
                    },
                ))
            }
        }
        Direction::Down => {
            if y == 7 {
                None
            } else {
                Some((
                    x,
                    y + 1,
                    Die {
                        front: die.top,
                        back: die.bottom,
                        top: die.back,
                        bottom: die.front,
                        left: die.left,
                        right: die.right,
                    },
                ))
            }
        }
        Direction::Left => {
            if x == 0 {
                None
            } else {
                Some((
                    x - 1,
                    y,
                    Die {
                        front: die.right,
                        back: die.left,
                        top: die.top,
                        bottom: die.bottom,
                        left: die.front,
                        right: die.back,
                    },
                ))
            }
        }
    }
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (u8, u8, Die),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(start: (u8, u8, Die), goal: (u8, u8, Die)) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<(u8, u8, Die), usize> = HashMap::new();
    let mut prev: HashMap<(u8, u8, Die), Direction> = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            //prev.get(&position).
            // based on reversing the direction we probably would be able to backtarck the path here

            return Some(cost);

        }

        // Important as we may have already found a better way
        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            if let Some(node) = move_dice(position.0, position.1, position.2, edge) {
                let next = State {
                    cost: cost + 1,
                    position: node,
                };

                // If so, add it to the frontier and continue
                if next.cost < *dist.get(&next.position).unwrap_or(&usize::MAX) {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist.insert(next.position, next.cost);

                    prev.insert(next, edge);
                }
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    let current: (u8, u8, Die) = (
        0,
        0,
        Die {
            front: One,
            back: Six,
            top: Two,
            bottom: Five,
            left: Three,
            right: Four,
        },
    );
    let goal: (u8, u8, Die) = (
        0,
        1,
        Die {
            front: Two,
            back: Five,
            top: Six,
            bottom: One,
            left: Three,
            right: Four,
        },
    );
    println!("{:?}", shortest_path(current, goal));
}
