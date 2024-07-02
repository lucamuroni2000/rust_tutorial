// fn main() {
//     panic!("crash and burn");
// }

fn main() {
    let v = vec![1, 2, 3];

    v[99];
} //$env:RUST_BACKTRACE=1; cargo run
