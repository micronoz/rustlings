// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    call_me(5);
}

fn call_me(num: i32) {
    for i in 0..num - 1 {
        call_me(num - 1);
        println!("Ring! Call number {}", i + 1);
    }
}
