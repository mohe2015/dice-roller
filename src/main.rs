

enum Direction {
    Up, Right, Down, Left
}

// x_position y_position top_dice_number front_dice_number
fn move_dice(pos_and_state: (u8, u8, u8, u8), direction: Direction) -> (u8, u8, u8, u8) {
     match direction {
        Direction::Up => (pos_and_state.0, pos_and_state.1-1, pos_and_state.3, match pos_and_state.3 {
           1 => 6,
           2 => 5,
           3 => 4,
           4 => 3,
           5 => 2,
           6 => 1,
           
           /* State::OneTwo => State::TwoSix,
            State::OneThree => State::ThreeSix,
            State::OneFive => State::FiveSix,
            State::OneFour => State::FourSix,

            State::TwoOne => State::OneFive,
            State::TwoFour => State::FourFive,
            State::TwoSix => State::SixFive,
            State::TwoThree => State::ThreeFive,

            State::ThreeOne => State::OneFour,
            State::ThreeTwo => State::TwoFour,
            State::ThreeSix => State::SixFour,
            State::ThreeFive => State::FiveFour,

            State::FourOne => State::OneThree,
            State::FourFive => State::FiveThree,
            State::FourSix => State::SixThree,
            State::FourTwo => State::TwoThree,

            State::FiveOne => State::OneTwo,
            State::FiveThree => State::ThreeTwo,
            State::FiveSix => State::SixTwo,
            State::FiveFour => State::FourTwo,

            State::SixTwo => State::TwoOne,
            State::SixFour => State::FourOne,
            State::SixFive => State::FiveOne,
            State::SixThree => State::ThreeOne,*/
        }),
        Direction::Right => (pos_and_state.0+1, pos_and_state.1, match pos_and_state.2 {
            State::OneTwo => todo!(),
            State::OneThree => todo!(),
            State::OneFive => todo!(),
            State::OneFour => todo!(),
            State::TwoOne => todo!(),
            State::TwoFour => todo!(),
            State::TwoSix => todo!(),
            State::TwoThree => todo!(),
            State::ThreeOne => todo!(),
            State::ThreeTwo => todo!(),
            State::ThreeSix => todo!(),
            State::ThreeFive => todo!(),
            State::FourOne => todo!(),
            State::FourFive => todo!(),
            State::FourSix => todo!(),
            State::FourTwo => todo!(),
            State::FiveOne => todo!(),
            State::FiveThree => todo!(),
            State::FiveSix => todo!(),
            State::FiveFour => todo!(),
            State::SixTwo => todo!(),
            State::SixFour => todo!(),
            State::SixFive => todo!(),
            State::SixThree => todo!(),
        }),
        Direction::Down => (pos_and_state.0, pos_and_state.1+1, match pos_and_state.2 {
            State::OneTwo => todo!(),
            State::OneThree => todo!(),
            State::OneFive => todo!(),
            State::OneFour => todo!(),
            State::TwoOne => todo!(),
            State::TwoFour => todo!(),
            State::TwoSix => todo!(),
            State::TwoThree => todo!(),
            State::ThreeOne => todo!(),
            State::ThreeTwo => todo!(),
            State::ThreeSix => todo!(),
            State::ThreeFive => todo!(),
            State::FourOne => todo!(),
            State::FourFive => todo!(),
            State::FourSix => todo!(),
            State::FourTwo => todo!(),
            State::FiveOne => todo!(),
            State::FiveThree => todo!(),
            State::FiveSix => todo!(),
            State::FiveFour => todo!(),
            State::SixTwo => todo!(),
            State::SixFour => todo!(),
            State::SixFive => todo!(),
            State::SixThree => todo!(),
        }),
        Direction::Left => (pos_and_state.0-1, pos_and_state.1,match pos_and_state.2 {
            State::OneTwo => todo!(),
            State::OneThree => todo!(),
            State::OneFive => todo!(),
            State::OneFour => todo!(),
            State::TwoOne => todo!(),
            State::TwoFour => todo!(),
            State::TwoSix => todo!(),
            State::TwoThree => todo!(),
            State::ThreeOne => todo!(),
            State::ThreeTwo => todo!(),
            State::ThreeSix => todo!(),
            State::ThreeFive => todo!(),
            State::FourOne => todo!(),
            State::FourFive => todo!(),
            State::FourSix => todo!(),
            State::FourTwo => todo!(),
            State::FiveOne => todo!(),
            State::FiveThree => todo!(),
            State::FiveSix => todo!(),
            State::FiveFour => todo!(),
            State::SixTwo => todo!(),
            State::SixFour => todo!(),
            State::SixFive => todo!(),
            State::SixThree => todo!(),
        }),
    }
}

fn main() {
    let pos_and_state: (u8, u8, u8, u8) = (0,0, 1, 2);


    println!("Hello, world!");
}
