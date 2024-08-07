// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s); // word will get the value 5
//
//     s.clear(); // this empties the String, making it equal to ""
//
//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }
//
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

// fn main() {
//     // let s = String::from("hello world");
//     //
//     // let hello = &s[0..5];
//     // let world = &s[6..11];
//
//     // let s = String::from("hello");
//     //
//     // let slice = &s[0..2];
//     // let slice = &s[..2];
//
//     // let s = String::from("hello");
//     //
//     // let len = s.len();
//     //
//     // let slice = &s[3..len];
//     // let slice = &s[3..];
//
//     let s = String::from("hello");
//
//     let len = s.len();
//
//     let slice = &s[0..len];
//     let slice = &s[..];
// }

// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s);
//
//     s.clear(); // error!
//
//     println!("the first word is: {word}");
// }
//
// fn first_word(s: &Str) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("the first word is: {word}");
    let word = first_word(&my_string[..]);
    println!("the second word is: {word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("the third word is: {word}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}