fn main() {
    // panic!("crash and burn");

    // Using a panic! backtrace
    let v = vec![1, 2, 3];

    v[99];
    // run with RUST_BACKTRACE=1 to get full backtrace error info
}
