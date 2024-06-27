fn main() {
    let x = 1; //i32
    let y: i64 = 2; //i64

    let x: u32 =1; //u32
    let y: u64 = 2; //u64

    println!("Max i32: {}", i32::MAX);
    println!("Max i64: {}", i64::MAX);
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
