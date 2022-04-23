use std::{collections::{HashSet, BinaryHeap, HashMap}, cmp::Ordering};

use crate::DieNumber::*;

// just store the full cube and then simulate the rotation

#[derive(Debug)]
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
fn move_dice(x: u8, y: u8, die: Die, direction: Direction) -> (u8, u8, Die) {
    match direction {
        Direction::Up => (
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
        ),
        Direction::Right => (
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
        ),
        Direction::Down => (
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
        ),
        Direction::Left => (
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
        ),
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
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: (u8, u8, Die), goal: (u8, u8, Die)) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<(u8, u8, Die), usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if Ordering::Less == dist.get(&position).cmp(&Some(&cost)) { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < *dist.get(&next.position).unwrap_or(&usize::MAX) {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}

fn find_goal(already_tried: &mut HashSet<(u8, u8, Die)>, current: (u8, u8, Die), goal: (u8, u8, Die)) -> Option<Vec<Direction>> {
    if already_tried.contains(&current) {
        return None
    }
    if current == goal {
        return Some(vec![])
    }
    already_tried.insert(current);
    if current.0 > 0 {
        let target: (u8, u8, Die) = move_dice(current.0, current.1, current.2, Direction::Left);
        let path = find_goal(already_tried, target, goal);
        if let Some(mut path) = path {
            path.push(Direction::Left);
            return Some(path)
        }
    }
    if current.0 < 8 {
        let target: (u8, u8, Die) = move_dice(current.0, current.1, current.2, Direction::Right);
        let path = find_goal(already_tried, target, goal);
        if let Some(mut path) = path {
            path.push(Direction::Right);
            return Some(path)
        }
    }
    if current.1 > 0 {
        let target: (u8, u8, Die) = move_dice(current.0, current.1, current.2, Direction::Up);
        let path = find_goal(already_tried, target, goal);
        if let Some(mut path) = path {
            path.push(Direction::Up);
            return Some(path)
        }
    }
    if current.1 < 8 {
        let target: (u8, u8, Die) = move_dice(current.0, current.1, current.2, Direction::Down);
        let path = find_goal(already_tried, target, goal);
        if let Some(mut path) = path {
            path.push(Direction::Down);
            return Some(path)
        }
    }
    None
}

fn main() {
    let mut already_tried: HashSet<(u8, u8, Die)> = HashSet::new();

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
    println!("{:?}", find_goal(&mut already_tried, current, goal));
}
