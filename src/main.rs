

enum Direction {
    Up, Right, Down, Left
}


// Dice: Top Front
enum State {
    OneTwo,
    OneThree,
    OneFive,
    OneFour,

    TwoOne,
    TwoFour,
    TwoSix,
    TwoThree,
    
    ThreeOne,
    ThreeTwo,
    ThreeSix,
    ThreeFive,

    FourOne,
    FourFive,
    FourSix,
    FourTwo,

    FiveOne,
    FiveThree,
    FiveSix,
    FiveFour,

    SixTwo,
    SixFour,
    SixFive,
    SixThree
}

fn move_dice(pos: (u8, u8), state: State, direction: Direction) {

}

fn main() {
    let pos: (u8, u8) = (0,0);
    let state: State = State::OneTwo;


    println!("Hello, world!");
}
