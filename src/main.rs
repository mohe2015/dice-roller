use std::collections::HashSet;

use crate::DieNumber::*;

// just store the full cube and then simulate the rotation

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum DieNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
