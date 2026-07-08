#![deny(clippy::pedantic, clippy::restriction, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    reason = "Selectively allowing features is preferred"
)]
#![allow(clippy::print_stdout, reason = "This is a CLI tool")]

fn main() {
    println!("Hello, World!");
}
