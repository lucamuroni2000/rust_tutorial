// use std::fs::File;
//
// fn main() {
//     let greeting_file_result = File::open("hello.txt");
// }

// use std::fs::File;
//
// fn main() {
//     let greeting_file_result = File::open("hello.txt");
//
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {error:?}"),
//     };
// }

// use std::fs::File;
// use std::io::ErrorKind;
//
// fn main() {
//     let greeting_file_result = File::open("hello.txt");
//
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {other_error:?}");
//             }
//         },
//     };
// }

// use std::fs::File;
//
// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap();
// }

// use std::fs::File;
//
// fn main() {
//     let greeting_file = File::open("hello.txt")
//         .expect("hello.txt should be included in this project");
// }

// use std::fs::File;
// use std::io::{self, Read};
//
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// use std::fs::File;
// use std::io::{self, Read};
//
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// use std::fs::File;
// use std::io::{self, Read};
//
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//
//     Ok(username)
// }

// use std::fs;
// use std::io;
//
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

// use std::fs::File;
//
// fn main() {
//     let greeting_file = File::open("hello.txt")?; //ERROR
// }

// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}