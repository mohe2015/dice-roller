use crate::DieNumber::*;

// just store the full cube and then simulate the rotation

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum DieNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

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
        Direction::Right => todo!(),
        Direction::Down => todo!(),
        Direction::Left => todo!(),
    }
}

fn main() {
    let pos_and_state: (u8, u8, Die) = (
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

    println!("Hello, world!");
}
