use crate::DieNumber::*;

// just store the full cube and then simulate the rotation

enum Direction {
    Up, Right, Down, Left
}

enum DieNumber {
    One, Two, Three, Four, Five, Six
}

// front back top bottom left right
struct Die([DieNumber; 6]);

// x_position y_position top_dice_number front_dice_number
fn move_dice(pos_and_state: (u8, u8, Die), direction: Direction) -> (u8, u8, Die) {
     
}

fn main() {
    let pos_and_state: (u8, u8, Die) = (0,0, Die([One, Six, Four, Three, Two, Five]));


    println!("Hello, world!");
}
