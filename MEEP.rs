use std::env;
fn main() {
    for _ in env::args() {
        println!("MEEP");
    }
}