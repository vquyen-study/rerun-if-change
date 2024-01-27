// src/main.rs

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", message());
    println!("{}", message());
    println!("{}", message());
    println!("{}", message());
}
