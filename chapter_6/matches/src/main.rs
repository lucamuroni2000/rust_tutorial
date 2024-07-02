// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
//
// // fn value_in_cents(coin: Coin) -> u8 {
// //     match coin {
// //         Coin::Penny => 1,
// //         Coin::Nickel => 5,
// //         Coin::Dime => 10,
// //         Coin::Quarter => 25,
// //     }
// // }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
//
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

// // If you roll a 3 on a dice roll, your player doesn’t move, but instead gets a new fancy hat.
// // If you roll a 7, your player loses a fancy hat.
// // For all other values, your player moves that number of spaces on the game board.
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }
// }
//
// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}

// // If you roll anything other than a 3 or a 7, you must roll again.
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }
// }
//
// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn reroll() {}

// Nothing else happens on your turn if you roll anything other than a 3 or a 7
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}