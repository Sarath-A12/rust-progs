fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];

    v[99];
    //RUN with RUST_BACKTRACE=1 cargo run
}
