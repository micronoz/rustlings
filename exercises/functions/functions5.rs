// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

use std::convert::TryInto;

fn main() {
    let answer = square(32);
    println!("The answer is {}", answer);
}

fn square(num: i16) -> i16 {
    return (num * num).try_into().unwrap();
}
