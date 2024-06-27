// use std::io;
//
// //Convert from fahrenheit to celsius
// fn main() {
//     loop {
//         println!("Enter a temperature in fahrenheit: ");
//
//         let mut fahrenheit = String::new();
//
//         io::stdin()
//             .read_line(&mut fahrenheit)
//             .expect("Failed to read line");
//
//         let fahrenheit = fahrenheit.trim();
//
//         let celsius = match fahrenheit.parse::<f32>() {
//             Ok(num) => (num - 32.0) / 1.8,
//             Err(_) => continue,
//         };
//
//         println!("{fahrenheit}°F is {celsius}°C");
//
//         println!("Continue? (y/n)");
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         if input.trim() != "y" {
//             break;
//         }
//     }
// }

// //Fibonacci sequence
// fn main() {
//     let mut f0 = 0;
//     let mut f1 = 1;
//
//     print!("{} {} ", f0, f1);
//     for _i in 2..10 {
//         let fn_1 = f1;
//         let fn_2 = f0 + f1;
//         print!("{} ", fn_2);
//         f0 = fn_1;
//         f1 = fn_2;
//     }
//     println!();
// }


// On the first day of Christmas
// My true love sent to me
// A partridge in a pear tree!

// Three french hens
// On the third day of Christmas
// My true love sent to me
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Christmas Songs - The Twelve Days of Christmas Lyrics
fn main() {
    let presents = ["partridge in a pear tree!", "turtle doves", "french hens", "calling birds",
    "golden rings!", "geese a layin'", "swans a swimmin'", "maids milkin'", "ladies dancin'", "lords a leapin'",
    "pipers pipin'", "drummers drummin'"];

    for cont in 1..presents.len() + 1 {
        if cont != 1 {
            println!("{cont} {}", presents[cont - 1]);
        }
        println!("On the {cont}° day of Christmas");
        println!("My true love sent to me");
        for _i in (1..cont+1).rev() {
            if cont == 1 {
                println!("A {}", presents[0]);
                println!();
            } else {
                if _i == 1{
                    println!("And a {}", presents[0]);
                    println!();
                } else {
                    println!("{_i} {}", presents[_i - 1]);
                }
            }
        }
    }
}